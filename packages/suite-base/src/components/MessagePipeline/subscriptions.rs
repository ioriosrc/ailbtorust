```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use serde_json::{Value as JsonValue, Serialize};

#[derive(Serialize)]
struct Immutable<T> {
    // Define your Immutable struct here
}

#[derive(Serialize)]
struct InternalSubscribePayload {
    fields: Option<Vec<String>>,
    sampling_request: Option<JsonValue>,
    sampling_authorized: Option<bool>,
}

fn make_subscription_memoizer() -> Box<dyn Fn(SubscribePayload) -> SubscribePayload> {
    let memoize = moize::memoize(
        |val: SubscribePayload| val,
        MoizeConfig { is_deep_equal: true, maxSize: usize::MAX },
    );
    Box::new(memoize)
}

fn merge_subscription(a: &InternalSubscribePayload, b: &InternalSubscribePayload) -> InternalSubscribePayload {
    let a_fields = a.fields.as_ref();
    let b_fields = b.fields.as_ref();

    if a_fields.is_none() || b_fields.is_none() {
        return *a;
    }

    let fields: Vec<String> = a_fields
        .iter()
        .cloned()
        .chain(b_fields.iter().cloned())
        .filter(|field| !field.trim().is_empty())
        .collect();

    let same_sampling_mode = a.sampling_request == b.sampling_request;

    let sampling_request = if same_sampling_mode {
        a.sampling_request.clone()
    } else {
        None
    };

    let sampling_authorized = if same_sampling_mode && (a.sampling_authorized.unwrap_or(false) || b.sampling_authorized.unwrap_or(false)) {
        true
    } else {
        None
    };

    InternalSubscribePayload {
        fields: Some(fields),
        sampling_request,
        sampling_authorized,
    }
}

fn denormalize_subscriptions(subscriptions: Vec<InternalSubscribePayload>) -> Vec<InternalSubscribePayload> {
    let mut grouped = HashMap::new();

    for subscription in subscriptions {
        if !grouped.contains_key(&subscription.topic) {
            grouped.insert(subscription.topic.clone(), vec![subscription]);
        } else {
            grouped.get_mut(&subscription.topic).unwrap().push(subscription);
        }
    }

    let full_subscriptions: Vec<InternalSubscribePayload> = grouped
        .iter()
        .filter(|(_, subs)| subs.len() == 1)
        .map(|(_, subs)| subs[0].clone())
        .collect();

    let partial_subscriptions: Vec<InternalSubscribePayload> = grouped
        .iter()
        .filter(|(_, subs)| subs.len() > 1)
        .map(|(_, subs)| {
            let fields: Vec<String> = subs.iter().map(|sub| sub.fields.clone()).flatten().collect();
            InternalSubscribePayload {
                fields: Some(fields),
                sampling_request: None,
                sampling_authorized: None,
            }
        })
        .collect();

    full_subscriptions
        .into_iter()
        .chain(partial_subscriptions.into_iter())
        .collect()
}

fn merge_subscriptions(subscriptions: Vec<InternalSubscribePayload>) -> Vec<InternalSubscribePayload> {
    let mut full_subscriptions = subscriptions.clone();
    let partial_subscriptions = subscriptions.clone();

    for subscription in &full_subscriptions {
        if subscription.preload_type == "full" {
            // This implies a "partial" subscription to those fields
            full_subscriptions.push(InternalSubscribePayload {
                preload_type: "partial",
                ..subscription.clone()
            });
        }
    }

    let mut result = Vec::new();

    for subscription in &full_subscriptions {
        if let Some(same_subscription) = merge_subscription(subscription, &partial_subscriptions[0]) {
            result.push(same_subscription);
        }
    }

    result
}
```

This Rust code provides the functionality of merging and normalizing subscriptions based on the provided TypeScript/React code. The `mergeSubscription` function handles combining fields and authorization, while the `denormalizeSubscriptions` function groups and merges individual topic subscriptions. The `mergeSubscriptions` function ensures that both full and partial subscriptions are correctly merged.