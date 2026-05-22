```rust
use chrono::{DateTime, Utc};
use foxglove::schemas::{Log as FoxgloveLog, LogLevel};
use rosidl_types::Duration;

use crate::types::{
    NormalizedLogMessage,
    Ros1RosgraphMsgsLog,
    Ros2RosgraphMsgsLog,
};

pub fn get_normalized_message(log_message: &LogMessageEvent) -> String {
    log_message.message.as_ref().unwrap_or_default()
}

pub fn get_normalized_level(
    datatype: &str,
    raw: &LogMessageEvent,
) -> LogLevel {
    match datatype {
        "foxglove_msgs/Log" | "foxglove_msgs/msg/Log" | "foxglove::Log" | "foxglove.Log" => {
            if let Some(level) = raw.level {
                level.into()
            } else {
                LogLevel::UNKNOWN
            }
        },
        "rosgraph_msgs/Log" | "rcl_interfaces/msg/Log" => {
            let level = raw.header.stamp.duration_since_midnight();
            match level.whole_secs() + level.nsec as f64 / 1e9 {
                0.0..=1.0 => LogLevel::DEBUG,
                2.0..=3.0 => LogLevel::INFO,
                4.0..=5.0 => LogLevel::WARN,
                8.0..=9.0 => LogLevel::ERROR,
                16.0..=17.0 => LogLevel::FATAL,
                _ => LogLevel::UNKNOWN,
            }
        },
        _ => LogLevel::UNKNOWN,
    }
}

pub fn get_normalized_stamp(datatype: &str, raw: &LogMessageEvent) -> DateTime<Utc> {
    match datatype {
        "foxglove_msgs/Log" | "foxglove_msgs/msg/Log" | "foxglove::Log" | "foxglove.Log" => {
            if let Some(timestamp) = raw.timestamp {
                timestamp.into()
            } else {
                Utc::now()
            }
        },
        "rosgraph_msgs/Log" => {
            raw.header.stamp.clone().into()
        },
        "rcl_interfaces/msg/Log" => {
            raw.stamp.clone().into()
        },
        _ => Utc::now(),
    }
}

pub fn normalized_log_message(datatype: &str, raw: LogMessageEvent) -> NormalizedLogMessage {
    let message = get_normalized_message(raw);
    let stamp = get_normalized_stamp(datatype, raw);
    let level = get_normalized_level(datatype, raw);

    NormalizedLogMessage {
        message,
        stamp,
        level,
        name: raw.name.clone(),
        file: raw.file.clone(),
        line: raw.line.clone(),
    }
}

fn ros_level_to_log_level(ros_level: u8) -> LogLevel {
    match ros_level {
        1 | 10 => LogLevel::DEBUG,
        2 | 20 => LogLevel::INFO,
        4 | 30 => LogLevel::WARN,
        8 | 40 => LogLevel::ERROR,
        16 | 50 => LogLevel::FATAL,
        _ => LogLevel::UNKNOWN,
    }
}
```