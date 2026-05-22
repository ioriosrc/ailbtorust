```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Config {
    topic_path: String,
}

#[derive(Serialize, Deserialize)]
struct TableDataItem {
    value: Vec<u8>,
}

fn main() {
    // Your Rust code here
}
```