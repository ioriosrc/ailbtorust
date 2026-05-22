```rust
use std::collections::HashMap;

// Define a struct to hold the configuration for the menu
struct MuiMenuConfig {
    transition_component: String,
    paper_style_overrides: HashMap<String, Box<dyn Fn(&'static str) -> &str>>,
    list_style_overrides: HashMap<String, Box<dyn Fn(&'static str) -> &str>>,
}

fn main() {
    // Create a new instance of MuiMenuConfig
    let config = MuiMenuConfig {
        transition_component: "Fade".to_string(),
        paper_style_overrides: [
            ("paper".to_string(), Box::new(|key| format!("border-radius: {}px;", theme.shape.border_radius))),
            (
                "list".to_string(),
                Box::new(|key| {
                    let theme = &theme;
                    format!(
                        "{0} {{ &.body1 {{ color: {1} !important; }} }",
                        key, theme.typography.body1.color
                    )
                }),
            ),
        ]
        .iter()
        .cloned()
        .collect(),
        list_subheader_style_overrides: HashMap::new(),
    };

    // Print the configuration to verify it
    println!("Transition Component: {}", config.transition_component);
    for (key, value) in &config.paper_style_overrides {
        println!("{0}: {1}", key, value);
    }
    for (key, value) in &config.list_style_overrides {
        println!("{0}: {1}", key, value);
    }
}
```

This Rust code defines a struct `MuiMenuConfig` to encapsulate the configuration details for the MUI Menu component. It then creates an instance of this struct and prints out the configuration properties to verify that they match the TypeScript/React code provided. The style overrides are defined as closures that use string formatting to generate CSS styles dynamically based on the theme's properties.