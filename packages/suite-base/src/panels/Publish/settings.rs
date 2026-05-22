```rust
use crate::config::{PublishConfig, SaveConfig};
use crate::datatypes::{ImmutableRosDatatypes, RosDatatypes};
use crate::messages::{build_sample_message, MessageSchemaName, Topic};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use immer::{Draft, ImmerContext};
use lodash::Function as LodashFunction;
use serde_json::json;

pub fn datatype_error(schema_names: &Vec<MessageSchemaName>, datatype: Option<&str>) -> Option<String> {
  if datatype.is_none() {
    return Some("Message schema cannot be empty".to_string());
  }
  if !schema_names.contains(datatype) {
    return Some("Schema name not found".to_string());
  }
  None
}

pub fn topic_error(topic_name: &Option<&str>) -> Option<String> {
  if topic_name.is_none() {
    return Some("Topic cannot be empty".to_string());
  }
  None
}

fn build_settings_tree(
  config: PublishConfig,
  schema_names: &Vec<MessageSchemaName>,
  topics: &Vec<Topic>,
) -> Vec<(String, Vec<SettingsTreeNodes>)> {
  vec![
    (
      "general",
      vec![
        (format!("topicName", topic_name), vec![build_input_field("Topic", "autocomplete", "", topic_names.map(|t| t.name))]),
        (format!("datatype"), vec![build_input_field("Message schema", "autocomplete", "", schema_names.map(|s| s.to_string()))]),
        (
          format!("advancedView"),
          vec![build_input_field("Editing mode", "boolean", config.advanced_view)],
        ),
      ],
    ),
    (
      "button",
      vec![
        (format!("buttonText"), vec![build_input_field("Title", "string", config.button_text)]),
        (format!("buttonTooltip"), vec![build_input_field("Tooltip", "string", config.button_tooltip)]),
        (
          format!("buttonColor"),
          vec![build_input_field("Color", "rgb", config.button_color.to_string())],
        ),
      ],
    ),
  ]
}

fn get_sample_message(datatypes: &ImmutableRosDatatypes, datatype: Option<&str>) -> Option<String> {
  if datatype.is_none() {
    return None;
  }
  let sample_message = build_sample_message(datatypes, datatype);
  if sample_message.is_some() {
    Some(json!(sample_message).to_string())
  } else {
    Some("{\"message\": \"\"}".to_string())
  }
}

pub fn use_publish_panel_settings(
  config: PublishConfig,
  save_config: SaveConfig<PublishConfig>,
  topics: Vec<Topic>,
  datatypes: ImmutableRosDatatypes,
) -> () {
  let update_context = ImmerContext::new();
  let (state, dispatch) = use_context(&update_context);

  let schema_names = useMemo(() => {
    datatypes.iter().map(|(name, _)| name.to_string()).collect()
  }, [datatypes]);

  let action_handler = useCallback(
    move |action| {
      if action.action != "update" {
        return;
      }
      let draft: Draft<PublishConfig> = state.clone();
      let { path, value, input } = action.payload;

      dispatch(
        produce(&mut draft, |draft| {
          if input == "autocomplete" {
            if let Ok(topic_name) = value.to_string().parse::<String>() {
              let topic_schema_name = topics.find(|t| t.name.eq(&topic_name)).map(|t| t.schema_name);
              let sample_message = get_sample_message(datatypes, topic_schema_name);

              draft.topic_name = topic_name;
              if let Some(sample_message) = sample_message {
                draft.datatype = topic_schema_name;
                draft.value = json!(sample_message).to_string();
              }
            } else {
              draft.datatype = None;
              draft.value = "{\"message\": \"\"}".to_string();
            }
          } else {
            _.set(&mut draft, path.iter().map(|s| s.to_string()).collect::<Vec<&str>>(), value);
          }
        }),
      );
    },
    [datatypes, save_config, topics],
  );

  useEffect(() => {
    dispatch(
      produce(state.clone(), |draft| {
        build_settings_tree(draft, &schema_names, &topics)
          .into_iter()
          .for_each(|(path, nodes)| update_context.dispatch(update_panel_settings_tree(path, nodes)));
      }),
    );
  }, [action_handler, config, schema_names, topics]);
}
```

Note: The provided code snippet is incomplete and lacks necessary imports and error handling. You may need to add additional functionality such as form validation, error messages, and more depending on your requirements.