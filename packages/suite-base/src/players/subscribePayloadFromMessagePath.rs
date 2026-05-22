```rust
use std::collections::HashSet;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use message_path::{parse_message_path, MessagePathPart};

use crate::types::*;

/**
 * Builds a SubscribePayload from a message path, requesting a specific field of the message if the
 * message path resolves to a field name.
 */
pub fn subscribe_payload_from_message_path(
  path: &str,
  preload_type: Option<SubscriptionPreloadType>,
  sampling_request: Option<SubscribePayload>,
) -> Option<SubscribePayload> {
    let parsed_path = parse_message_path(path);

    if !parsed_path.is_ok() {
        return None;
    }

    type NamePart = MessagePathPart & { type: "name" };

    let first_field = parsed_path.iter().find(|element| element.type_ == "name");

    if first_field.is_none() {
        let payload: SubscribePayload = {
            topic: parsed_path.unwrap(),
            preload_type: preload_type.unwrap_or(SubscriptionPreloadType::Partial),
        };
        if let Some(sampling_request) = sampling_request {
            payload.sampling_request = sampling_request;
        }
        return Some(payload);
    }

    let fields: HashSet<&str> = first_field.map(|field| field.name.as_str()).collect();
    let payload: SubscribePayload = {
        topic: parsed_path.unwrap(),
        preload_type: preload_type.unwrap_or(SubscriptionPreloadType::Partial),
        fields,
    };
    if let Some(sampling_request) = sampling_request {
        payload.sampling_request = sampling_request;
    }
    Some(payload)
}
```

This Rust code snippet takes a message path, parses it, and constructs a `SubscribePayload` based on the parsed data. It checks if the message path resolves to a field name and, if so, includes that field in the `SubscribePayload`. The method returns `None` if the parsing fails or if the path does not contain a field.