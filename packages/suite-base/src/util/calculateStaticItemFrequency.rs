```rust
use chrono::Duration; // Assuming chrono crate is used for time manipulation

pub fn calculate_static_item_frequency(
    num_messages: usize,
    first_message_time: Option<chrono::DateTime<Utc>>,
    last_message_time: Option<chrono::DateTime<Utc>>,
    duration: Duration,
) -> Option<f64> {
    if first_message_time.is_none() || last_message_time.is_none() {
        let full_duration = duration.as_secs_f64();
        if full_duration > 0.0 {
            return Some(num_messages as f64 / full_duration);
        } else {
            return None;
        }
    }

    if num_messages < 2 || first_message_time == last_message_time {
        return None;
    }

    let topic_duration = last_message_time - first_message_time;
    if topic_duration.as_secs_f64() > 0.0 {
        return Some((num_messages as f64 - 1.0) / topic_duration.as_secs_f64());
    } else {
        return None;
    }
}
```