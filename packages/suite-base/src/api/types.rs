```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericApiEntity {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
}
```