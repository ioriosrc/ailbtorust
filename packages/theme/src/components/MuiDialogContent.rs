```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MuiDialogContentOverrideComponentReturn {
    pub style_overrides: Option<HashMap<String, HashMap<String, String>>>,
}
```