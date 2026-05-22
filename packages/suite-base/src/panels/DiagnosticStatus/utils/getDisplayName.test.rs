```rust
use std::cmp;

fn get_display_name(hardware_id: &str, name: &str) -> String {
    if hardware_id.is_empty() && name.is_empty() {
        return DISPLAY_EMPTY_STATE.to_string();
    }
    let mut result = hardware_id;
    if !name.is_empty() {
        result.push_str(": ");
        result.push_str(name);
    }
    result
}
```