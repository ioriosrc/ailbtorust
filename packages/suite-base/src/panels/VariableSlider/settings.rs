```rust
use crate::VariableSliderConfig;
use crate::{SaveConfig, SettingsTreeAction, SettingsTreeNode};
use im_rc::HashMap;
use std::collections::VecDeque;

pub fn build_settings_tree(config: VariableSliderConfig) -> HashMap<String, SettingsTreeNode> {
    let mut tree = HashMap::new();
    tree.insert(
        "general".to_string(),
        SettingsTreeNode {
            label: "General",
            fields: [
                ("min", "number"),
                ("max", "number"),
                ("step", "number"),
                ("globalVariableName", "string"),
            ]
            .into_iter()
            .map(|(label, input)| (
                label.to_string(),
                SettingsTreeNode {
                    label,
                    fields: vec![(
                        label.to_string(),
                        SettingsTreeNode::new(input, config.slider_props.min),
                    )],
                },
            ))
            .collect::<Vec<_>>(),
        },
    );
    tree
}

pub fn use_variable_slider_settings(
    config: VariableSliderConfig,
    save_config: SaveConfig<VariableSliderConfig>,
): () {
    let action_handler = move |action: SettingsTreeAction| {
        if action.action != "update" {
            return;
        }

        save_config(move |draft| {
            let path = action.payload.path.split_whitespace().skip(1);
            match (path.next(), path.next()) {
                (_, None) => {
                    let field_name = path.collect::<Vec<_>>()[0];
                    if ["min", "max"].contains(&field_name[..]) {
                        draft.slider_props.min = action.payload.value;
                    } else if field_name == "step" && action.payload.input == "number" {
                        draft.slider_props.step = action.payload.value;
                    }
                }
                (Some(field_name), Some(_)) => {
                    let value = action.payload.value.parse::<f64>().unwrap_or(0.0);
                    draft.slider_props.insert(field_name.to_string(), value);
                },
            }
        });
    };

    use_effect(() => {
        update_panel_settings_tree(action_handler, build_settings_tree(config));
    }, [action_handler, config]);
}
```