```rust
use std::time::{Duration, Instant};

#[derive(Debug)]
struct MessageEvent<T> {
    topic: String,
    schema_name: String,
    receive_time: Duration,
    message: T,
    size_in_bytes: usize,
}

impl<T> MessageEvent<T> {
    fn new(topic: String, schema_name: String, message: T) -> Self {
        Self {
            topic,
            schema_name,
            receive_time: Instant::now(),
            message,
            size_in_bytes: std::mem::size_of::<T>(),
        }
    }

    fn with_fields(mut self, fields: impl IntoIterator<Item = (String, String)>) -> Self {
        for (key, value) in fields.into_iter() {
            match key.as_str() {
                "topic" => self.topic = value,
                "schemaName" => self.schema_name = value,
                _ => {}
            }
        }

        self
    }
}

fn mock_message<T>(message: T, fields: Option<impl IntoIterator<Item = (String, String)>>) -> MessageEvent<T> {
    let mut event = MessageEvent::new("topic".to_string(), "schema".to_string(), message);

    if let Some(fields) = fields {
        event = event.with_fields(fields);
    }

    event
}
```