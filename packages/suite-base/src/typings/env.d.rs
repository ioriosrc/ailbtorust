```rust
use std::env;

fn main() {
    let dev_workspace = env::var("DEV_WORKSPACE").unwrap_or_default();
    println!("Dev workspace: {}", dev_workspace);
}
```