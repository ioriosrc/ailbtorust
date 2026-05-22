```rust
use std::collections::HashMap;

pub fn pick_fields(record: &HashMap<String, serde_json::Value>, fields: &[String]) -> HashMap<String, serde_json::Value> {
  if fields.is_empty() {
    return HashMap::new();
  }

  let mut result = HashMap::new();

  for field in fields {
    if record.contains_key(field) {
      let value = record.get(field).unwrap().clone();
      result.insert(field.clone(), value);
    }
  }

  result
}
```