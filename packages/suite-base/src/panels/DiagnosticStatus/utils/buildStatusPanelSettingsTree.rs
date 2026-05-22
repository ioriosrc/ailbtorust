```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DiagnosticStatusConfig {
    numeric_precision: i32,
    seconds_until_stale: u64,
}

const DEFAULT_SECONDS_UNTIL_STALE: u64 = 60;

pub fn build_status_panel_settings_tree(
    config: DiagnosticStatusConfig,
    topic_to_render: &str,
    available_topics: &[&str],
) -> serde_json::Value {
    let mut tree = serde_json::json!({
        "general": {
            "label": "General",
            "fields": {
                "topicToRender": {
                    "label": "Topic",
                    "input": "select",
                    "value": topic_to_render,
                    "error": config.topic_is_available().to_string(),
                    "options": available_topics.iter().map(|&t| serde_json::json!({
                        "value": t.to_string(),
                        "label": t.to_string()
                    })).collect::<Vec<_>>(),
                },
                "numericPrecision": {
                    "label": "Numeric precision",
                    "input": "number",
                    "min": 0,
                    "max": 17,
                    "precision": 0,
                    "step": 1,
                    "placeholder": "auto",
                    "value": config.numeric_precision,
                },
                "secondsUntilStale": {
                    "label": "Stale timeout",
                    "help": "Number of seconds after which entries will be marked as stale if no new diagnostic message(s) have been received",
                    "input": "number",
                    "placeholder": format!("{} seconds", DEFAULT_SECONDS_UNTIL_STALE),
                    "min": 0,
                    "step": 1,
                    "precision": 0,
                    "value": config.seconds_until_stale,
                },
            }
        }
    });

    if !config.topic_is_available() {
        let topic_options = available_topics.iter().map(|&t| serde_json::json!({
            "value": t.to_string(),
            "label": t.to_string()
        })).collect::<Vec<_>>();

        tree["general"]["fields"]["topicToRender"]["options"] = topic_options;
    }

    tree
}
```