```rust
use crate::types::*;

fn handle_frame(state: PieChartState, action: &PieChartAction) -> PieChartState {
    match action.type.as_str() {
        "frame" => {
            if let Some(path_parse_error) = state.parsed_path.path_parse_error.as_ref() {
                return state;
            }

            let mut latest_matching_queried_data = None;

            for msg in &action.messages[..] {
                if let Some(ref topic) = msg.topic.as_str() {
                    if topic == &state.parsed_path.topic_name {
                        latest_matching_queried_data = Some(msg.queries.get(0).unwrap().value.clone());
                        break;
                    }
                }
            }

            PieChartState {
                ..state
            }
        }
        _ => state,
    }
}
```