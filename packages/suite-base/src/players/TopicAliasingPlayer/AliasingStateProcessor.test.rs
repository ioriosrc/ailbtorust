```rust
use std::collections::{HashMap, HashSet};

struct Topic {
    name: String,
    schema_name: String,
}

struct MessageEvent {
    topic: String,
    receive_time: chrono::DateTime<chrono::Utc>,
    message: Option<String>,
    schema_name: String,
    size_in_bytes: u32,
}

struct PlayerState {
    active_data: Option<MessageData>,
    published_topics: HashMap<&str, HashSet<&str>>,
    subscribed_topics: HashMap<&str, HashSet<&str>>,
    progress: Progress,
}

struct Progress {
    fully_loaded_fraction_ranges: Vec<f64>,
    message_cache: MessageCache,
}

struct MessageCache {
    start_time: chrono::DateTime<chrono::Utc>,
    blocks: Vec<MessageBlock>,
}

struct MessageBlock {
    messages_by_topic: HashMap<&str, Vec<Message>>,
    size_in_bytes: u32,
}

struct AliasingStateProcessor {
    alias_map: HashMap<String, String>,
}

impl AliasingStateProcessor {
    fn new(alias_map: HashMap<String, String>) -> Self {
        Self { alias_map }
    }

    fn process(&self, state: &PlayerState, subscriptions: Vec<MessageEvent>) -> PlayerState {
        let mut mapped = state.clone();

        // Map topics and subscribed topics
        if let Some(active_data) = &mapped.active_data {
            let new_active_data = self.map_topics_and_subscribed_topics(
                active_data,
                &self.alias_map,
                &subscriptions,
            );
            mapped.active_data = Some(new_active_data);
        }

        if let Some(published_topics) = &mut mapped.published_topics {
            let new_published_topics = self.map_published_topics(published_topics, &self.alias_map);
            mapped.published_topics = new_published_topics;
        }

        mapped
    }

    fn map_topics_and_subscribed_topics(
        &self,
        active_data: &MessageData,
        alias_map: &HashMap<String, String>,
        subscriptions: &[MessageEvent],
    ) -> MessageData {
        let mut mapped_messages = HashMap::new();

        for topic in &active_data.messages {
            if let Some(new_topic) = alias_map.get(&topic.topic) {
                mapped_messages.insert(*new_topic, topic.clone());
            } else {
                mapped_messages.insert(topic.topic, topic.clone());
            }
        }

        MessageData {
            messages: mapped_messages.into_iter().collect(),
            published_topics: active_data.published_topics.clone(),
            subscribed_topics: active_data.subscribed_topics.clone(),
            progress: active_data.progress.clone(),
        }
    }

    fn map_published_topics(
        &self,
        published_topics: &HashMap<&str, HashSet<&str>>,
        alias_map: &HashMap<String, String>,
    ) -> HashMap<&str, HashSet<&str>> {
        let mut new_published_topics = HashMap::new();

        for (topic_name, subscribed_topics) in published_topics {
            if let Some(new_topic) = alias_map.get(topic_name) {
                new_published_topics.insert(*new_topic, subscribed_topics.clone());
            } else {
                new_published_topics.insert(topic_name, subscribed_topics.clone());
            }
        }

        new_published_topics
    }
}

struct MessageData {
    messages: Vec<Message>,
    published_topics: HashMap<&str, HashSet<&str>>,
    subscribed_topics: HashMap<&str, HashSet<&str>>,
    progress: Progress,
}

fn mock_player_state(
    active_data: Option<MessageData>,
    topics: Vec<Topic>,
) -> PlayerState {
    PlayerState {
        active_data,
        published_topics: HashMap::new(),
        subscribed_topics: HashMap::new(),
        progress: Progress {
            fully_loaded_fraction_ranges: vec![],
            message_cache: MessageCache {
                start_time: chrono::Utc::now().with_timezone(&chrono::offset::Local),
                blocks: Vec::new(),
            },
        },
    }
}

fn main() {
    // Test cases go here
}
```