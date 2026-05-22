```rust
use std::time::{Duration, Instant};

/// Returns a promise resolved after `ms` milliseconds
pub async fn delay(ms: u32) -> Result<(), Box<dyn std::error::Error>> {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    Ok(())
}
```