```rust
use std::time::{Duration, SystemTime};

fn calculate_static_item_frequency(num_messages: u64, first_message_time: Option<SystemTime>, last_message_time: Option<SystemTime>, topic_duration: Duration) -> Option<f32> {
    if num_messages < 2 || first_message_time == Some(last_message_time) {
        return None;
    }

    let start_time = first_message_time.unwrap();
    let end_time = last_message_time.unwrap();

    // Calculate the duration of the topic
    let topic_duration_seconds = topic_duration.as_secs_f32();
    let topic_duration_nanos = topic_duration.subsec_nanos() as f32 / 1e9;

    let total_elapsed_seconds = end_time.duration_since(start_time).as_secs_f32();
    let total_elapsed_nanos = end_time.duration_since(start_time).subsec_nanos();

    // Calculate the number of seconds between the first and last message
    let time_between_messages_seconds = (end_time - start_time) as f32;

    if topic_duration_seconds == 0.0 {
        return None;
    }

    Some(time_between_messages_seconds / topic_duration_seconds)
}
```