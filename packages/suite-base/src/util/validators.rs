```rust
type ValidationResult = &str | &HashMap<&str, &str>;

fn validation_error_to_string(validation_result: ValidationResult) -> String {
    match validation_result {
        s if s.is_str() => s,
        m if m.is_hashmap() => {
            let mut result = Vec::new();
            for (key, value) in m.iter() {
                result.push(format!("{}: {}", key, value));
            }
            result.join(", ")
        },
        _ => unreachable!(),
    }
}
```