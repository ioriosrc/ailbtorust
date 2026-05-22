```rust
use std::convert::{TryFrom, TryInto};
use std::vec::Vec;

// Assuming PieChartState and PieChartAction are defined elsewhere

fn handle_frame(state: PieChartState, action: Extract<PieChartAction, { type: "frame" }>) -> PieChartState {
    if state.path_parse_error.is_some() {
        return {
            state,
            latest_message: Some(action.messages.last().unwrap()),
            error: None,
        };
    }

    let mut latest_matching_queried_data = state.latest_matching_queried_data;
    let latest_message = state.latest_message;
    let error = state.error;

    if let Some(parsed_path) = &state.parsed_path {
        for message in action.messages {
            if message.topic != parsed_path.topic_name {
                continue;
            }

            match simple_get_message_path_data_items(message, parsed_path) {
                Ok(extracted_data) => {
                    let data: Vec<f32> = extracted_data.into_iter().map(|item| item as f32).collect();

                    latest_matching_queried_data = Some(data);
                    latest_message = message;
                    error = None;
                },
                Err(err) => {
                    error = Some(err);
                }
            };
        }
    }

    {
        let state: &mut PieChartState = &mut state;
        *state = state
            .clone()
            .map(|s| {
                s.latest_message = latest_message.clone();
                s.latest_matching_queried_data = latest_matching_queried_data.clone();
                s.error = error.clone();
                s
            })
            .unwrap_or(state);
    }

    return state;
}
```