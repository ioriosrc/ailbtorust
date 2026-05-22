```rust
// Cargo.toml
[package]
name = "your_project_name"
version = "1.0.0"
edition = "2021"

[dependencies]
ts-rust = { version = "0.6", features = ["no_std"] }
```

```rust
fn main() {
    println!("Hello, Rust!");
}
```