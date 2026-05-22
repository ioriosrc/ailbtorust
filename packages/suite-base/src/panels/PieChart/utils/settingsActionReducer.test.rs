```rust
use std::collections::HashMap;

fn settings_action_reducer(config: HashMap<String, String>, action: &SettingsTreeAction) -> HashMap<String, String> {
    match action.action {
        "update" => {
            if config.contains_key(&action.payload.path[0]) && config.get(&action.payload.path[0]).unwrap() == &"/general/path" {
                let mut new_config = config.clone();
                new_config.insert(action.payload.path[1].to_string(), action.payload.input.to_string());
                return new_config;
            }
            config
        },
        "perform-node-action" => {
            if action.payload.action == "test-action" && action.payload.path[0] == "path" {
                // Handle the specific action here
                println!("Handling test-action with path: {}", &action.payload.path);
            }
            config
        },
        _ => config,
    }
}
```