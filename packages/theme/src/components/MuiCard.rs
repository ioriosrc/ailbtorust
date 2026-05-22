```rust
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MuiCardProps {
    variant: Option<String>,
    square: bool,
}

impl Default for MuiCardProps {
    fn default() -> Self {
        Self {
            variant: Some("outlined".to_string()),
            square: false,
        }
    }
}
```