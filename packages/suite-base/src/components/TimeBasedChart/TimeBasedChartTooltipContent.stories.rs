```rust
use react::forward_ref;
use react::props::ForwardRefProps;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2018-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use mui::material::{Tooltip, TooltipProps};
use mui::material::TooltipContentProps;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct TimeBasedChartTooltipData {
    config_index: usize,
    value: f64,
    constant_name: String,
}

// Function to convert a single data point to JSON
fn json_data_point(data: &TimeBasedChartTooltipData) -> Value {
    serde_json::json!({
        "configIndex": data.config_index,
        "value": data.value,
        "constantName": data.constant_name,
    })
}

#[forward_ref]
pub fn TimeBasedChartTooltipContent(props: ForwardRefProps<TimeBasedChartTooltipContentProps>) -> Html {
    let TooltipContent = props.children;

    html! {
        <Tooltip
            open={props.open}
            title={<TooltipContent />}
            placement="top"
            arrow
            slotProps={{
                popper: {
                    anchorEl: {
                        getBoundingClientRect: || {
                            return Box::new(Rect {
                                x: 200.0,
                                y: 100.0,
                                width: 0.0,
                                height: 0.0,
                            });
                        },
                    },
                },
            }}
        >
            <div style={{ width: "100%", height: "100%" }} />
        </Tooltip>
    }
}

#[forward_ref]
pub fn TimeBasedChartTooltipContentMultiDataset(props: ForwardRefProps<TimeBasedChartTooltipContentProps>) -> Html {
    let TooltipContent = props.children;
    let labels_by_config_index = props.labels_by_config_index;

    html! {
        <Tooltip
            open={props.open}
            title={
                <TooltipContent />
            }
            placement="top"
            arrow
            slotProps={{
                popper: {
                    anchorEl: {
                        getBoundingClientRect: || {
                            return Box::new(Rect {
                                x: 200.0,
                                y: 100.0,
                                width: 0.0,
                                height: 0.0,
                            });
                        },
                    },
                },
            }}
        >
            <div style={{ width: "100%", height: "100%" }} />
        </Tooltip>
    }
}

// Additional stories can be added similarly for multiple datasets and labels
```