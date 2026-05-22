```rust
use crate::types::{LogLevel, LogLevelToString};

fn level_to_string(level: LogLevel) -> &'static str {
    match level {
        LogLevel::DEBUG => "DEBUG",
        LogLevel::INFO => "INFO",
        LogLevel::WARN => "WARN",
        LogLevel::ERROR => "ERROR",
        LogLevel::FATAL => "FATAL",
        _ => unreachable!(),
    }
}
```