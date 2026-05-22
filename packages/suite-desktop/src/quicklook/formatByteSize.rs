```rust
fn format_byte_size(size: f64) -> String {
    let mut value = size;
    let mut suffix = 0;

    while value > 1023.9 && suffix + 1 < suffixes.len() {
        value /= 1024.0;
        suffix += 1;
    }

    format!("{:.1} {}", value, suffixes[suffix])
}
```