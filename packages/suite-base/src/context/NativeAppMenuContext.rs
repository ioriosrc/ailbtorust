```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

type Handler = Box<dyn Fn()>;

struct NativeAppMenu {
    handlers: RefCell<Vec<(String, Rc<Handler>)>>,
}

impl NativeAppMenu {
    fn new() -> Self {
        NativeAppMenu {
            handlers: RefCell::new(Vec::new()),
        }
    }

    fn on(&mut self, name: String, handler: Handler) -> Option<Box<dyn Fn()>> {
        let weak_handler = Rc::downgrade(&handler);
        self.handlers.borrow_mut().push((name, weak_handler));
        Some(Box::from(handler))
    }

    fn unregister(&mut self, name: &str) -> bool {
        for (index, &(ref _name, ref weak_handler)) in self.handlers.borrow_mut().iter_mut() {
            if *name == *_name && weak_handler.upgrade().is_none() {
                self.handlers.borrow_mut().remove(index);
                return true;
            }
        }
        false
    }
}

pub struct NativeAppMenuContext {
    inner: Rc<RefCell<NativeAppMenu>>,
}

impl NativeAppMenuContext {
    pub fn new() -> Self {
        NativeAppMenuContext {
            inner: Rc::new(RefCell::new(NativeAppMenu::new())),
        }
    }

    pub fn on(&mut self, name: &str, handler: Box<dyn Fn()> {
        let weak_handler = Rc::downgrade(&handler);
        self.inner.borrow_mut().handlers.push((name.to_string(), weak_handler));
    }

    pub fn unregister(&mut self, name: &str) -> bool {
        for (index, &(ref _name, ref weak_handler)) in self.inner.borrow_mut().handlers.iter_mut() {
            if *name == *_name && weak_handler.upgrade().is_none() {
                self.inner.borrow_mut().handlers.remove(index);
                return true;
            }
        }
        false
    }

    pub fn subscribe(&self, name: &str, handler: Box<dyn Fn()> {
        let weak_handler = Rc::downgrade(&handler);
        self.inner.borrow_mut().handlers.push((name.to_string(), weak_handler));
    }
}

impl Default for NativeAppMenuContext {
    fn default() -> Self {
        NativeAppMenuContext::new()
    }
}
```