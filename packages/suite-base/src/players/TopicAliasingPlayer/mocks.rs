```rust
use std::time::{Duration, Instant};

pub struct MessageEvent<T> {
    topic: String,
    schema_name: String,
    receive_time: Duration,
    message: T,
    size_in_bytes: usize,
}

impl<T> MessageEvent<T> {
    pub fn new(topic: &str, schema_name: &str, message: T) -> Self {
        let receive_time = Instant::now();
        Self {
            topic: String::from(topic),
            schema_name: String::from(schema_name),
            receive_time,
            message,
            size_in_bytes: 1,
        }
    }
}

pub struct PlayerState {
    active_data: PlayerStateActiveData,
    capabilities: Vec<String>,
    presence: PlayerPresence,
    profile: Option<serde_json::Value>,
    playerId: String,
    progress: Progress,
}

impl PlayerState {
    pub fn new(
        overrides: &Option<Vec<(String, serde_json::Value)>>,
        data_overrides: &Option<PlayerStateActiveData>,
    ) -> Self {
        let mut active_data = PlayerStateActiveData {
            messages: Vec::new(),
            currentTime: Instant::now() - Duration::from_secs(1),
            endTime: Instant::now() - Duration::from_secs(2),
            last_seek_time: 1,
            topics: Vec::new(),
            speed: 1.0,
            is_playing: false,
            topic_stats: std::collections::HashMap::new(),
            startTime: Instant::now() - Duration::from_secs(3),
            datatypes: std::collections::HashSet::new(),
            total_bytes_received: 0,
        };

        if let Some(data) = data_overrides {
            active_data.update_with(&data);
        }

        PlayerState {
            active_data,
            capabilities: Vec::new(),
            presence: PlayerPresence::PRESENT,
            profile: None,
            playerId: String::from("1"),
            progress: Progress {
                fully_loaded_fraction_ranges: Vec::new(),
                message_cache: None,
            },
        }
    }
}

#[derive(Debug)]
struct PlayerStateActiveData {
    messages: Vec<MessageEvent<serde_json::Value>>,
    currentTime: Instant,
    endTime: Instant,
    last_seek_time: u64,
    topics: Vec<String>,
    speed: f64,
    is_playing: bool,
    topic_stats: std::collections::HashMap<String, f64>,
    startTime: Instant,
    datatypes: std::collections::HashSet<serde_json::Value>,
    total_bytes_received: usize,
}

impl PlayerStateActiveData {
    fn update_with(&mut self, other: &PlayerStateActiveData) {
        // Update with the data from other
        self.messages.extend(other.messages.iter().cloned());
        self.currentTime = other.currentTime.max(self.currentTime);
        self.endTime = other.endTime.max(self.endTime);
        self.last_seek_time = std::cmp::max(self.last_seek_time, other.last_seek_time);
        self.topics.extend(other.topics.clone());
        self.speed = std::cmp::max(self.speed, other.speed);
        self.is_playing = self.is_playing || other.is_playing;
        self.topic_stats.extend(other.topic_stats.iter().cloned());
        self.datatypes.extend(other.datatypes.iter().cloned());
        self.total_bytes_received += other.total_bytes_received;
    }
}

pub struct Progress {
    fully_loaded_fraction_ranges: Vec<(u64, u64)>,
    message_cache: Option<Vec<MessageEvent<serde_json::Value>>>,
}
```