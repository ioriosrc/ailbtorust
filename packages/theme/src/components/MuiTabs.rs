```rust
use std::collections::HashMap;

pub struct MuiTabs {
    style_overrides: HashMap<String, String>,
}

impl MuiTabs {
    pub fn new() -> Self {
        let mut style_overrides = HashMap::new();
        style_overrides.insert(String::from(".MuiTabs-indicator"), String::from(
            "left: 0; right: 'auto';",
        ));
        MuiTabs { style_overrides }
    }

    pub fn get_style_overrides(&self) -> &HashMap<String, String> {
        &self.style_overrides
    }
}
```