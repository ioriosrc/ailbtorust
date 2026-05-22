```rust
use std::collections::HashMap;

struct RosTime { sec: i32, nsec: i32 };
struct TopicStats { num_messages: usize };
const LOG_SCHEMAS: Vec<&str> = vec!["log", "error"];

fn is_topic_high_frequency(topic_stats: &HashMap<String, TopicStats>, topic: &Topic, duration: &RosTime) -> bool {
    let high_frequency_topic_name = if let Some(name) = topic.name.as_ref() {
        name.to_string()
    } else {
        return false;
    };

    for (schema, stats) in topic_stats.iter() {
        if schema == "log" || schema == "error" {
            continue;
        }

        if stats.num_messages > 1000 && stats.num_messages < 2000 {
            return true;
        }
    }

    false
}

// Example usage:
fn main() {
    // Assume these structures are defined elsewhere
    let topic_stats = HashMap::from([
        ("topic1".to_string(), TopicStats { num_messages: 1500 }),
        ("topic2".to_string(), TopicStats { num_messages: 3000 }),
    ]);
    let topic = Topic {
        name: Some("topic2".to_string()),
        schema_name: "data".to_string(),
    };
    let duration = RosTime { sec: 30, nsec: 15 };

    println!("{}", is_topic_high_frequency(&topic_stats, &topic, &duration)); // Output: true
}
```