```rust
use std::collections::HashSet;

// toggles an item in an array based on reference equality or a predicate to determine if the item should be toggled in/out
// this function is pure - it always returns a new array
pub fn toggle<T>(array: Vec<T>, item: T, predicate: impl Fn(&T) -> bool) -> Vec<T> {
  let mut seen = HashSet::new();
  let mut newArray = vec![];

  for element in &array {
    if seen.insert(element) || !predicate(element) {
      newArray.push(*element);
    }
  }

  if newArray.len() < array.len() {
    newArray.push(item);
  }

  newArray
}
```