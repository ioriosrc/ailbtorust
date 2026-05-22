```rust
use std::collections::{HashMap, VecDeque};

// Define the necessary structures and types for Rust code
struct RosTime {
    sec: u64,
    nsec: u32,
}

impl RosTime {
    fn time(&self) -> i64 {
        self.sec as i64 * 1_000_000_000 + self.nsec as i64
    }
}

struct Metadata {
    name: String,
    value: i32,
}

struct TopicStats {
    num_messages: u32,
    first_message_time: RosTime,
    last_message_time: RosTime,
}

// Define the necessary structures and types for Rust code
fn merge_metadata(metadata1: &Vec<Metadata>, metadata2: &Option<Vec<Metadata>>) -> Vec<Metadata> {
    let mut merged = Vec::new();
    for metadata in metadata1.iter().chain(metadata2.as_ref().unwrap_or(&vec![])) {
        merged.push(metadata.clone());
    }
    merged
}

fn accumulate_map(map1: HashMap<String, i32>, map2: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut acc = map1;
    for (key, value) in map2 {
        if acc.contains_key(key) {
            acc.insert(key, acc[key] + *value);
        } else {
            acc.insert(key, *value);
        }
    }
    acc
}

fn set_start_time(time1: RosTime, time2: RosTime) -> RosTime {
    RosTime {
        sec: if time1.sec < time2.sec { time1.sec } else { time2.sec },
        nsec: if time1.nsec < time2.nsec { time1.nsec } else { time2.nsec },
    }
}

fn set_end_time(time1: RosTime, time2: RosTime) -> RosTime {
    RosTime {
        sec: if time1.sec > time2.sec { time1.sec } else { time2.sec },
        nsec: if time1.nsec > time2.nsec { time1.nsec } else { time2.nsec },
    }
}

fn merge_topic_stats(stats_map1: &HashMap<String, TopicStats>, stats_map2: &HashMap<String, TopicStats>) -> HashMap<String, TopicStats> {
    let mut result = HashMap::new();
    for (topic, stats) in stats_map1.iter().chain(stats_map2.iter()) {
        let existing_stats = result.get(topic).unwrap_or(&TopicStats {
            num_messages: 0,
            first_message_time: RosTime { sec: 0, nsec: 0 },
            last_message_time: RosTime { sec: 0, nsec: 0 },
        });

        let new_stats = TopicStats {
            num_messages: existing_stats.num_messages + stats.num_messages,
            first_message_time: set_start_time(existing_stats.first_message_time, stats.first_message_time),
            last_message_time: set_end_time(existing_stats.last_message_time, stats.last_message_time),
        };

        result.insert(topic.clone(), new_stats);
    }
    result
}
```