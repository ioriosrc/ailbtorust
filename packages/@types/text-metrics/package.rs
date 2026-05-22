```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TextMetrics {
    pub length: usize,
    pub words: usize,
    pub sentences: usize,
}
```