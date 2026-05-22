```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GaugeConfig {
    // Define the structure of your GaugeConfig here
}

pub fn settings_action_reducer(
    prev_config: &GaugeConfig,
    action: SettingsActionReducerProps,
) -> Result<GaugeConfig, String> {
    let { action: settings_tree_action, payload } = action;

    match settings_tree_action {
        "perform-node-action" => Err("Unhandled node action: {}".to_string()),
        "update" if payload.path[0] == "general" => {
            let mut config_clone = prev_config.clone();
            for (path_key, path_value) in payload.path.iter().enumerate() {
                match path_key {
                    0 => {
                        let key: &str = match *path_value {
                            String::from("value") => "value",
                            _ => return Err(format!("Unexpected payload.path[1]: {}", path_value)),
                        };
                        config_clone.general.value = path_value.clone();
                    },
                    _ => return Err(format!("Unexpected path length: {}", path_key)),
                }
            }
            Ok(config_clone)
        },
        _ => Err("Unsupported action or payload".to_string()),
    }
}
```