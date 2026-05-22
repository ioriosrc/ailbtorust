```rust
use std::collections::HashMap;

pub struct MuiTypography {
    variant_mapping: HashMap<&'static str, &'static str>,
}

impl MuiTypography {
    pub fn new() -> Self {
        let mut mapping = HashMap::new();
        mapping.insert("h1", "div");
        mapping.insert("h2", "div");
        mapping.insert("h3", "div");
        mapping.insert("h4", "div");
        mapping.insert("h5", "div");
        mapping.insert("h6", "div");
        mapping.insert("subtitle1", "div");
        mapping.insert("subtitle2", "div");
        mapping.insert("body1", "div");
        mapping.insert("body2", "div");
        mapping.insert("inherit", "div");

        Self { variant_mapping }
    }
}
```