```rust
use std::collections::HashMap;

pub struct TopicSelection {
    topics: HashMap<String, Topic>,
}

impl TopicSelection {
    pub fn mock_topic_selection<T>(topics: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        let mut topics_map: HashMap<String, Topic> = HashMap::new();
        for topic in topics.into_iter() {
            topics_map.insert(topic, Topic { topic });
        }
        TopicSelection { topics: topics_map }
    }
}

pub struct Topic {
    pub topic: String,
}
```