```rust
use std::collections::HashMap;

fn use_session_storage_value(key: &str, default_value: Option<&str>) -> (Option<String>, fn(&mut Option<&mut String>)) {
    let mut state = HashMap::new();
    if let Some(value) = state.get(key) {
        return (Some(value.to_string()), move |s| s.replace(value.to_string(), ""));
    }
    state.insert(key, default_value.map(|v| v.to_string()));
    (state.remove(key).map(|v| v.to_string()), move |_| {})
}
```