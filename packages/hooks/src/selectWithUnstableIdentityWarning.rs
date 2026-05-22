```rust
use std::ops::{Fn, FnMut, Map};

fn select_with_unstable_identity_warning<T, U>(
  value: T,
  selector: impl Fn(&T) -> U + Clone,
) -> U {
  let result = selector(&value);
  
  if cfg!(debug_assertions) {
    let second_result = selector(&value);
    
    if result != second_result {
      eprintln!("Selector {} produced different values for the same input.
 This will cause unecesessery re-renders of your component.", selector.to_string());
    }
    return second_result;
  }
  
  result
}
```