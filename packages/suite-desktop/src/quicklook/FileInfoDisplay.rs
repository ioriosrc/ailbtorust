```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[wasm_bindgen]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub compression_types: Option<Vec<String>>,
    pub topics: Option<Vec<TopicInfo>>,
    pub num_chunks: u32,
    pub total_messages: u32,
    pub num_attachments: u32,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct TopicInfo {
    pub topic: String,
    pub schema_name: String,
    pub num_messages: u32,
    pub num_connections: u32,
}

fn format_byte_size(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    let (bytes, postfix) = match bytes {
        _ if bytes >= 1099512L => (bytes / 1099512L, "GB"),
        _ if bytes >= 1048576L => (bytes / 1048576L, "MB"),
        _ if bytes >= 1024L => (bytes / 1024L, "KB"),
        _ => (bytes, "B"),
    };
    format!("{:.1} {}", bytes as f64, postfix)
}

fn format_time_raw(stamp: u64) -> String {
    if stamp < 0 {
        log_error!("Times are not allowed to be negative");
        return "(invalid negative time)";
    }
    format!("{:09}", stamp)
}

#[wasm_bindgen]
pub fn main() {
    console_log!("Hello, World!");
}
```