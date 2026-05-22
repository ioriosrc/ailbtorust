```rust
fn filter_map<T, U>(
  values: impl IntoIterator<Item = T>,
  map_fn: fn(T, usize) -> Option<U>,
) -> Vec<U> {
  let mut results = Vec::new();
  for (index, item) in values.into_iter().enumerate() {
    if let Some(mapped_item) = map_fn(item, index) {
      results.push(mapped_item);
    }
  }
  results
}
```