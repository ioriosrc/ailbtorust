```rust
use std::error::Error;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// SPDX-FileCopyrightText: Copyright (C) 2025 Takayuki Honda <takayuki.honda@tier4.jp>
// SPDX-License-Identifier: MPL-2.0

use crate::types::{PieChartState, PieChartAction};

pub type HandlePathProps = {
    state: PieChartState;
    action: Extract<PieChartAction, { type: "path" }>;
};

fn handle_path(props: &HandlePathProps) -> PieChartState {
    let mut new_path;
    let path_parse_error: Option<String> = None;
    let error: Option<Box<dyn Error>> = None;

    if let Ok(new_path) = parse_message_path(&props.action.path) {
        if (new_path.message_path.iter()
            .any(|part| match part {
                &MessagePathPart::Filter { ref value } => is_variable(value),
                &MessagePathPart::Slice { .. } => true,
            }))
        {
            path_parse_error = Some("Message paths using variables are not currently supported");
        }
    } else {
        error = Some(Box::new(new_path));
    }

    let latest_matching_queried_data: Option<serde_json::Value> = if new_path.is_some() && path_parse_error.is_none()
    {
        if let Ok(data) = simple_get_message_path_data_items(&props.state.latest_message, &new_path) {
            Some(data)
        } else {
            error = Some(Box::new(simple_get_message_path_data_items_error));
        }
    } else {
        None
    };

    let new_state = props.state.clone();
    new_state.path = props.action.path.to_string();
    new_state.parsed_path = new_path;
    new_state.latest_matching_queried_data = latest_matching_queried_data;
    new_state.error = error.map(Box::new);
    new_state.path_parse_error = path_parse_error;
    new_state
}
```