```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "debug" => LogLevel::DEBUG,
            "info" => LogLevel::INFO,
            "warn" => LogLevel::WARN,
            "error" => LogLevel::ERROR,
            "fatal" => LogLevel::FATAL,
            _ => panic!("Invalid log level: {}", s),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Topic {
    name: String,
}

fn build_settings_tree(
    topic_to_render: &str,
    min_log_level: LogLevel,
    name_filter_enabled: bool,
    available_topics: Vec<Topic>,
    available_names: Vec<&str>,
) -> serde_json::Value {
    let mut tree = serde_json::json!({
        "general": {
            "fields": {
                "topicToRender": {
                    "input": "select",
                    "label": "topic",
                    "value": topic_to_render,
                    "error": None,
                    "options": available_topics
                        .iter()
                        .map(|topic| serde_json::json!({
                            "label": &topic.name,
                            "value": topic.name.to_string(),
                        }))
                        .collect::<Vec<serde_json::Value>>(),
                },
                "minLogLevel": {
                    "input": "select",
                    "label": "minLogLevel",
                    "value": min_log_level.to_string().to_lowercase(),
                    "error": None,
                    "options": [
                        serde_json::json!({
                            "label": ">= DEBUG",
                            "value": LogLevel::DEBUG.to_string(),
                        }),
                        serde_json::json!({
                            "label": ">= INFO",
                            "value": LogLevel::INFO.to_string(),
                        }),
                        serde_json::json!({
                            "label": ">= WARN",
                            "value": LogLevel::WARN.to_string(),
                        }),
                        serde_json::json!({
                            "label": ">= ERROR",
                            "value": LogLevel::ERROR.to_string(),
                        }),
                        serde_json::json!({
                            "label": ">= FATAL",
                            "value": LogLevel::FATAL.to_string(),
                        }),
                    ],
                },
            },
        },
        "nameFilter": {
            "enableVisibilityFilter": name_filter_enabled,
            "children": available_names
                .iter()
                .map(|&name| serde_json::json!({
                    "label": name,
                    "visible": true, // Assuming default visibility is true for all names
                }))
                .collect::<serde_json::Value>(),
            "label": "nameFilter",
            "actions": [
                { "id": "show-all", "type": "action", "label": "showAll" },
                { "id": "hide-all", "type": "action", "label": "hideAll" },
            ],
        },
    });

    tree
}
```