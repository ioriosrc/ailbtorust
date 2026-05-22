```rust
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use ruma::events::{self, types};
use ruma::user_id::UserId;

pub struct Subscription {
    pub topic: String,
    pub preload_type: SubscribeType,
}

#[derive(Clone)]
pub enum SubscribeType {
    Full,
    Partial,
}

impl From<&Subscription> for SubscribePayload {
    fn from(subscription: &Subscription) -> Self {
        Self {
            topic: subscription.topic.clone(),
            fields: Some(subscription.preload_type.to_string()),
            force_subscribe: false,
        }
    }
}

fn get_preload_types(
    subscriptions: Vec<Subscription>,
) -> HashMap<String, SubscribePayload> {
    let mut result = HashMap::new();

    for subscription in &subscriptions {
        if !result.contains_key(&subscription.topic) {
            let preload_type = match subscription.preload_type {
                SubscribeType::Full => "full",
                SubscribeType::Partial => "partial",
            };

            result.insert(subscription.topic.clone(), SubscribePayload {
                topic: subscription.topic,
                fields: Some(preload_type.to_string()),
                force_subscribe: false,
            });
        }
    }

    result
}

fn remap_virtual_subscriptions(
    subscriptions: Vec<Subscription>,
    inputs_by_output_topic: HashMap<String, HashSet<&UserId>>,
) -> Vec<SubscribePayload> {
    let mut payload_inputs_pairs = vec![];

    for subscription in &subscriptions {
        if !inputs_by_output_topic.contains_key(&subscription.topic) {
            continue;
        }

        payload_inputs_pairs.push((subscription.clone(), inputs_by_output_topic[&subscription.topic]));
    }

    let payload_inputs_pairs: Vec<(Subscription, HashSet<&UserId>)> = payload_inputs_pairs
        .into_iter()
        .filter(|(_, topics)| !topics.is_empty())
        .collect();

    let mut unique_subscriptions = vec![];

    for (subscription, topics) in payload_inputs_pairs {
        let preload_type = subscription.preload_type.clone();

        let fields: HashSet<String> = topics.iter().map(|user_id| {
            let user_event = events::Event::<types::RoomMessage>::new(event!({
                sender: Some(UserId::from_str("user1").unwrap()),
                content: ruma::events::Content::Text { body: "Hello!".to_string() }.into(),
            }));

            fields_from_message(user_event)
        });

        unique_subscriptions.push(Subscription {
            topic: subscription.topic,
            preload_type,
            fields: Some(fields.into_iter().collect::<Vec<String>>().join(", ").to_string()),
        });
    }

    merge_unique_subscriptions(unique_subscriptions)
}

fn fields_from_message(event: &events::Event<types::RoomMessage>) -> HashSet<&str> {
    event.content.as_ref()
        .and_then(|content| content.as_json())
        .map(|json| serde_json::from_str::<HashMap<String, serde_json::Value>>(json).unwrap())
        .unwrap_or_default()
        .keys()
        .cloned()
        .collect()
}

fn merge_unique_subscriptions(subscriptions: Vec<Subscription>) -> Vec<SubscribePayload> {
    let mut result = HashMap::new();

    for subscription in subscriptions {
        if !result.contains_key(&subscription.topic) {
            result.insert(subscription.topic, subscription);
        } else {
            // If the subscription already exists, update it with the new fields
            result.entry(subscription.topic)
                .and_modify(|existing_subscription| existing_subscription.fields = Some(
                    format!(
                        "{}{}",
                        existing_subscription.fields.as_ref().unwrap_or_default(),
                        &subscription.fields.clone().unwrap_or_default()
                    ),
                ));
        }
    }

    result.values().cloned().collect()
}
```