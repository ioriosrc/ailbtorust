```rust
use std::time::{SystemTime, UNIX_EPOCH};

fn format_time(sec: i64, nsec: u32) -> String {
    let datetime = UNIX_EPOCH + SystemTime::from_secs_f64((sec as f64 + nsec as f64 / 1_000_000_000.0));
    datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
}

fn format_date(sec: i64, nsec: u32) -> String {
    let datetime = UNIX_EPOCH + SystemTime::from_secs_f64((sec as f64 + nsec as f64 / 1_000_000_000.0));
    datetime.format("%Y-%m-%d").to_string()
}

fn format_duration(sec: i64, nsec: u32) -> String {
    let total_seconds = sec + (nsec as f64 / 1_000_000_000.0);
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02}.{:09}", hours, minutes, seconds, (nsec as f64 * 1_000_000.0).round() as u32)
}

fn format_time_str(text: &str, timezone: &str) -> Option<(i64, u32)> {
    let date_time = time::OffsetDateTime::parse(text, "%Y-%m-%d %H:%M:%S.%f").ok()?;
    let datetime = date_time.with_timezone(timezone);
    Some((datetime.timestamp() as i64, datetime.nanosecond()))
}

fn get_validated_time_and_method_from_string(text: &str, timezone: &str) -> Option<(time::SystemTime, &str)> {
    if text.is_empty() || !text.contains('.') || !text.contains(':') {
        return None;
    }

    let mut parts = text.split_whitespace();
    let time_str = parts.next().unwrap_or("");
    let method = parts.next().unwrap_or("").to_lowercase();

    match (method, format_time_str(time_str, timezone)) {
        ("sec", Some((sec, nsec))) => Some((UNIX_EPOCH + SystemTime::from_secs_f64(sec as f64 + nsec as f64 / 1_000_000_000.0)), "SEC"),
        ("tod", Some((sec, nsec))) => Some((UNIX_EPOCH + SystemTime::from_secs_f64(sec as f64 + nsec as f64 / 1_000_000_000.0)), "TOD"),
        _ => None,
    }
}
```