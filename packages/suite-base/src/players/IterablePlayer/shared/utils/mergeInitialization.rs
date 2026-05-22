```rust
use chrono::{DateTime, Duration, Utc};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub fn set_start_time(accumulated: &DateTime<Utc>, current: &DateTime<Utc>) -> DateTime<Utc> {
    if compare(&current, accumulated) < 0 {
        current.clone()
    } else {
        accumulated.clone()
    }
}

pub fn set_end_time(accumulated: &DateTime<Utc>, current: &DateTime<Utc>) -> DateTime<Utc> {
    if compare(current, accumulated) > 0 {
        current.clone()
    } else {
        accumulated.clone()
    }
}

pub fn merge_metadata(
    accumulated: Option<&Vec<InitMetadata>>,
    current: Option<&Vec<InitMetadata>>,
) -> Vec<InitMetadata> {
    let mut merged = Vec::new();
    if let Some(a) = accumulated {
        merged.extend(a);
    }
    if let Some(c) = current {
        merged.extend(c);
    }
    merged
}

pub fn accumulate_map<K, V>(
    accumulated: &mut std::collections::HashMap<String, V>,
    current: &std::collections::HashMap<String, V>,
) {
    for (k, v) in current {
        if !accumulated.contains_key(k) {
            accumulated.insert(k.clone(), v.clone());
        } else {
            let acc_v = accumulated.get_mut(k).unwrap();
            *acc_v += v;
        }
    }
}

pub fn merge_topic_stats<K>(
    accumulated: &mut std::collections::HashMap<String, InitTopicStatsMap>,
    current: &std::collections::HashMap<String, InitTopicStatsMap>,
) {
    for (topic, stats) in current {
        if !accumulated.contains_key(topic) {
            accumulated.insert(topic.clone(), InitTopicStatsMap {
                num_messages: 0,
            });
        }
        let acc_stats = accumulated.get_mut(topic).unwrap();
        acc_stats.num_messages += stats.num_messages;

        if let Some(first_msg_time) = stats.first_message_time {
            if !acc_stats.first_message_time.is_none() && compare(&first_msg_time, acc_stats.first_message_time.as_ref()) < 0 {
                acc_stats.first_message_time = Some(first_msg_time);
            }
        }

        if let Some(last_msg_time) = stats.last_message_time {
            if !acc_stats.last_message_time.is_none() && compare(&last_msg_time, acc_stats.last_message_time.as_ref()) > 0 {
                acc_stats.last_message_time = Some(last_msg_time);
            }
        }
    }
}
```