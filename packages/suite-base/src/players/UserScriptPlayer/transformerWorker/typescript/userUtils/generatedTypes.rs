```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageTypeByTopic {}

#[derive(Serialize, Deserialize)]
pub struct MessageTypeBySchemaName {}
```