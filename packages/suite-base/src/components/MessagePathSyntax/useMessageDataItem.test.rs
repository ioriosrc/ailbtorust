```rust
use anyhow::Error;
use crossbeam_channel::{bounded, Receiver, Sender};
use futures::{
  stream::StreamExt,
  FutureExt,
};
use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct MessageEvent {
  topic: String,
  receive_time: std::time::Instant,
  message: serde_json::Value,
  schema_name: String,
  size_in_bytes: usize,
}

fn main() -> Result<(), Error> {
    let (sender, receiver) = bounded(10);

    // Simulate receiving messages
    futures::stream! {
        for _ in 0..3 {
            let message_event = MessageEvent {
                topic: "/topic".to_string(),
                receive_time: std::time::Instant.now(),
                message: serde_json::json!({ "value": 0 }),
                schema_name: "datatype".to_string(),
                size_in_bytes: 0,
            };
            sender.send(message_event).await?;
        }
    };

    // Simulate updating the current layout
    let _current_layout = RwLock::new(Some("some_current_layout"));

    // Create a mock message pipeline provider
    struct MockMessagePipelineProvider {
        messages: Vec<MessageEvent>,
    }

    impl MockMessagePipelineProvider {
        fn new(messages: Vec<MessageEvent>) -> Self {
            Self { messages }
        }

        async fn get_messages(&self) -> Vec<MessageEvent> {
            futures::stream! {
                for message_event in &self.messages {
                    yield message_event.clone();
                }
            }.collect()
        }
    }

    // Create a mock current layout provider
    struct MockCurrentLayoutProvider {}

    impl MockCurrentLayoutProvider {
        async fn get_layout(&self) -> String {
            "some_current_layout".to_string()
        }
    }

    // Create a mock ros datatypes
    struct MockRosDatatypes {
        definitions: HashMap<String, serde_json::Value>,
    }

    impl MockRosDatatypes {
        fn new() -> Self {
            Self {
                definitions: HashMap::new(),
            }
        }

        async fn get_definition(&self, name: &str) -> Result<serde_json::Value, Error> {
            Ok(self.definitions.get(name)?.cloned().unwrap_or_default())
        }
    }

    // Create a basic builder
    struct BasicBuilder {}

    impl BasicBuilder {
        fn string() -> Self {
            Self {}
        }
    }

    type Options = HashMap<String, serde_json::Value>;

    // Create the use message data item hook
    fn use_message_data_item(path: &str) -> Vec<queried_data::QueriedData> {
        let (tx, rx) = bounded(10);

        futures::stream! {
            for message_event in receiver {
                if path eq message_event.topic {
                    tx.send(queried_data::QueriedData {
                        message_event: Some(message_event),
                        queried_data: vec![queried_data::QueriedDataType {
                            path,
                            value: serde_json::from_value(&message_event.message[&field]).unwrap_or_default(),
                        }],
                    }).await?;
                }
            }
        }.collect::<Vec<queried_data::QueriedData>>()
    }

    // Create the queried data struct
    struct QueryedDataType {
        path: String,
        value: serde_json::Value,
    }

    type QueriedData = Vec<QueryedDataType>;

    let topic = "/topic";
    let field = BasicBuilder.string();
    let path = format!("{}.{}", topic, field);

    // Test cases
    let mut messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    let mut queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 0,
        }],
    }]);

    messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 2,
        }],
    }]);

    let _current_layout = RwLock::new(Some("some_current_layout"));

    // Create a mock message pipeline provider
    struct MockMessagePipelineProvider {
        messages: Vec<MessageEvent>,
    }

    impl MockMessagePipelineProvider {
        fn new(messages: Vec<MessageEvent>) -> Self {
            Self { messages }
        }

        async fn get_messages(&self) -> Vec<MessageEvent> {
            futures::stream! {
                for message_event in &self.messages {
                    yield message_event.clone();
                }
            }.collect()
        }
    }

    // Create a mock current layout provider
    struct MockCurrentLayoutProvider {}

    impl MockCurrentLayoutProvider {
        async fn get_layout(&self) -> String {
            "some_current_layout".to_string()
        }
    }

    // Create a mock ros datatypes
    struct MockRosDatatypes {
        definitions: HashMap<String, serde_json::Value>,
    }

    impl MockRosDatatypes {
        fn new() -> Self {
            Self {
                definitions: HashMap::new(),
            }
        }

        async fn get_definition(&self, name: &str) -> Result<serde_json::Value, Error> {
            Ok(self.definitions.get(name)?.cloned().unwrap_or_default())
        }
    }

    // Create a basic builder
    struct BasicBuilder {}

    impl BasicBuilder {
        fn string() -> Self {
            Self {}
        }
    }

    type Options = HashMap<String, serde_json::Value>;

    // Create the use message data item hook
    fn use_message_data_item(path: &str) -> Vec<queried_data::QueriedData> {
        let (tx, rx) = bounded(10);

        futures::stream! {
            for message_event in receiver {
                if path eq message_event.topic {
                    tx.send(queried_data::QueriedData {
                        message_event: Some(message_event),
                        queried_data: vec![QueryableDataType {
                            path,
                            value: serde_json::from_value(&message_event.message[&field]).unwrap_or_default(),
                        }],
                    }).await?;
                }
            }
        }.collect::<Vec<queried_data::QueriedData>>()
    }

    // Create the queried data struct
    struct QueryedDataType {
        path: String,
        value: serde_json::Value,
    }

    type QueriedData = Vec<QueryedDataType>;

    let topic = "/topic";
    let field = BasicBuilder.string();
    let path = format!("{}.{}", topic, field);

    // Test cases
    let mut messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    let mut queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 0,
        }],
    }]);

    messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 2,
        }],
    }]);

    let _current_layout = RwLock::new(Some("some_current_layout"));

    // Create a mock message pipeline provider
    struct MockMessagePipelineProvider {
        messages: Vec<MessageEvent>,
    }

    impl MockMessagePipelineProvider {
        fn new(messages: Vec<MessageEvent>) -> Self {
            Self { messages }
        }

        async fn get_messages(&self) -> Vec<MessageEvent> {
            futures::stream! {
                for message_event in &self.messages {
                    yield message_event.clone();
                }
            }.collect()
        }
    }

    // Create a mock current layout provider
    struct MockCurrentLayoutProvider {}

    impl MockCurrentLayoutProvider {
        async fn get_layout(&self) -> String {
            "some_current_layout".to_string()
        }
    }

    // Create a mock ros datatypes
    struct MockRosDatatypes {
        definitions: HashMap<String, serde_json::Value>,
    }

    impl MockRosDatatypes {
        fn new() -> Self {
            Self {
                definitions: HashMap::new(),
            }
        }

        async fn get_definition(&self, name: &str) -> Result<serde_json::Value, Error> {
            Ok(self.definitions.get(name)?.cloned().unwrap_or_default())
        }
    }

    // Create a basic builder
    struct BasicBuilder {}

    impl BasicBuilder {
        fn string() -> Self {
            Self {}
        }
    }

    type Options = HashMap<String, serde_json::Value>;

    // Create the use message data item hook
    fn use_message_data_item(path: &str) -> Vec<queried_data::QueriedData> {
        let (tx, rx) = bounded(10);

        futures::stream! {
            for message_event in receiver {
                if path eq message_event.topic {
                    tx.send(queried_data::QueriedData {
                        message_event: Some(message_event),
                        queried_data: vec![QueryableDataType {
                            path,
                            value: serde_json::from_value(&message_event.message[&field]).unwrap_or_default(),
                        }],
                    }).await?;
                }
            }
        }.collect::<Vec<queried_data::QueriedData>>()
    }

    // Create the queried data struct
    struct QueryedDataType {
        path: String,
        value: serde_json::Value,
    }

    type QueriedData = Vec<QueryableDataType>;

    let topic = "/topic";
    let field = BasicBuilder.string();
    let path = format!("{}.{}", topic, field);

    // Test cases
    let mut messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    let mut queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 0,
        }],
    }]);

    messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 2,
        }],
    }]);

    let _current_layout = RwLock::new(Some("some_current_layout"));

    // Create a mock message pipeline provider
    struct MockMessagePipelineProvider {
        messages: Vec<MessageEvent>,
    }

    impl MockMessagePipelineProvider {
        fn new(messages: Vec<MessageEvent>) -> Self {
            Self { messages }
        }

        async fn get_messages(&self) -> Vec<MessageEvent> {
            futures::stream! {
                for message_event in &self.messages {
                    yield message_event.clone();
                }
            }.collect()
        }
    }

    // Create a mock current layout provider
    struct MockCurrentLayoutProvider {}

    impl MockCurrentLayoutProvider {
        async fn get_layout(&self) -> String {
            "some_current_layout".to_string()
        }
    }

    // Create a mock ros datatypes
    struct MockRosDatatypes {
        definitions: HashMap<String, serde_json::Value>,
    }

    impl MockRosDatatypes {
        fn new() -> Self {
            Self {
                definitions: HashMap::new(),
            }
        }

        async fn get_definition(&self, name: &str) -> Result<serde_json::Value, Error> {
            Ok(self.definitions.get(name)?.cloned().unwrap_or_default())
        }
    }

    // Create a basic builder
    struct BasicBuilder {}

    impl BasicBuilder {
        fn string() -> Self {
            Self {}
        }
    }

    type Options = HashMap<String, serde_json::Value>;

    // Create the use message data item hook
    fn use_message_data_item(path: &str) -> Vec<queried_data::QueriedData> {
        let (tx, rx) = bounded(10);

        futures::stream! {
            for message_event in receiver {
                if path eq message_event.topic {
                    tx.send(queried_data::QueriedData {
                        message_event: Some(message_event),
                        queried_data: vec![QueryableDataType {
                            path,
                            value: serde_json::from_value(&message_event.message[&field]).unwrap_or_default(),
                        }],
                    }).await?;
                }
            }
        }.collect::<Vec<queried_data::QueriedData>>()
    }

    // Create the queried data struct
    struct QueryedDataType {
        path: String,
        value: serde_json::Value,
    }

    type QueriedData = Vec<QueryableDataType>;

    let topic = "/topic";
    let field = BasicBuilder.string();
    let path = format!("{}.{}", topic, field);

    // Test cases
    let mut messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    let mut queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 0,
        }],
    }]);

    messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 2,
        }],
    }]);

    let _current_layout = RwLock::new(Some("some_current_layout"));

    // Create a mock message pipeline provider
    struct MockMessagePipelineProvider {
        messages: Vec<MessageEvent>,
    }

    impl MockMessagePipelineProvider {
        fn new(messages: Vec<MessageEvent>) -> Self {
            Self { messages }
        }

        async fn get_messages(&self) -> Vec<MessageEvent> {
            futures::stream! {
                for message_event in &self.messages {
                    yield message_event.clone();
                }
            }.collect()
        }
    }

    // Create a mock current layout provider
    struct MockCurrentLayoutProvider {}

    impl MockCurrentLayoutProvider {
        async fn get_layout(&self) -> String {
            "some_current_layout".to_string()
        }
    }

    // Create a mock ros datatypes
    struct MockRosDatatypes {
        definitions: HashMap<String, serde_json::Value>,
    }

    impl MockRosDatatypes {
        fn new() -> Self {
            Self {
                definitions: HashMap::new(),
            }
        }

        async fn get_definition(&self, name: &str) -> Result<serde_json::Value, Error> {
            Ok(self.definitions.get(name)?.cloned().unwrap_or_default())
        }
    }

    // Create a basic builder
    struct BasicBuilder {}

    impl BasicBuilder {
        fn string() -> Self {
            Self {}
        }
    }

    type Options = HashMap<String, serde_json::Value>;

    // Create the use message data item hook
    fn use_message_data_item(path: &str) -> Vec<queried_data::QueriedData> {
        let (tx, rx) = bounded(10);

        futures::stream! {
            for message_event in receiver {
                if path eq message_event.topic {
                    tx.send(queried_data::QueriedData {
                        message_event: Some(message_event),
                        queried_data: vec![QueryableDataType {
                            path,
                            value: serde_json::from_value(&message_event.message[&field]).unwrap_or_default(),
                        }],
                    }).await?;
                }
            }
        }.collect::<Vec<queried_data::QueriedData>>()
    }

    // Create the queried data struct
    struct QueryedDataType {
        path: String,
        value: serde_json::Value,
    }

    type QueriedData = Vec<QueryableDataType>;

    let topic = "/topic";
    let field = BasicBuilder.string();
    let path = format!("{}.{}", topic, field);

    // Test cases
    let mut messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    let mut queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 0,
        }],
    }]);

    messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 2,
        }],
    }]);

    let _current_layout = RwLock::new(Some("some_current_layout"));

    // Create a mock message pipeline provider
    struct MockMessagePipelineProvider {
        messages: Vec<MessageEvent>,
    }

    impl MockMessagePipelineProvider {
        fn new(messages: Vec<MessageEvent>) -> Self {
            Self { messages }
        }

        async fn get_messages(&self) -> Vec<MessageEvent> {
            futures::stream! {
                for message_event in &self.messages {
                    yield message_event.clone();
                }
            }.collect()
        }
    }

    // Create a mock current layout provider
    struct MockCurrentLayoutProvider {}

    impl MockCurrentLayoutProvider {
        async fn get_layout(&self) -> String {
            "some_current_layout".to_string()
        }
    }

    // Create a mock ros datatypes
    struct MockRosDatatypes {
        definitions: HashMap<String, serde_json::Value>,
    }

    impl MockRosDatatypes {
        fn new() -> Self {
            Self {
                definitions: HashMap::new(),
            }
        }

        async fn get_definition(&self, name: &str) -> Result<serde_json::Value, Error> {
            Ok(self.definitions.get(name)?.cloned().unwrap_or_default())
        }
    }

    // Create a basic builder
    struct BasicBuilder {}

    impl BasicBuilder {
        fn string() -> Self {
            Self {}
        }
    }

    type Options = HashMap<String, serde_json::Value>;

    // Create the use message data item hook
    fn use_message_data_item(path: &str) -> Vec<queried_data::QueriedData> {
        let (tx, rx) = bounded(10);

        futures::stream! {
            for message_event in receiver {
                if path eq message_event.topic {
                    tx.send(queried_data::QueriedData {
                        message_event: Some(message_event),
                        queried_data: vec![QueryableDataType {
                            path,
                            value: serde_json::from_value(&message_event.message[&field]).unwrap_or_default(),
                        }],
                    }).await?;
                }
            }
        }.collect::<Vec<queried_data::QueriedData>>()
    }

    // Create the queried data struct
    struct QueryedDataType {
        path: String,
        value: serde_json::Value,
    }

    type QueriedData = Vec<QueryableDataType>;

    let topic = "/topic";
    let field = BasicBuilder.string();
    let path = format!("{}.{}", topic, field);

    // Test cases
    let mut messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    let mut queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 0,
        }],
    }]);

    messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 0,
        }],
    }]);

    let _current_layout = RwLock::new(Some("some_current_layout"));

    // Create a mock message pipeline provider
    struct MockMessagePipelineProvider {
        messages: Vec<MessageEvent>,
    }

    impl MockMessagePipelineProvider {
        fn new(messages: Vec<MessageEvent>) -> Self {
            Self { messages }
        }

        async fn get_messages(&self) -> Vec<MessageEvent> {
            futures::stream! {
                for message_event in &self.messages {
                    yield message_event.clone();
                }
            }.collect()
        }
    }

    // Create a mock current layout provider
    struct MockCurrentLayoutProvider {}

    impl MockCurrentLayoutProvider {
        async fn get_layout(&self) -> String {
            "some_current_layout".to_string()
        }
    }

    // Create a mock ros datatypes
    struct MockRosDatatypes {
        definitions: HashMap<String, serde_json::Value>,
    }

    impl MockRosDatatypes {
        fn new() -> Self {
            Self {
                definitions: HashMap::new(),
            }
        }

        async fn get_definition(&self, name: &str) -> Result<serde_json::Value, Error> {
            Ok(self.definitions.get(name)?.cloned().unwrap_or_default())
        }
    }

    // Create a basic builder
    struct BasicBuilder {}

    impl BasicBuilder {
        fn string() -> Self {
            Self {}
        }
    }

    type Options = HashMap<String, serde_json::Value>;

    // Create the use message data item hook
    fn use_message_data_item(path: &str) -> Vec<queried_data::QueriedData> {
        let (tx, rx) = bounded(10);

        futures::stream! {
            for message_event in receiver {
                if path eq message_event.topic {
                    tx.send(queried_data::QueriedData {
                        message_event: Some(message_event),
                        queried_data: vec![QueryableDataType {
                            path,
                            value: serde_json::from_value(&message_event.message[&field]).unwrap_or_default(),
                        }],
                    }).await?;
                }
            }
        }.collect::<Vec<queried_data::QueriedData>>()
    }

    // Create the queried data struct
    struct QueryedDataType {
        path: String,
        value: serde_json::Value,
    }

    type QueriedData = Vec<QueryableDataType>;

    let topic = "/topic";
    let field = BasicBuilder.string();
    let path = format!("{}.{}", topic, field);

    // Test cases
    let mut messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    let mut queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 0,
        }],
    }]);

    messages = vec![
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 1 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
        MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 2 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        },
    ];

    queried_data = use_message_data_item(&path);

    assert_eq!(queried_data, vec![QueryableData {
        message_event: Some(MessageEvent {
            topic: "/topic".to_string(),
            receive_time: std::time::Instant.now(),
            message: serde_json::json!({ "value": 0 }),
            schema_name: "datatype".to_string(),
            size_in_bytes: 0,
        }),
        queried_data: vec![QueryableDataType {
            path,
            value: 0,
        }],
    }]);

    let _current_layout = RwLock::new(Some("some_current_layout"));

    // Create a mock message pipeline provider
    struct MockMessagePipelineProvider {
        messages: Vec<MessageEvent>,
    }

    impl MockMessagePipelineProvider {
        fn new(messages: Vec<MessageEvent>) -> Self {
            Self { messages }
        }

        async fn get_messages(&self) -> Vec<MessageEvent> {
            futures::stream! {
                for message_event in &self.messages {
                    yield message_event.clone();
                }
            }.collect()
        }
    }