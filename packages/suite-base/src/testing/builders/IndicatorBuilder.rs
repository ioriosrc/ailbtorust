```rust
use std::collections::HashMap;

#[derive(Default)]
pub struct Style {
    pub bulb: Option<String>,
    pub background: Option<String>,
}

#[derive(Default)]
pub struct Operator {
    pub equal_to: bool,
    pub less_than: bool,
    pub less_than_or_equal_to: bool,
    pub greater_than: bool,
    pub greater_than_or_equal_to: bool,
}

#[derive(Default)]
pub struct Rule {
    color: Option<String>,
    label: Option<String>,
    operator: Operator,
    raw_value: Option<String>,
}

#[derive(Default)]
pub struct Rules(Vec<Rule>);

impl From<Vec<Rule>> for Rules {
    fn from(rules: Vec<Rule>) -> Self {
        Rules(rules)
    }
}

#[derive(Default)]
pub struct Config {
    fallback_color: Option<String>,
    fallback_label: Option<String>,
    path: Option<String>,
    rules: Rules,
    style: Style,
}

impl From<HashMap<&str, String>> for Config {
    fn from(mut map: HashMap<&str, String>) -> Self {
        let mut config = Config::default();

        if let Some(s) = map.remove("fallback_color") {
            config.fallback_color = Some(s);
        }

        if let Some(s) = map.remove("fallback_label") {
            config.fallback_label = Some(s);
        }

        if let Some(s) = map.remove("path") {
            config.path = Some(s);
        }

        if let Some(rules_str) = map.remove("rules") {
            let rules: Vec<Rule> = serde_json::from_str(&rules_str).unwrap();
            config.rules = Rules::from(rules);
        }

        if let Some(style_str) = map.remove("style") {
            let style: Style = serde_json::from_str(&style_str).unwrap();
            config.style = style;
        }

        config
    }
}
```