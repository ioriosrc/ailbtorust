```rust
use std::fmt::{Display, Formatter};

pub struct TimeFormatError;

impl Display for TimeFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(invalid negative time)")
    }
}

pub fn format_time_raw(time_event: &time_event::MessageEvent) -> Result<String, TimeFormatError> {
    let nsec = if time_event.message.header.stamp.nsec > 999000000 {
        time_event.message.header.stamp.nsec % 1000000000
    } else {
        time_event.message.header.stamp.nsec
    };

    Ok(format!(
        "{:.6}",
        (time_event.message.header.stamp.sec as f64) + nsec as f64 / 1e9
    ))
}

pub fn get_timestamp_for_message_event(
    message_base: &time_event::MessageEvent,
    stamp_field: &'static str,
) -> Option<time_event::Time> {
    if let Some(header_stamp) = message_base.message.header.get(stamp_field) {
        if header_stamp.nsec > 999000000 {
            header_stamp.nsec % 1000000000
        } else {
            header_stamp.nsec
        }
    } else {
        None
    }
}

pub fn parse_time_url_string(time_str: &str) -> Option<time_event::Time> {
    if let Ok(timestamp) = time_str.parse::<f64>() {
        Some(time_event::Time {
            sec: (timestamp as i32),
            nsec: (timestamp % 1.0) * 1e9 as i32,
        })
    } else {
        None
    }
}
```