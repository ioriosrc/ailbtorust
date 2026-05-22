```rust
use std::sync::{Arc, Mutex};
use parking_lot::MutexGuard;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2019-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use crate::{
    MessageAndData,
    use_cached_get_message_path_data_items,
};

/**
 * Return an array of MessageAndData[] for matching messages on @param path.
 *
 * The first array item is the oldest matched message, and the last item is the newest.
 *
 * The `historySize` option configures how many matching messages to keep. The default is 1.
 */
pub fn use_message_data_item(path: &str, options: Option<Options>) -> Arc<Mutex<Vec<MessageAndData>>>> {
    let (history_size = options.unwrap_or_default().history_size), sampling_request = options.unwrap_or_default().sampling_request;

    let topics: Vec<SubscribePayload> = Arc::new(Mutex::new(vec![
        subscribe_payload_from_message_path(path, "partial", &sampling_request),
    ]));

    let cached_get_message_path_data_items = use_cached_get_message_path_data_items(vec![path]);

    let add_messages = Arc::new(Mutex::new(move |prev_value: Arc<Mutex<Vec<MessageAndData>>>>| {
        if prev_value.lock().is_empty() {
            return prev_value.clone();
        }

        let new_matches: Vec<MessageAndData> = prev_value
            .lock()
            .iter()
            .rev()
            .filter_map(|message_event| {
                let queried_data = cached_get_message_path_data_items(path, *message_event);
                if !queried_data.is_empty() {
                    Some(MessageAndData { message_event, queried_data })
                } else {
                    None
                }
            })
            .collect();

        if new_matches.len() == history_size {
            Arc::new(Mutex::new(new_matches))
        } else {
            let prev_matches = prev_value.lock().clone();
            Arc::new(Mutex::new(
                prev_matches.into_iter()
                    .rev()
                    .take(history_size)
                    .collect::<Vec<_>>(),
            ))
        }
    }));

    let restore = Arc::new(Mutex::new(move |prev_value: Option<Arc<Mutex<Vec<MessageAndData>>>>| {
        if prev_value.is_none() || path != prev_value.lock().unwrap().lock().unwrap().path() {
            return Arc::new(Mutex::new(vec![]));
        }

        let new_matches = prev_value
            .as_ref()
            .map(|prev_value| {
                prev_value.lock().unwrap().iter()
                    .rev()
                    .filter_map(|message_event| {
                        let queried_data = cached_get_message_path_data_items(path, *message_event);
                        if !queried_data.is_empty() {
                            Some(MessageAndData { message_event, queried_data })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if new_matches.len() == history_size {
            Arc::new(Mutex::new(new_matches))
        } else {
            let prev_matches = prev_value.unwrap().lock().unwrap();
            Arc::new(Mutex::new(
                prev_matches
                    .into_iter()
                    .rev()
                    .take(history_size)
                    .collect::<Vec<_>>(),
            ))
        }
    }));

    use_message_reducer(&topics, add_messages, restore);

    Arc::new(Mutex::new(vec![]))
}
```