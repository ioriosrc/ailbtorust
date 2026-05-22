```rust
use crate::gauge_and_indicator_state_reducer::{stateReducer, get_single_data_item};
use crate::types::{
    GaugeAndIndicatorAction, FrameAction, PathAction, SeekAction, MessagePath, MessagePathPart,
};

mod gauge_and_indicator_state_reducer {
    use super::*;

    pub fn stateReducer(state: &mut GaugeAndIndicatorState, action: &GaugeAndIndicatorAction) {
        match action {
            GaugeAndIndicatorAction::Frame(messages) => {
                let parsed_path = parse_message_path(action.path);
                if parsed_path.is_empty() || parsed_path.topic_name_repr.is_empty() {
                    state.error = Some("Topic name not found in message path");
                } else {
                    let latest_matching_queried_data = simple_get_message_path_data_items(parsed_path, messages);
                    state.latestMatchingQueriedData = latest_matching_queried_data.first().cloned();
                }
            },
            GaugeAndIndicatorAction::Path(path) => {
                let parsed_path = parse_message_path(action.path);
                if parsed_path.is_empty() || parsed_path.topic_name_repr.is_empty() {
                    state.error = Some("Topic name not found in message path");
                } else {
                    state.parsedPath = parsed_path;
                    state.latestMatchingQueriedData = simple_get_message_path_data_items(parsed_path, &state.latestMessage);
                }
            },
            GaugeAndIndicatorAction::Seek => {
                state.latestMatchingQueriedData = None;
                state.latestMessage = None;
            },
            GaugeAndIndicatorAction::UpdateGlobalVariables(global_variables) => {
                state.globalVariables = global_variables.clone();
            },
        }
    }

    pub fn get_single_data_item(items: &[MessageEvent]) -> Option<&MessageEvent> {
        if items.len() == 1 {
            Some(&items[0])
        } else if items.is_empty() {
            None
        } else {
            panic!("Message path produced multiple results");
        }
    }

    fn parse_message_path(path: &str) -> MessagePath {
        let mut message_path = Vec::new();
        for part in path.split('.') {
            let value = part.parse::<f64>().unwrap(); // Assuming numbers are represented as floats
            message_path.push(MessagePathPart {
                type_: "filter".to_string(),
                value: value,
            });
        }
        MessagePath {
            message_path,
            topic_name: path.to_string(),
            topic_name_repr: path.to_string(),
        }
    }

    fn simple_get_message_path_data_items(parsed_path: MessagePath, messages: &[MessageEvent]) -> Vec<MessageEvent> {
        let mut matching_events = Vec::new();
        for message in messages {
            if parsed_path.is_match(message) {
                matching_events.push(message.clone());
            }
        }
        matching_events
    }
}

mod types {
    pub enum GaugeAndIndicatorAction {
        Frame(Vec<MessageEvent>),
        Path(String),
        Seek,
        UpdateGlobalVariables(GlobalVariableBuilder::global_variables()),
    }

    #[derive(Default)]
    pub struct GlobalVariableBuilder;

    impl GlobalVariableBuilder {
        pub fn global_variables() -> GlobalVariableBuilder {
            Self
        }
    }

    pub enum MessageEvent {
        // Define the MessageEvent structure if needed
    }

    pub enum MessagePathPart {
        Filter { value: f64 },
    }

    pub struct MessagePath {
        message_path: Vec<MessagePathPart>,
        topic_name: String,
        topic_name_repr: String,
    }
}
```