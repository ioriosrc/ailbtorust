```rust
use crate::settings_tree_field::{SettingsTreeField, SettingsTreeFieldBoolean, SettingsTreeFieldNumber, SettingsTreeFieldSelectString};
use crate::settings_tree_nodes::DEFAULT_SETTINGS_TREE_NODE;
use crate::types::DiagnosticSummaryConfig;

#[derive(Debug, Clone)]
pub struct BuildSettingsTreeProps {
    config: DiagnosticSummaryConfig,
    topic_to_render: String,
    available_topics: Vec<String>,
}

pub fn build_settings_tree(props: BuildSettingsTreeProps) -> SettingsTreeNodes {
    let topic_options = props.available_topics.iter().map(|&topic| {
        (topic.clone(), topic.clone())
    }).collect::<Vec<_>>();

    let topic_is_available = props.available_topics.contains(&props.topic_to_render);

    if !topic_is_available {
        topic_options.insert(0, ("", props.topic_to_render));
    }

    let topic_error = if topic_is_available {
        None
    } else {
        Some(format!("Topic {}", props.topic_to_render))
    };

    SettingsTreeNodes {
        general: DEFAULT_SETTINGS_TREE_NODE.general.clone(),
        fields: SettingsTreeField::new("topicToRender", props.topic_to_render)
            .with_option(&topic_options)
            .with_error(topic_error)
            .as_select_string(),
        fields: SettingsTreeField::new("sortByLevel", props.config.sortByLevel)
            .with_boolean()
            .as_boolean(),
        fields: SettingsTreeField::new("secondsUntilStale", props.config.secondsUntilStale)
            .with_number()
            .as_number(),
    }
}
```