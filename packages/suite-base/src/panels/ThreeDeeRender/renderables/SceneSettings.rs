```rust
use std::collections::HashMap;

// Define the SceneSettings struct with fields and methods to interact with the scene configuration
pub struct SceneSettings {
    config: HashMap<String, serde_json::Value>,
    renderer: Box<dyn IRenderer>,
}

impl SceneSettings {
    pub fn new(renderer: Box<dyn IRenderer>, name: &str) -> Self {
        Self {
            config: Self::default_config(),
            renderer,
        }
    }

    // Default configuration for the scene settings
    fn default_config() -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert("enableStats".to_string(), serde_json::Value::Bool(true));
        config.insert("debugPicking".to_string(), serde_json::Value::Bool(false));
        config.insert("backgroundColor".to_string(), serde_json::Value::String("#ffffff".to_string()));
        config.insert(
            "labelScaleFactor".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(1.0).unwrap()),
        );
        config.insert(
            "ignoreColladaUpAxis".to_string(),
            serde_json::Value::Bool(false),
        );
        config.insert("meshUpAxis".to_string(), serde_json::Value::String("y_up".to_string()));
        config
    }

    // Dispose method to clean up resources and reset the settings
    pub fn dispose(&mut self) {
        // Cleanup code here if needed
    }

    // Settings nodes method to define the structure of the scene settings in a tree-like format
    pub fn settings_nodes(&self) -> Vec<SettingsNode> {
        vec![
            SettingsNode {
                label: "Scene",
                actions: vec![Action::new("reset-scene", "Reset Scene")],
                fields: HashMap::from([
                    ("enableStats".to_string(), Field::boolean(self.config.get("enableStats").unwrap_or(&false))),
                    ("debugPicking".to_string(), Field::boolean(self.config.get("debugPicking").unwrap_or(&false))),
                    (
                        "backgroundColor".to_string(),
                        Field::rgb(self.config.get("backgroundColor").unwrap_or(&"#ffffff")),
                    ),
                    (
                        "labelScaleFactor".to_string(),
                        Field::number(
                            self.config.get("labelScaleFactor")
                                .unwrap_or(&serde_json::Number::from_f64(1.0))
                                .as_f64()
                                .unwrap() as i32,
                        ),
                    ),
                    (
                        "ignoreColladaUpAxis".to_string(),
                        Field::boolean(self.config.get("ignoreColladaUpAxis").unwrap_or(&false)),
                    ),
                    (
                        "meshUpAxis".to_string(),
                        Field::select(
                            self.config.get("meshUpAxis")
                                .unwrap_or(&"y_up")
                                .as_str()
                                .unwrap(),
                            vec![("y_up", "Y Up"), ("z_up", "Z Up")],
                        ),
                    ),
                ]),
            },
        ]
    }

    // Handle settings action method to update the scene configuration based on user input
    pub fn handle_settings_action(&mut self, action: &SettingsAction) {
        if action.action == "perform-node-action" && action.payload.id == "reset-scene" {
            self.renderer.update_config(|draft| draft.remove("scene"));
            self.update_settings_tree();
            return;
        }

        if action.action != "update" || action.payload.path.is_empty() {
            return;
        }

        let path = &action.payload.path[0];
        let value = &action.payload.value;

        if path == "enableStats" {
            self.config.insert("enableStats".to_string(), serde_json::Value::Bool(value.to_bool().unwrap_or(true)));
            self.update_settings_tree();
            return;
        }
        if path == "debugPicking" {
            self.renderer.debug_picking = value.to_bool().unwrap_or(false);
            self.update_settings_tree();
            return;
        }
        if path == "backgroundColor" {
            self.config.insert(
                "backgroundColor".to_string(),
                serde_json::Value::String(value.to_string()),
            );
            self.update_color_scheme(self.renderer.color_scheme, &value.to_string());
        } else if path == "labelScaleFactor" {
            let label_scale_factor = value.as_f64().unwrap_or(1.0) as i32;
            self.config.insert(
                "labelScaleFactor".to_string(),
                serde_json::Value::Number(label_scale_factor.into()),
            );
            self.renderer.label_pool.set_scale_factor(label_scale_factor);
        } else if path == "ignoreColladaUpAxis" {
            let ignore_collada_up_axis = value.to_bool().unwrap_or(false);
            self.config.insert(
                "ignoreColladaUpAxis".to_string(),
                serde_json::Value::Bool(ignore_collada_up_axis),
            );
            self.update_settings_tree();
        } else if path == "meshUpAxis" {
            let mesh_up_axis = value.as_str().unwrap_or("y_up").into();
            self.config.insert(
                "meshUpAxis".to_string(),
                serde_json::Value::String(mesh_up_axis),
            );
            self.update_settings_tree();
        }
    }
}
```