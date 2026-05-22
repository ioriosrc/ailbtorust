```rust
use std::cmp;

/// Continues to return the same instance as long as shallow equality is maintained.
pub fn use_shallow_memo<T>(value: T) -> T {
  let mut ref = &mut value; // We need a mutable reference for mutation

  if cmp::PartialEq::eq(&value, &*ref) {
    return *ref;
  }

  *ref = value; // Update the reference to the new value
  value // Return the updated value
}
```