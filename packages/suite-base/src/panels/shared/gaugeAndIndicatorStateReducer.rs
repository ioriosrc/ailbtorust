```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

type MessageEvent = HashMap<String, String>;

struct GaugeAndIndicatorState {
    latest_message: Option<MessageEvent>,
    latest_matching_queried_data: Option<HashMap<String, String>>,
    error: Option<Error>,
    parsed_path: Option<HashMap<String, String>>,
    path_parse_error: Option<Error>,
    global_variables: Option<HashMap<String, String>>,
}

fn get_single_data_item(results: &Vec<HashMap<String, String>>) -> Option<HashMap<String, String>> {
    if results.len() <= 1 {
        return Some(results[0]);
    }
    None
}

fn handle_frame_action_state(state: GaugeAndIndicatorState) -> GaugeAndIndicatorState {
    if let Some(path_parse_error) = state.path_parse_error {
        return { ...state, latest_message: None, error: Some(path_parse_error), parsed_path: None };
    }

    if !state.parsed_path.is_some() {
        return { ...state, error: Some("Message path not parsed".into()) };
    }
    
    let filled_in_path = fill_in_global_variables_in_path(state.parsed_path.as_ref().unwrap(), state.global_variables.as_ref().unwrap());
    let mut latest_matching_queried_data = None;
    let mut latest_message = None;

    for message in &state.messages {
        if message.get(&filled_in_path.topic_name).is_some() {
            let data = get_single_data_item(&simple_get_message_path_data_items(message, filled_in_path.as_ref().unwrap()));
            if let Some(data) = data {
                latest_matching_queried_data = Some(data);
                latest_message = Some(message.clone());
            }
        }
    }

    return { ...state, latest_message, latest_matching_queried_data };
}

fn handle_path_action_state_with_global_vars(state: GaugeAndIndicatorState) -> GaugeAndIndicatorState {
    let path_result = parse_message_path(&state.path);
    let mut latest_matching_queried_data: Option<HashMap<String, String>> = None;
    let error: Option<Error> = None;

    if let Ok(path) = path_result {
        let filled_in_path = fill_in_global_variables_in_path(&path, state.global_variables.as_ref().unwrap());
        match &state.latest_message {
            Some(message) => {
                latest_matching_queried_data = get_single_data_item(&simple_get_message_path_data_items(message, filled_in_path.as_ref().unwrap()));
            }
            None => {}
        }
    } else {
        error = Some(path_result.err().unwrap());
    }

    return {
        ...state,
        error,
        latest_matching_queried_data,
        parsed_path: Some(path),
        path_parse_error: None,
    };
}

fn stateReducer(state: GaugeAndIndicatorState, action: String) -> GaugeAndIndicatorState {
    match action.as_str() {
        "frame" => handle_frame_action_state(state),
        "path" => handle_path_action_state_with_global_vars(state),
        "seek" => {
            let mut new_state = state;
            new_state.latest_message = None;
            new_state.latest_matching_queried_data = None;
            new_state.error = None;
            new_state
        }
        "updateGlobalVariables" => {
            let mut new_state = state;
            new_state.global_variables = Some(action.as_ref().parse::<HashMap<String, String>>().unwrap());
            new_state
        }
        _ => state,
    }
}
```