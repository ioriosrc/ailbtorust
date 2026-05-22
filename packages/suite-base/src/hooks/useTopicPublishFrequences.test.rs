```rust
use std::collections::HashMap;

fn calculate_frequencies(active_data: &PlayerState) -> HashMap<&str, f64> {
    let mut freqs = HashMap::new();

    for (topic, stats) in &active_data.topic_stats {
        if stats.num_messages > 0 {
            let duration = stats.last_message_time - stats.first_message_time;
            let seconds = duration.sec as f64 + duration.nsec as f64 / 1_000_000_000.0;
            freqs.insert(topic, seconds / stats.num_messages);
        }
    }

    if !freqs.is_empty() {
        freqs
    } else {
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PlayerState, PLAYER_CAPABILITIES};

    #[test]
    fn static_source() {
        let active_data = PlayerState {
            activeData: Partial::default(),
            capabilities: vec![PLAYER_CAPABILITIES.playbackControl],
        };

        let expected = HashMap::from([
            ("topic_a", 2.25),
            ("topic_b", 3.8),
        ]);

        assert_eq!(calculate_frequencies(&active_data), expected);
    }

    #[test]
    fn live_source() {
        let active_data: PlayerState = PlayerState {
            activeData: Partial::default(),
            capabilities: vec![PLAYER_CAPABILITIES.playbackControl],
        };

        // Simulate some time passing
        let mut active_data = active_data.clone();
        active_data.activeData.current_time = { sec: 3, nsec: 0 };
        active_data.activeData.endTime = { sec: 10, nsec: 0 };
        active_data.activeData.startTime = { sec: 0, nsec: 0 };

        let updated_active_data = PlayerState {
            activeData,
            capabilities: vec![PLAYER_CAPABILITIES.playbackControl],
        };

        assert!(calculate_frequencies(&updated_active_data).is_empty());
    }
}
```