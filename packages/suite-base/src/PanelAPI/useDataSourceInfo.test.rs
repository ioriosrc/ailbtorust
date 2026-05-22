```rust
use jest::prelude::*;
use wasm_bindgen_test::*;

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

mod utils;

use crate::utils::render_hook_with;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_use_data_source_info() {
    let topics = vec![Topic {
        name: "/foo".to_string(),
        schema_name: "Foo".to_string(),
    }];
    let messages = vec![
        MessageEvent {
            topic: "/foo".to_string(),
            receive_time: Time::from_nanos(1, 2),
            message: std::collections::HashMap::new(),
            schema_name: "foo".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/foo".to_string(),
            receive_time: Time::from_nanos(5, 6),
            message: std::collections::HashMap::new(),
            schema_name: "foo".to_string(),
            size_in_bytes: 0,
        },
    ];
    let datatypes = Map::new();
    let capabilities = vec!["hello"];
    let startTime = Time::from_nanos(0, 1);

    let wrapper = move |children| {
        MockMessagePipelineProvider {
            topics,
            datatypes,
            capabilities,
            messages: vec![messages[0].clone()],
            start_time: startTime.clone(),
        }
        .render_with(children)
    };

    let { result } = render_hook_with(wrapper, |_| PanelAPI::use_data_source_info());

    expect(result.value).to_eq(
        DataSourceInfo {
            topics: vec![
                Topic {
                    name: "/foo".to_string(),
                    schema_name: "Foo".to_string(),
                },
            ],
            services: vec![],
            datatypes,
            capabilities,
            start_time,
            player_id: "1",
        }
    );

    let first_result = result.value;

    messages[0].message.insert("key", "value");

    let rerendered_result = render_hook_with(wrapper, |_| PanelAPI::use_data_source_info());

    expect(rerendered_result.value).to_eq(first_result);

    topics.push(Topic {
        name: "/bar".to_string(),
        schema_name: "Bar".to_string(),
    });

    let rerendered_result = render_hook_with(wrapper, |_| PanelAPI::use_data_source_info());

    expect(rerendered_result.value).to_eq(DataSourceInfo {
        topics: vec![
            Topic {
                name: "/bar".to_string(),
                schema_name: "Bar".to_string(),
            },
            Topic {
                name: "/foo".to_string(),
                schema_name: "Foo".to_string(),
            },
        ],
        services: vec![],
        datatypes,
        capabilities,
        start_time,
        player_id: "1",
    });
}
```