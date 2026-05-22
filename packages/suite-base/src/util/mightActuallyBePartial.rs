```rust
pub fn might_actually_be_partial<T>(value: &T) -> std::collections::HashMap<&'static str, Option<&dyn std::any::Any>> {
    let mut result = std::collections::HashMap::new();

    for field_name in value.field_names() {
        if !field_name.starts_with("_") { // Skip fields starting with an underscore
            if value.get(field_name) != Some(&std::any::Any::None) {
                result.insert(field_name, Some(value.get(field_name).unwrap()));
            } else {
                result.insert(field_name, None);
            }
        }
    }

    result
}
```