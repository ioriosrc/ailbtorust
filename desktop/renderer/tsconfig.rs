```rust
// Cargo.toml

[package]
name = "your_project_name"
version = "0.1.0"
edition = "2023"

[dependencies]
core = { version = "1", features = ["panic_unwind"] }

[target.'cfg(target_arch = "wasm")']
features = [
  "wasm-bindgen",
  "js_sys",
  "web_worker"
]

[build-dependencies]
rustc-embed = "0.8"
```