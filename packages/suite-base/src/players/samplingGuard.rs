```rust
use std::fmt;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

type SamplingGuardInput = {
    topic: String,
    sampling_request: Option<String>,
    sampling_authorized: bool,
};

/**
 * Shared sampling authorization guard.
 *
 * Sampling requests are honored only when the subscription also carries
 * internal sampling authorization from trusted pipeline code.
 */
fn apply_sampling_guard_to_subscription<T>(subscription: T) -> T {
    if let Some(sampling_request) = &subscription.sampling_request {
        if sampling_request != "latest-per-render-tick" || !subscription.sampling_authorized {
            return Subscription::new(subscription.topic, None, false);
        }
    }

    subscription
}

/**
 * Applies the shared sampling guard to an array of subscriptions.
 */
fn apply_sampling_guard_to_subscriptions<T>(subscriptions: Vec<T>) -> Vec<T> {
    subscriptions.into_iter().map(apply_sampling_guard_to_subscription).collect()
}
```