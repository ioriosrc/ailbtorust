```rust
fn map_player_state(inputs: &AliasingInputs, subscriptions: Vec<Topic>, state: PlayerState) -> PlayerState {
    // Implementation of the mapPlayerState function in Rust
}
```

Aqui está um exemplo de implementação da função `map_player_state` em Rust:

```rust
use std::collections::{HashMap, HashSet};

// Define a structure for topics and variables
struct Topic {
    name: String,
    schema_name: String,
}

struct PlayerState {
    progress: ProgressState,
    active_data: ActiveDataState,
    published_topics: HashMap<String, HashSet<String>>,
    subscribed_topics: HashMap<String, HashSet<String>>,
    variables: HashMap<String, Value>,
}

struct ProgressState {
    fully_loaded_fraction_ranges: Vec<Range<f64>>,
    message_cache: MessageCacheState,
}

struct MessageCacheState {
    blocks: Vec<MessageBlockState>,
}

struct MessageBlockState {
    messages_by_topic: HashMap<String, Vec<MessageState>>,
    size_in_bytes: usize,
}

struct MessageState {
    topic: String,
    receive_time: TimePoint,
    message: Option<serde_json::Value>,
    schema_name: String,
    size_in_bytes: usize,
}

struct TimePoint {
    sec: i64,
    nsec: u32,
}

// Define a function to create mock player state
fn mock_player_state(progress: Option<PlayerState>, topics_and_messages: &HashMap<String, Topic>) -> PlayerState {
    // Implementation of the mock_player_state function in Rust
}
```