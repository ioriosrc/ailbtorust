```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MuiListItemButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_ripple: Option<bool>,
}

impl MuiListItemButton {
    pub fn new() -> Self {
        MuiListItemButton { disable_ripple: None }
    }

    pub fn set_disable_ripple(&mut self, value: bool) {
        self.disable_ripple = Some(value);
    }
}
```