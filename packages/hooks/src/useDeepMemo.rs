```rust
use std::rc::Rc;

pub fn use_deep_memo<T>(value: &T) -> Rc<T> {
  let mut current_ref = Rc::new(value.clone());

  if value == current_ref.as_ref() {
    return current_ref;
  }

  current_ref = Rc::clone(value);
  return current_ref;
}
```