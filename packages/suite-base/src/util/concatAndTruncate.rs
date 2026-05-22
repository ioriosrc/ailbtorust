```rust
use std::slice::Iter;

pub fn concat_and_truncate<T>(
  array1: &[T],
  array2: &[T],
  limit: usize,
) -> Vec<T> {
  let mut result = Vec::new();

  // Add elements from array1 up to the limit minus the length of array2
  if limit > array2.len() {
    result.extend_from_slice(&array1[array1.len() - limit..]);
  }

  // Add elements from array2 to the result, limited by the remaining space in the limit
  for &item in array2.iter().skip(limit - result.len()) {
    result.push(item);
  }

  result
}
```