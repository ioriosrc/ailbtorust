```rust
use std::cmp::Ordering;

fn get_topic_match_prefix(topic: &str) -> Option<&str> {
    let mut parts = topic.split('/');
    if parts.len() > 1 && parts.last().unwrap() == "" {
        None // No / at the end
    } else if parts.last().unwrap() != "" {
        Some(parts.iter().take(parts.len() - 1).collect::<String>())
    } else {
        Some("")
    }
}

fn sort_prefix_matchesToFront<T: Ord>(array: &mut Vec<T>, prefix: &str, key_fn: fn(&T) -> String) {
    array.sort_by(|a, b| {
        let a_key = key_fn(a);
        let b_key = key_fn(b);

        if a_key.starts_with(prefix) && !b_key.starts_with(prefix) {
            Ordering::Less
        } else if !a_key.starts_with(prefix) && b_key.starts_with(prefix) {
            Ordering::Greater
        } else {
            a_key.cmp(&b_key)
        }
    });
}
```