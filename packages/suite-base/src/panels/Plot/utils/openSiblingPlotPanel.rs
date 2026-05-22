```rust
use serde_json::{json, Value};
use std::collections::HashSet;

pub fn open_sibling_plot_panel(open_sibling_panel: OpenSiblingPanel, topic_name: String) {
    open_sibling_panel({
        panel_type: "Plot".to_string(),
        update_if_exists: true,
        sibling_config_creator: |config| {
            let mut paths = config.paths.clone();
            let plot_config = config.downcast_ref::<PlotConfig>().unwrap();

            paths.extend(plot_config.paths.iter().cloned());
            paths.push(Value::Object({
                topic_name.clone().into(),
                vec![("enabled".to_string(), Value::Bool(true))],
                vec![(
                    "timestampMethod".to_string(),
                    Value::String(String::from("receiveTime")),
                )],
            }));

            json!({
                "paths": paths.into_iter().collect::<Vec<_>>()
            })
        },
    });
}
```