```rust
use std::collections::HashMap;

fn simple_get_message_path_data_items<T>(message: &MessageEvent, path: &[&str]) -> Vec<T> {
    match message.topic.split('/').nth(1) {
        Some(topic) if topic != "foo" => Vec::new(),
        _ => {
            let mut data = HashMap::<String, T>::new();
            for item in parse_message_path(path) {
                if item.is_global() {
                    continue;
                }
                let value = get_nested_value(message.message, &item);
                data.insert(item.name.clone(), value);
            }

            data.values().cloned().collect()
        },
    }
}

fn parse_message_path(path: &[&str]) -> Vec<Part> {
    path.iter().map(|part| Part::from(part)).collect()
}

struct Part {
    name: String,
    is_global: bool,
}

impl From<&str> for Part {
    fn from(part: &str) -> Self {
        if part.starts_with("$") {
            Part {
                name: part[1..].to_string(),
                is_global: true,
            }
        } else {
            Part {
                name: part.to_string(),
                is_global: false,
            }
        }
    }
}

fn get_nested_value<'a>(data: &'a serde_json::Value, path: &[Part]) -> &'a serde_json::Value {
    let mut current = data;

    for part in path {
        match &current {
            serde_json::Value::Object(obj) => {
                if let Some(value) = obj.get(&part.name) {
                    current = value;
                } else {
                    return &serde_json::Value::Null;
                }
            },
            serde_json::Value::Array(arr) => {
                if arr.len() > 0 {
                    current = &arr[part.index as usize];
                } else {
                    return &serde_json::Value::Null;
                }
            },
            _ => {
                return &serde_json::Value::Null;
            }
        }
    }

    current
}

struct MessageEvent {
    topic: String,
    receive_time: serde_json::Time,
    size_in_bytes: u64,
    schema_name: String,
    message: serde_json::Value,
}
```