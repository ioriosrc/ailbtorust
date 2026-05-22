```rust
pub const LOGS: &str = include_str!("log.json");

#[derive(Deserialize)]
struct Log {
    name: String,
    level: i32,
}

fn main() {
    // Your Rust code here
}
```