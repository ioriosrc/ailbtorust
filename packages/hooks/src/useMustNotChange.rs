```rust
use std::rc::Rc;

pub fn use_must_not_change<T>(value: &mut T) {
    let value_ref = Rc::new(value);
    if *value_ref != *value {
        panic!("Value must not change");
    }
    *value_ref = *value;
}
```