```rust
pub const DESKTOP_WINDOW: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/desktop-window.tsx"));
```

Note: This code assumes that the `desktop-window.tsx` file is located in the `assets` directory of your Rust project. You will need to ensure that this directory exists and contains the `desktop-window.tsx` file.