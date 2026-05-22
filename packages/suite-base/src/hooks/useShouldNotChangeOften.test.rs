```rust
use std::cell::{RefCell, Ref};
use std::rc::Rc;

fn use_should_not_change_often<T>(initial_val: T) -> Rc<RefCell<bool>> {
    let state = Rc::new(RefCell::new(false));

    if initial_val == *state.borrow() {
        println!("Value has not changed");
    }

    return Rc::new(RefCell::new(true));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_use_should_not_change_often(warn: Vec<&str>) {
        let warn = &mut warn;
        *warn.clear();

        let value = Rc::new(RefCell::new("a"));

        let result = use_should_not_change_often(value.clone());

        assert_eq!(result.borrow(), true);
        assert_eq!(&*warn, &[]);

        value.modify(|val| {
            *val = "a";
        });

        assert_eq!(result.borrow(), true);
        assert_eq!(&*warn, &[]);

        value.modify(|val| {
            *val = "b";
        });

        assert_eq!(result.borrow(), true);
        assert_eq!(&*warn, &[&"Value has not changed".to_string()]);

        value.modify(|val| {
            *val = "b";
        });

        assert_eq!(result.borrow(), true);
        assert_eq!(&*warn, &[]);

        value.modify(|val| {
            *val = "c";
        });

        assert_eq!(result.borrow(), true);
        assert_eq!(&*warn, &[&"Value has not changed".to_string()]);

        value.modify(|val| {
            *val = "d";
        });

        assert_eq!(result.borrow(), false);
        assert_eq!(
            &*warn,
            &["Value has not changed", "Value has not changed"]
        );
    }
}
```