```rust
use std::collections::HashMap;

fn migrate_legacy_to_new_image_panels(layout: serde_json::Value, config_by_id: HashMap<&str, serde_json::Value>, global_variables: HashMap<String, serde_json::Value>, user_nodes: HashMap<String, serde_json::Value>, playback_config: serde_json::Value) -> serde_json::Value {
    let mut new_layout = serde_json::json!({
        "direction": layout["direction"],
        "first": migrate_legacy_to_new_image_panels(layout["first"], config_by_id.clone(), global_variables.clone(), user_nodes.clone(), playback_config),
        "second": migrate_legacy_to_new_image_panels(layout["second"], config_by_id, global_variables, user_nodes, playback_config)
    });

    if let serde_json::Value::Object(ref mut obj) = new_layout["first"] {
        obj.insert("layout".to_string(), layout["first"]["layout"].clone());
        obj.insert("configById".to_string(), config_by_id.clone());
        obj.insert("globalVariables".to_string(), global_variables.clone());
        obj.insert("userNodes".to_string(), user_nodes.clone());
        obj.insert("playbackConfig".to_string(), playback_config.clone());
    } else if let serde_json::Value::Object(ref mut obj) = new_layout["second"] {
        obj.insert("layout".to_string(), layout["second"]["layout"].clone());
        obj.insert("configById".to_string(), config_by_id.clone());
        obj.insert("globalVariables".to_string(), global_variables.clone());
        obj.insert("userNodes".to_string(), user_nodes.clone());
        obj.insert("playbackConfig".to_string(), playback_config.clone());
    }

    new_layout
}

fn main() {
    let layout = serde_json::json!({
        "direction": "row",
        "first": { "direction": "row", "first": "XXX!a", "second": "ImageViewPanel!a" },
        "second": "XXX!b"
    });

    let config_by_id = HashMap::from([
        ("ImageViewPanel!a".to_string(), serde_json::json!({
            "cameraTopic": "/cam/image_rect_compressed",
            "enabledMarkerTopics": ["/cam/annotations", "/cam/lidar"],
            "transformMarkers": false,
            "synchronize": true,
            "mode": "fit",
            "pan": {
                "x": 0,
                "y": 0
            },
            "rotation": 90,
            "zoom": 1,
            "flipHorizontal": true,
            "flipVertical": true,
            "minValue": 2,
            "maxValue": 6
        })),
        ("XXX!a".to_string(), serde_json::json!({
            "foo": "bar"
        })),
        ("XXX!b".to_string(), serde_json::json!({
            "foo": "baz"
        }))
    ]);

    let global_variables = HashMap::new();

    let user_nodes = HashMap::new();

    let playback_config = serde_json::json!({
        "speed": 1
    });

    let migrated_layout = migrate_legacy_to_new_image_panels(layout, config_by_id, global_variables, user_nodes, playback_config);

    println!("{}", serde_json::to_string_pretty(&migrated_layout).unwrap());
}
```