```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct MemoizeWeak<T> {
    fn_: Rc<dyn Fn(&mut Self) -> Option<Self>>,
    result_weak_: RefCell<Option<Rc<T>>>,
}

impl<T> MemoizeWeak<T> {
    pub fn new(fn_: Rc<dyn Fn(&mut Self) -> Option<Self>>) -> Self {
        Self {
            fn_,
            result_weak_: RefCell::new(None),
        }
    }

    pub fn get_mut(&self) -> Option<&mut T> {
        if let Some(result_weak) = self.result_weak_.borrow().as_ref() {
            return Rc::downgrade(result_weak);
        }
        self.fn_(&mut *self).map(|value| Rc::new(value))
    }

    pub fn clear(&self) {
        *self.result_weak_.borrow_mut() = None;
    }
}
```