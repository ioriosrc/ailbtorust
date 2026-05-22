```rust
use std::rc::Rc;

pub fn use_synchronous_mounted_state() -> Rc<dyn Fn() -> bool>> {
    let mut mounted = Rc::new(false);

    let get = Rc::new(move || {
        mounted.clone()
    });

    use_layout_effect(|| {
        mounted.set(true);

        move || {
            mounted.set(false);
        }
    }, []);

    get
}
```