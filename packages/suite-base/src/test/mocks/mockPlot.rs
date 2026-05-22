```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

// Define the necessary types from the Rust programming language

type RosDatatypes = HashMap<String, Vec<(String, String)>>;

struct MessageBlock {
    messages_by_topic: HashMap<String, Vec<MessageEvent>>;
    size_in_bytes: usize;
    need_topics: HashMap<String, Duration>;
}

struct Fixture {
    datatypes: RosDatatypes;
    topics: Vec<FixtureTopic>;
    active_data: ActiveData;
    frame: Frame;
    progress: Progress;
}

struct MessageEvent {
    topic: String;
    receive_time: Instant;
    message: serde_json::Value;
    schema_name: String;
    size_in_bytes: usize;
}

struct ActiveData {
    start_time: Duration;
    end_time: Duration;
    current_time: Instant;
    is_playing: bool;
    speed: f64;
}

struct Frame {
    topics: HashMap<String, Vec<MessageEvent>>;
}

struct Progress {
    message_cache: MessageCache;
}

// Define the necessary functions to implement the code

fn main() {
    // Initialize the message cache
    let mut message_cache = MessageCache {
        blocks: vec![
            ...[0.6..1.0].map(|seconds| MessageBlock {
                size_in_bytes: 0,
                messages_by_topic: { "/preloaded_topic": vec![get_preloaded_message(seconds)] },
                need_topics: HashMap::new(),
            }),
            EmptyBlock,
            EmptyBlock,
            EmptyBlock,
            EmptyBlock,
            ...[1.5..2.0].map(|seconds| MessageBlock {
                size_in_bytes: 0,
                messages_by_topic: { "/preloaded_topic": vec![get_preloaded_message(seconds)] },
                need_topics: HashMap::new(),
            }),
        ],
        startTime: Duration::from_secs(0.6),
    };

    // Initialize the active data
    let active_data = ActiveData {
        start_time: Duration::from_secs(202050),
        end_time: Duration::from_secs(2499997069),
        current_time: Instant::now(),
        is_playing: false,
        speed: 0.2,
    };

    // Initialize the frame
    let frame = Frame {
        topics: HashMap::new(),
    };

    // Initialize the progress
    let progress = Progress {
        message_cache,
    };

    // Implement other functions to populate the fixture and handle messages

    // Example usage of the fixture and its components
    println!("{:?}", active_data);
    println!("{:?}", frame);
}
```

Note: This is a simplified version of the Rust code provided. The actual implementation would involve more complex logic for handling topics, message serialization/deserialization, and more sophisticated data management.