```rust
use serde_json::{self, Value};
use chrono::prelude::*;

use crate::{
    custom_typography::custom_typography,
    log_message::{LevelToString, NormalizedLogMessage},
};

pub fn log_message(props: &NormalizedLogMessage) -> Value {
    let str_level = LevelToString(props.level);
    let stamp = props.stamp;
    let lines = props.message.split("\n");
    
    serde_json::to_value(&[
        ("info/stamp/name".to_string(), format!(
            "[{}][{}] {}",
            pad_str(str_level, 5),
            stamp.to_rfc3339(),
            if !props.name.is_empty() {
                format!("{}:", props.name)
            } else {
                String::new()
            }
        )),
        ("extra_lines".to_string(), lines.iter().map(|line| format!(" {}", line)).join("\n")),
    ])
}

fn pad_str(input: &str, width: usize) -> String {
    input.to_string().chars().chain(std::iter::repeat(' ')).take(width).collect()
}
```