```rust
use chrono::{NaiveDateTime, TimeZone};
use log::Level;

fn level_to_string(level: Level) -> String {
    match level {
        Level::Trace => "TRACE",
        Level::Debug => "DEBUG",
        Level::Info => "INFO",
        Level::Warning => "WARNING",
        Level::Error => "ERROR",
        Level::Critical => "CRITICAL",
    }
}

fn format_time(item: NaiveDateTime, timezone: Option<&str>) -> String {
    match timezone {
        Some(timezone_str) => item.to_rfc3339_opts(chrono::Rfc3339Options::ALL | chrono::Rfc3339Options::TIMEZONE, timezone_str),
        None => item.to_rfc3339_opts(chrono::Rfc3339Options::ALL, ""),
    }
}

fn format_messages(items: Vec<NormalizedLogMessage>, timezone: Option<&str>) -> Vec<String> {
    items.into_iter()
        .map(|item| format!("[{}] [{}] [{:?}] {}", level_to_string(item.level), format_time(item.stamp, timezone), item.name, item.message))
        .collect()
}
```