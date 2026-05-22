```rust
use serde_json::{Value, Map};

fn build_settings_tree(
    config: Option<&Map<String, Value>>,
    extension_settings: &Map<String, Map<String, Map<String, Value>>>,
    message_pipeline_state: impl Fn() -> Vec<serde_json::Value>,
    panel_type: String,
    settings_tree: Option<&Map<String, Map<String, Value>>>,
) -> Option<Map<String, Map<String, Value>>> {
    if panel_type.is_empty() || settings_tree.is_none() {
        return None;
    }

    let topic_to_schema_name_map = get_topic_to_schema_name_map(&message_pipeline_state());
    let topics = settings_tree?.get("topics")
        .and_then(|topics| topics.as_object())
        .map(|topics| topics.keys().cloned().collect::<Vec<&str>>());

    if let Some(topics) = topics {
        let topics_config = config
            .as_ref()
            .and_then(|config| config.get("topics"))
            .and_then(|topics_config| topics_config.as_object());

        let topics_settings: Map<String, Value> = topics.iter().map(|topic| {
            let schema_name = topic_to_schema_name_map.get(topic).unwrap();
            extension_settings
                .get(&panel_type)
                .and_then(|schema_settings| schema_settings.get(schema_name))
                .and_then(|settings| settings(topics_config.unwrap()[topic].clone()))
        }).collect();

        return Some(settings_tree?.merge_with(&topics_settings));
    }

    None
}
```