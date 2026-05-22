```rust
use anyhow::{Context, Result};
use serde_json::Value;

fn main() -> Result<()> {
    let config = serde_json::from_str(include_str!("../config.json"))?;
    // Your Rust code here
    Ok(())
}
```