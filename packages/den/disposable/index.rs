```rust
use std::future::Future;
use std::sync::{Arc, Mutex};

pub struct Defer {
    inner: Arc<Mutex<dyn FnMut() + 'static>>,
}

impl Defer {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut() + 'static,
    {
        Defer {
            inner: Arc::new(Mutex::new(f)),
        }
    }

    pub async fn defer(&self) -> Result<(), ()> {
        let mut f = self.inner.lock().unwrap();
        (*f)();
        Ok(())
    }
}
```