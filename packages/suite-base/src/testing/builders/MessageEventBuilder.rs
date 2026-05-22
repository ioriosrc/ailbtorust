```rust
use std::time::SystemTime;

pub struct MessageEvent<T> {
    pub message: T,
    pub publish_time: SystemTime,
    pub receive_time: SystemTime,
    pub schema_name: String,
    pub size_in_bytes: u64,
    pub topic: String,
    pub topic_config: serde_json::Value,
}

impl<T> MessageEvent<T> {
    pub fn new(message: T, publish_time: SystemTime, receive_time: SystemTime, schema_name: String, size_in_bytes: u64, topic: String, topic_config: serde_json::Value) -> Self {
        MessageEvent {
            message,
            publish_time,
            receive_time,
            schema_name,
            size_in_bytes,
            topic,
            topic_config,
        }
    }
}

pub struct MessageEventBuilder {}

impl MessageEventBuilder {
    pub fn message_event<T>(props: Option<MessageEventProps<T>>) -> MessageEvent<T> {
        let props = props.unwrap_or_default();
        MessageEvent::new(
            props.message,
            props.publish_time,
            props.receive_time,
            props.schema_name,
            props.size_in_bytes,
            props.topic,
            props.topic_config,
        )
    }

    pub fn message_events(count: usize) -> Vec<MessageEvent> {
        (0..count).map(|_| self.message_event(None)).collect()
    }
}

pub struct MessageEventProps<T> {
    pub message: T,
    pub publish_time: SystemTime,
    pub receive_time: SystemTime,
    pub schema_name: String,
    pub size_in_bytes: u64,
    pub topic: String,
    pub topic_config: serde_json::Value,
}
```