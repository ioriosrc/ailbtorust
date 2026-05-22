```rust
use std::collections::HashMap;

pub fn use_chart_data(rawValue: &[f32], config: &HashMap<&str, String>) -> Vec<HashMap<String, f32>> {
    if raw_value.is_empty() {
        return vec![];
    }

    let total = raw_value.iter().sum::<f32>();

    raw_value
        .iter()
        .enumerate()
        .map(|(i, value)| {
            let percentage = (value / total) * 100;
            let legend_key = format!("legend{}", i + 1);
            let raw_name = config.get(&legend_key).unwrap_or(&"Data {}", i + 1);
            let name = if !raw_name.trim().is_empty() {
                raw_name
            } else {
                "Data {}", i + 1
            };

            vec![HashMap::from([
                ("name".to_string(), name.to_string()),
                ("value".to_string(), percentage.to_string()),
                ("color".to_string(), format!("hsl({},{},{}%)", (i as f32 / raw_value.len() * 40 + 200), 20, 50 - i * 5)),
            ])]
        })
        .collect()
}
```