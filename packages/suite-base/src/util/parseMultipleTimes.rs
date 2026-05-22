```rust
use chrono::{NaiveDateTime, Utc};

fn parse_timestamp_str(time_str: &str) -> Option<chrono::DateTime<Utc>> {
    if time_str.trim().is_empty() {
        return None;
    }

    match time_str.parse::<f64>() {
        Ok(time_number) => from_unix_time_seconds(i64::round(time_number)),
        Err(_) => parse_time_str_strict(time_str),
    }
}

fn from_unix_time_seconds(seconds: i64) -> Option<chrono::DateTime<Utc>> {
    NaiveDateTime::from_timestamp_opt(seconds, 0)
}
```