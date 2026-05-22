```rust
use crate::settings_tree_field::{SettingsTreeField, SettingsTreeFieldNumber, SettingsTreeFieldSelectString};
use crate::default_settings_tree_node::DEFAULT_SETTINGS_TREE_NODE;
use crate::diagnostics_builder::summary_config;
use crate::basic_builder::{strings, boolean, number};

fn build_settings_tree() -> SettingsTreeNodes {
    // Given
    let topics = strings().collect::<Vec<String>>();
    let config = summary_config(vec![boolean(), number()]);

    // When
    let result: SettingsTreeNodes = build_settings_tree_with_topics_and_config(topics, config);

    // Then
    let general_node = &result.general;
    assert_eq!(general_node.label, DEFAULT_SETTINGS_TREE_NODE.general!.label);
    let fields_result = general_node.fields.as_ref().unwrap();
    assert_eq!(
        fields_result.topic_to_render,
        Some({
            let mut options = vec![{
                label: topics[0],
                value: &topics[0],
            }];
            options.push({
                label: topics[1],
                value: &topics[1],
            });
            settings_tree_field::SettingsTreeFieldSelectString {
                input: "select",
                label: fields_result.topic_to_render.label,
                value: topics[0],
                error: None,
                options,
            }
        })
    );
    assert_eq!(
        fields_result.sort_by_level,
        Some({
            let settings_tree_field = settings_tree_field::SettingsTreeFieldNumber {
                input: "boolean",
                label: fields_result.sort_by_level.label,
                help: fields_result.sort_by_level.help,
                min: fields_result.sort_by_level.min.unwrap(),
                placeholder: fields_result.sort_by_level.placeholder.unwrap(),
                precision: fields_result.sort_by_level.precision.unwrap(),
                step: fields_result.sort_by_level.step.unwrap(),
            };
            settings_tree_field::SettingsTreeFieldSelectString {
                input: "select",
                label: fields_result.sort_by_level.label,
                value: config.sort_by_level,
                error: None,
                options: vec![settings_tree_field],
            }
        })
    );
    assert_eq!(
        fields_result.seconds_until_stale,
        Some({
            let settings_tree_field = settings_tree_field::SettingsTreeFieldNumber {
                input: "number",
                label: fields_result.seconds_until_stale.label,
                help: fields_result.seconds_until_stale.help.unwrap(),
                min: (fields_result.seconds_until_stale.min.unwrap() as i32),
                placeholder: fields_result.seconds_until_stale.placeholder.unwrap(),
                precision: fields_result.seconds_until_stale.precision.unwrap(),
                step: fields_result.seconds_until_stale.step.unwrap(),
            };
            settings_tree_field::SettingsTreeFieldSelectString {
                input: "select",
                label: fields_result.seconds_until_stale.label,
                value: config.seconds_until_stale,
                error: None,
                options: vec![settings_tree_field],
            }
        })
    );
}

fn build_settings_tree_with_topics_and_config(topics: Vec<String>, config: summary_config) -> SettingsTreeNodes {
    let general_node = settings_tree_field::SettingsTreeFieldGeneral::new(
        String::from(DEFAULT_SETTINGS_TREE_NODE.general.label),
        vec![
            settings_tree_field::SettingsTreeField {
                label: "topicToRender".to_string(),
                field: &settings_tree_field::SettingsTreeFieldSelectString::default_config(topics, topics[0]),
            },
            settings_tree_field::SettingsTreeField {
                label: "sortByLevel".to_string(),
                field: &settings_tree_field::SettingsTreeFieldBoolean::default_config(config.sort_by_level),
            },
            settings_tree_field::SettingsTreeField {
                label: "secondsUntilStale".to_string(),
                field: &settings_tree_field::SettingsTreeFieldNumber::default_config(config.seconds_until_stale),
            },
        ],
    );

    SettingsTreeNodes::new(vec![general_node])
}
```