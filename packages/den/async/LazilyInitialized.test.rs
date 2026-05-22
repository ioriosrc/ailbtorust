```rust
use std::rc::Rc;

pub struct LazilyInitialized {
    compute: Rc<dyn Fn() -> Result<i32, ()>>,
}

impl LazilyInitialized {
    pub fn new<F: 'static + Fn() -> Result<i32, ()>>(compute: F) -> Self {
        LazilyInitialized {
            compute: Rc::new(compute),
        }
    }

    pub async fn get(&self) -> i32 {
        let mut result = None;
        let response = (self.compute)();
        if let Ok(value) = response {
            result = Some(value);
        } else {
            panic!("Failed to compute value");
        }
        // Simulate an asynchronous operation
        std::thread::sleep(std::time::Duration::from_secs(1));
        result.unwrap()
    }
}
```