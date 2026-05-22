```rust
use std::collections::HashMap;

fn is_log_schema(schema_name: Option<&str>) -> bool {
    schema_name.is_some() && LOG_SCHEMAS.contains(schema_name)
}

pub fn is_topic_high_frequency(
    topic_stats: &HashMap<String, TopicStats>,
    topic: TopicFrequencyInfo,
    duration: Option<Time>,
) -> bool {
    if let Some(duration) = duration {
        if !topic.schema_name.is_none() || is_log_schema(topic.schema_name.as_ref()) {
            return false;
        }

        let topic_stat = topic_stats.get(&topic.name).unwrap();
        let frequency = calculate_static_item_frequency(
            topic_stat.num_messages,
            topic_stat.first_message_time,
            topic_stat.last_message_time,
            duration,
        );

        if let Some(frequency) = frequency {
            if frequency > FREQUENCY_LIMIT {
                return true;
            }
        }
    }

    false
}
```