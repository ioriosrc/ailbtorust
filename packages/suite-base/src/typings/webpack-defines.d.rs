```rust
use std::env;

#[cfg(target_os = "linux")]
pub fn main() {
    println!("Running on Linux");
}

#[cfg(windows)]
pub fn main() {
    println!("Running on Windows");
}

fn main() {
    let api_url = env::var("API_URL").ok();
    let lichtblick_suite_version = env::var("LICHTBLICK_SUITE_VERSION").ok();
    let dev_workspace = env::var("DEV_WORKSPACE").ok();

    // Use the values as needed
}
```