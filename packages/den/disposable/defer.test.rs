```rust
use std::sync::{Arc, Mutex};

/// A deferred function that can be executed when it is disposed.
struct Deferred {
    inner: Arc<Mutex<dyn FnMut() + Send>>,
}

impl Deferred {
    /// Creates a new deferred function.
    fn new<T>(f: T) -> Self {
        Deferred {
            inner: Arc::new(Mutex::new(f)),
        }
    }

    /// Disposes of the deferred function, executing it if it has not already been executed.
    fn dispose(&self) {
        let mut f = self.inner.lock().unwrap();
        (f)();
    }
}

/// Tests for the `Deferred` struct.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_call_the_provided_function_when_disposed() {
        let called = false;
        let deferred = Deferred::new(|| {
            called = true;
        });

        defer(deferred.dispose);

        assert_eq!(called, false);
    }

    #[test]
    fn should_call_the_provided_function_with_throw_in_scope() {
        let called = false;
        let deferred = Deferred::new(|| {
            called = true;
            panic!("some error");
        });

        assert_eq!(deferred.dispose(), Err("some error"));

        assert_eq!(called, true);
    }
}
```