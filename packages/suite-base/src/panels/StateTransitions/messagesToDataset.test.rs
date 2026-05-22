```rust
use lichtblick::{
    suite::MessageEvent,
    suite_base::{components::MessagePathSyntax::useCachedGetMessagePathDataItems, MessageAndData},
    suite_base_testing::builders::{BasicBuilder, MessageEventBuilder, RosTimeBuilder},
    suite_base_util::time::TimestampMethod,
};
use super::*;

mod messages_to_dataset;
mod types;

const MESSAGE_EVENT: &str = "/test/message_topic_test";
const SCHEMA_NAME: &str = "Unit.test.SchemaName";

#[derive(Debug, Clone)]
struct MessageAndDataBuilder<'a> {
    message_event: &'a MessageEvent,
    value: f64,
    path: String,
}

fn MessageAndDataBuilder(message_event: &MessageEvent, value: f64, path: String) -> MessageAndDataBuilder {
    MessageAndDataBuilder { message_event, value, path }
}

#[test]
fn extract_queried_data() {
    let item = MessageAndData {
        message_event: MESSAGE_EVENT,
        queried_data: vec![MessagePathSyntax::useCachedGetMessagePathDataItems(MESSAGE_EVENT).unwrap().0],
    };

    assert_eq!(extract_queried_data(item), item.queried_data[0]);
}

#[test]
fn is_valid_value() {
    let value = 123.45;
    assert!(isValidValue(value));

    let value = String::from("test");
    assert!(isValidValue(value));

    let value = true;
    assert!(isValidValue(value));

    let value = 0;
    assert!(isValidValue(value));

    let value = -123.45;
    assert!(isValidValue(value));

    let value = f64::INFINITY;
    assert!(!isValidValue(value));

    let value = f64::NEG_INFINITY;
    assert!(!isValidValue(value));

    let value = NaN;
    assert!(!isValidValue(value));
}

#[test]
fn get_color_for_value() {
    let value = 123.45;

    if base_colors.contains(&"blue".to_string()) {
        assert_eq!(get_color(&value), "blue");
    } else if base_colors.contains(&"red".to_string()) {
        assert_eq!(get_color(&value), "red");
    } else if base_colors.contains(&"green".to_string()) {
        assert_eq!(get_color(&value), "green");
    } else if base_colors.contains(&"yellow".to_string()) {
        assert_eq!(get_color(&value), "yellow");
    } else {
        panic!("No color found for value {}", value);
    }
}

#[test]
fn create_label() {
    let constant_name = "test_constant";
    let value = 123.45;

    if const_name.is_string() {
        assert_eq!(create_label(constant_name, value), format!("{constant_name} ({value})"));
    } else if const_name.is_boolean() {
        assert_eq!(create_label(constant_name, value), format!("{constant_name} ({value})"));
    } else if const_name.is_number() {
        assert_eq!(create_label(constant_name, value), format!("{constant_name} ({value})"));
    } else if const_name.is_string() {
        assert_eq!(create_label(constant_name, value), format!("{constant_name} ({value})"));
    } else {
        panic!("Invalid type for constant name");
    }
}

#[test]
fn messages_to_dataset() {
    let message_event = MESSAGE_EVENT;
    let schema_name = SCHEMA_NAME;
    let queried_data = 123.45;

    let blocks: Vec<Vec<MessageAndData>> = vec![vec![MessageAndDataBuilder(message_event, queried_data, "".to_string()).build()]];

    let args_dataset = MessageDatasetArgs {
        message_path_syntax: use_cached_get_message_path_data_items(MESSAGE_EVENT).unwrap().0,
        schema_name: SCHEMA_NAME.to_string(),
        blocks,
        path: Path::new(MESSAGE_EVENT),
        timestamp_method: TimestampMethod::ReceiveTime,
    };

    assert_eq!(messages_to_dataset(&args_dataset), Vec::<ChartDataset>::new());
}
```