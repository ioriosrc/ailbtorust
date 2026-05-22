```rust
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalStorageSaveState {
    pub global_level: Option<String>,
    pub disabled_channels: Vec<String>,
}
```