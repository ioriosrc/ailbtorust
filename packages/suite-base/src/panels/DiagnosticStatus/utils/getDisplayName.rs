```rust
fn get_display_name(hardware_id: &str, name: &str) -> String {
  match (name, hardware_id) {
    (Some(name), Some(hardware_id)) => format!("{}: {}", hardware_id, name),
    (Some(name), None) => name.to_string(),
    (None, Some(hardware_id)) => hardware_id.to_string(),
    _ => DISPLAY_EMPTY_STATE,
  }
}
```