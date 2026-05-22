```rust
use std::collections::HashMap;

fn get_topic_to_schema_name_map(state: &MessagePipelineContext) -> HashMap<String, Option<&str>> {
    let mut result: HashMap<String, Option<&str>> = HashMap::new();

    for topic in state.sorted_topics.iter() {
        result.insert(topic.name.to_string(), topic.schema_name);
    }

    result
}
```