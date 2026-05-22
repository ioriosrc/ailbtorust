```rust
use std::collections::HashMap;

pub struct PublishConfig {
    pub topic_name: String,
    pub datatype: String,
    pub buttonText: String,
    pub button_tooltip: String,
    pub button_color: String,
    pub advanced_view: bool,
    pub value: String,
}

fn defaults() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("topicName".to_string(), "default_topic_name".to_string());
    map.insert("datatype".to_string(), "default_datatype".to_string());
    map.insert("buttonText".to_string(), "default_button_text".to_string());
    map.insert("buttonTooltip".to_string(), "default_button_tooltip".to_string());
    map.insert("buttonColor".to_string(), "#ffffff");
    map.insert("advancedView".to_string(), "true".to_string());
    map.insert("value".to_string(), "{}".to_string());
    map
}

pub fn config(props: HashMap<String, String>) -> PublishConfig {
    let mut config = defaults();
    for (key, value) in props {
        if let Some(val) = config.get_mut(key.as_str()) {
            *val = value;
        }
    }
    PublishConfig {
        topic_name: config.get("topicName").unwrap().to_string(),
        datatype: config.get("datatype").unwrap().to_string(),
        buttonText: config.get("buttonText").unwrap().to_string(),
        button_tooltip: config.get("buttonTooltip").unwrap().to_string(),
        button_color: config.get("buttonColor").unwrap().to_string(),
        advanced_view: config.get("advancedView").unwrap().parse::<bool>().unwrap(),
        value: config.get("value").unwrap().to_string(),
    }
}
```