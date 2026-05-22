```rust
use serde_json::{Value, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MuiInputBase {
    style_overrides: Option<Map<String, String>>,
}

impl MuiInputBase {
    pub fn new() -> Self {
        MuiInputBase { style_overrides: None }
    }

    pub fn set_style_overrides(&mut self, overrides: Map<String, String>) {
        self.style_overrides = Some(overrides);
    }
}
```