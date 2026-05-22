```rust
fn normalize_extensions(extensions: Vec<&str>) -> Vec<String> {
    extensions
        .iter()
        .map(|&ext| if !ext.starts_with('.') { ".".to_string() + ext } else { ext.to_string() })
        .collect()
}
```