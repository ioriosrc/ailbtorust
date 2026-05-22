```rust
pub fn get_general() -> serde_json::Value {
    let general = json!({
        "foxglove": "Foxglove",
        "learnMore": "Learn more",
        "noDefaultLayoutParameter": format!(
            "The layout '{}' specified in the app parameters does not exist.",
            "{{layoutName}}"
        ),
    });

    serde_json::to_value(general).unwrap()
}
```