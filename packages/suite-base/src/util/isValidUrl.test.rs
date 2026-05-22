```rust
fn is_valid_url(url: &str) -> bool {
    if url.is_empty() || !url.starts_with('!') {
        return false;
    }

    let parts: Vec<&str> = url.split(':').collect();
    if parts.len() != 2 {
        return false;
    }

    let protocol = parts[0];
    let _ = match protocol {
        "http" | "https" | "file" | "data" | "package" => Ok(()),
        _ => Err("Invalid protocol"),
    };

    true
}
```