```rust
use std::sync::{Arc, Mutex};
use std::task::Poll;

pub struct LazilyInitialized<T> {
    state: Arc<Mutex<State<T>>>,
}

struct State<T> {
    promise: Option<Pin<Box<dyn Future<Output = T>>>>,
}

impl<T> LazilyInitialized<T> {
    pub fn new(compute: impl FnOnce() -> Pin<Box<dyn Future<Output = T>>>>) -> Self {
        LazilyInitialized {
            state: Arc::new(Mutex::new(State { promise: None })),
        }
    }

    pub async fn get(&self) -> T {
        let mut state = self.state.lock().unwrap();
        if state.promise.is_none() {
            let future = Box::pin(compute());
            state.promise = Some(future);
        }
        loop {
            match state.promise.as_mut().unwrap().poll(&mut ()) {
                Poll::Pending => continue,
                Poll::Ready(value) => return value,
            }
        }
    }
}
```