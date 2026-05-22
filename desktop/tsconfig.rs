```rust
// Cargo.toml
[package]
name = "my_project"
version = "1.0.0"
edition = "2021"

[dependencies]

[build-dependencies]
rustfmt = { version = "1", features = ["nightly"] }

[dev-dependencies]

[[test]]
build = "build.rs"
```