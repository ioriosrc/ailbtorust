```rust
fn merge_multiple_file_names(names: &[String]) -> String {
    if names.is_empty() {
        return "".to_string();
    }

    if names.len() == 1 {
        return names[0].clone();
    }

    let mut result = names[0].clone();
    for name in &names[1..] {
        result.push_str(", ");
        result.push_str(name);
    }

    result
}
```