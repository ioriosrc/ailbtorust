```rust
pub mod react {
    use std::rc::Rc;

    pub struct MutableRefObject<T> {
        value: Rc<dyn Any>,
    }

    impl<T> MutableRefObject<T> {
        pub fn new(value: T) -> Self {
            Self { value: Rc::new(Box::new(value)) }
        }

        pub fn get(&self) -> &T {
            unsafe { self.value.downcast_ref::<T>().unwrap() }
        }

        pub fn set(&mut self, value: T) {
            self.value = Rc::new(Box::new(value));
        }
    }

    pub struct Callback<T> {
        callback: Box<dyn Fn(T)>,

        deps: Vec<Rc<dyn Any>>,
    }

    impl<T> Callback<T> {
        pub fn new<F>(callback: F, deps: Vec<Rc<dyn Any>>) -> Self {
            Self {
                callback: Box::new(callback),
                deps,
            }
        }

        pub fn invoke(&self, args: T) {
            if self.deps.iter().all(|dep| dep.is_eq(&args)) {
                (self.callback)(args);
            }
        }
    }
}
```