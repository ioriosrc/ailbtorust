```rust
fn normalize_extensions(all_extensions: Vec<&str>) -> Vec<String> {
    all_extensions.into_iter()
                   .map(|ext| if ext.starts_with('.') { ext.to_string() } else { format!(".{}", ext) })
                   .collect()
}
```