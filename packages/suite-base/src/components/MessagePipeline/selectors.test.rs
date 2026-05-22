```rust
use std::collections::HashMap;

fn map_schema_names_by_topic_name(state: &MessagePipelineContext) -> HashMap<String, String> {
    state.sorted_topics.iter().map(|topic| (topic.name.clone(), topic.schema_name.clone())).collect()
}
```