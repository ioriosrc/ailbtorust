```rust
use std::rc::Rc;

#[cfg(debug_assertions)]
pub fn use_value_changed_debug_log_impl<T>(
  value: Rc<dyn Any>,
  msg: &str,
) {
  let prev_value = Rc::new(value.clone());
  if prev_value != *value {
    log::debug!("value changed: {}", msg);
  }
}

#[cfg(not(any(debug_assertions, test)))]
pub fn use_value_changed_debug_log_impl<T>(_value: Rc<dyn Any>, _msg: &str) {}

/// `useValueChangedDebugLog` logs `msg` if `value` changes
///
/// Note: In production builds this hook is a no-op.
#[cfg(not(any(debug_assertions, test)))]
pub fn use_value_changed_debug_log(value: Rc<dyn Any>) -> Rc<dyn Any> {
  let prev_value = Rc::new(value.clone());
  return value;
}
```