```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct DiagnosticInfo {
    pub display_name: String,
    pub id: String,
    pub status: DiagnosticStatusMessage,
    pub stamp: ros2_time::Clock,
}

#[derive(Clone, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct DiagnosticStatusMessage {
    pub name: String,
    pub hardware_id: String,
    pub level: u8, // see LEVELS in DiagnosticSummary/constants.rs
    pub message: String,
    pub values: Vec<KeyValue>,
}

#[derive(Clone, Debug)]
pub struct DiagnosticStatusArrayMsg {
    pub header: Header,
    pub status: Vec<DiagnosticStatusMessage>,
}

#[derive(Clone, Debug)]
pub struct Header {
    pub frame_id: String,
    pub stamp: ros2_time::Clock,
    pub seq: u32,
}

fn default_diag_status_config(props: Option<&str>) -> DiagnosticStatusConfig {
    let props = props.unwrap_or("{}");
    serde_json::from_str::<DiagnosticStatusConfig>(props).unwrap()
}

fn default_diag_summary_config(props: Option<&str>) -> DiagnosticSummaryConfig {
    let props = props.unwrap_or("{}");
    serde_json::from_str::<DiagnosticSummaryConfig>(props).unwrap()
}

fn default_header(props: Option<&str>) -> Header {
    let props = props.unwrap_or("{}");
    serde_json::from_str::<Header>(props).unwrap()
}

fn default_key_value(props: Option<&str>) -> KeyValue {
    let props = props.unwrap_or("{}");
    serde_json::from_str::<KeyValue>(props).unwrap()
}

fn default_key_values(count: usize) -> Vec<KeyValue> {
    (0..count)
        .map(|_| KeyValue {
            key: format!("key{}", count),
            value: format!("value{}", count),
        })
        .collect()
}

fn default_status_message(props: Option<&str>) -> DiagnosticStatusMessage {
    let props = props.unwrap_or("{}");
    serde_json::from_str::<DiagnosticStatusMessage>(props).unwrap()
}

fn default_status_messages(count: usize) -> Vec<DiagnosticStatusMessage> {
    (0..count)
        .map(|_| DiagnosticStatusMessage {
            name: format!("name{}", count),
            hardware_id: format!("hardware_id{}", count),
            level: 2, // example level value
            message: format!("message{}", count),
            values: default_key_values(3),
        })
        .collect()
}

fn default_status_array_msg(props: Option<&str>) -> DiagnosticStatusArrayMsg {
    let props = props.unwrap_or("{}");
    serde_json::from_str::<DiagnosticStatusArrayMsg>(props).unwrap()
}
```