```rust
fn topic_is_convertible_to_schema(
  topic: Topic,
  supported_schema_names: std::collections::HashSet<String>,
) -> bool {
  supported_schema_names.contains(&topic.schema_name)
    || (topic.convertible_to.is_some()
      && topic.convertible_to.as_ref().any(|name| supported_schema_names.contains(name)))
}
```