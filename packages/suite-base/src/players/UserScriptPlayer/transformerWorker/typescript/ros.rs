```rust
use std::time::{Duration, Instant};

pub struct Duration {
    pub sec: u64,
    pub nsec: i32,
}

pub struct Time {
    pub sec: u64,
    pub nsec: i32,
}

// Once a data source Messages will be populated with interfaces matching the data source messages.
pub mod Messages {}

// Once a data source TopicsToMessageDefinition will be populated with topic names to message interfaces.
pub mod TopicsToMessageDefinition {};

/**
 * To correctly type your inputs, you use this type to refer to specific
 * input topics, e.g. 'Input<"/your_input_topic">'. If you have
 * multiple input topics, use a union type, e.g.
 * 'Input<"/your_input_topic_1"> |
 * Input<"/your_input_topic_2">'.
 *
 * These types are dynamically generated from the bag(s) currently in your
 * Lichtblick session, so if a datatype changes, your User Script
 * may not compile on the newly formatted bag.
 */
pub type Input<T> = (String, Time, TopicsToMessageDefinition::MessageTypes[T]);
```