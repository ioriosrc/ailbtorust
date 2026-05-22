```rust
use std::collections::{HashMap, VecDeque};

pub struct Initialization {
    pub start: RosTime,
    pub end: RosTime,
    pub datatypes: HashMap<String, Type>,
    pub publishers_by_topic: HashMap<String, Publisher>,
    pub topic_stats: HashMap<String, TopicStats>,
    pub alerts: Vec<Alert>,
    pub topics: Vec<Topic>,
    pub metadata: Vec<Metadata>,
    pub profile: String,
}

pub struct Metadata {
    pub name: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug)]
pub struct RosTime;

impl RosTime {
    pub fn time() -> Self {
        // Implement RosTime logic here
    }
}

pub struct Type;

pub struct Publisher;

pub struct TopicStats;

pub struct Alert;

pub struct Topic;

pub type BasicBuilder<T> = impl Fn(usize) -> T;

pub fn defaults<T>(props: Option<T>, default_value: T) -> T {
    props.unwrap_or(default_value)
}

pub fn multiple<T, F>(builder: F, count: usize) -> Vec<T> {
    (0..count).map(|_| builder(count)).collect()
}
```