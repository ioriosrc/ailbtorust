```rust
use std::path::Path;

fn main() {
    let path = Path::new("EventOutlinedIcon.svg");
    // Process the SVG file or use it as needed in your Rust code.
    println!("SVG file located at: {}", path.to_string_lossy());
}
```

Note: This solution assumes that you have a package manager installed on your system to install `svg` crate, which is used for reading SVG files in Rust. You can add this dependency in your `Cargo.toml`:

```toml
[dependencies]
svg = "0.32.1"
```