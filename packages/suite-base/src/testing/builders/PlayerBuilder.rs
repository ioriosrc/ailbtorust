```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubscribePayload {
    fields: Vec<String>,
    preload_type: Option<SubscriptionPreloadType>,
    topic: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopicSelection {
    messages_by_topic: HashMap<String, Vec<MessageEventBuilder>>,
    need_topics: TopicSelectionBuilder,
    size_in_bytes: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MessageBlock {
    messages_by_topic: HashMap<String, Vec<MessageEventBuilder>>,
    need_topics: TopicSelectionBuilder,
    size_in_bytes: u64,
}

impl Default for SubscribePayload {
    fn default() -> Self {
        SubscribePayload {
            fields: vec![],
            preload_type: Some(SubscriptionPreloadType::FULL),
            topic: String::new(),
        }
    }
}

impl Default for TopicSelectionBuilder {
    fn default() -> Self {
        TopicSelectionBuilder::default()
    }
}

impl Default for MessageBlock {
    fn default() -> Self {
        MessageBlock {
            messages_by_topic: HashMap::new(),
            need_topics: TopicSelectionBuilder::default(),
            size_in_bytes: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerStateActiveData {
    currentTime: RosTimeBuilder,
    datatypes: HashMap<String, Option<RosDatatypeBuilder>>,
    endTime: RosTimeBuilder,
    is_playing: bool,
    last_seek_time: u64,
    messages: Vec<MessageEventBuilder>,
    speed: f64,
    startTime: RosTimeBuilder,
    topics: Vec<Topic>,
    topic_stats: HashMap<String, TopicStats>,
    total_bytes_received: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerState {
    capabilities: Vec<PlayerCapability>,
    name: String,
    playerId: String,
    presence: PlayerPresence,
    profile: String,
    progress: Progress,
}

impl Default for SubscribePayload {
    fn default() -> Self {
        SubscribePayload::default()
    }
}

impl Default for TopicSelectionBuilder {
    fn default() -> Self {
        TopicSelectionBuilder::default()
    }
}

impl Default for MessageBlock {
    fn default() -> Self {
        MessageBlock {
            messages_by_topic: HashMap::new(),
            need_topics: TopicSelectionBuilder::default(),
            size_in_bytes: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerStateActiveData {
    currentTime: RosTimeBuilder,
    datatypes: HashMap<String, Option<RosDatatypeBuilder>>,
    endTime: RosTimeBuilder,
    is_playing: bool,
    last_seek_time: u64,
    messages: Vec<MessageEventBuilder>,
    speed: f64,
    startTime: RosTimeBuilder,
    topics: Vec<Topic>,
    topic_stats: HashMap<String, TopicStats>,
    total_bytes_received: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerState {
    capabilities: Vec<PlayerCapability>,
    name: String,
    playerId: String,
    presence: PlayerPresence,
    profile: String,
    progress: Progress,
}
```