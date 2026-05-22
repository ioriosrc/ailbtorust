```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

mod Condvar {
    pub struct Condvar {
        cond: Arc<Mutex<CondvarInner>>,
        waiters: Arc<Vec<(Mutex<()>), Condvar>>,
    }

    struct CondvarInner {
        notified: bool,
    }

    impl Condvar {
        pub fn new() -> Self {
            let inner = CondvarInner { notified: false };
            let cond = Arc::new(Mutex::new(inner));
            let waiters = Arc::new(Vec::new());
            Self { cond, waiters }
        }

        pub async fn wait(&self) {
            let mut guard = self.cond.lock().unwrap();
            while !guard.notified {
                guard.wait().await.unwrap();
            }
            guard.notified = false;
        }

        pub async fn notify_one(&self) {
            let mut inner_guard = self.cond.lock().unwrap();
            let waiters = &mut *self.waiters.borrow_mut();

            if let Some(waker) = waiters.pop() {
                waker.wake();
            } else if !inner_guard.notified {
                inner_guard.notified = true;
                for waker in waiters.iter() {
                    waker.wake();
                }
            }
        }

        pub async fn notify_all(&self) {
            let mut inner_guard = self.cond.lock().unwrap();

            while !inner_guard.notified {
                inner_guard.notified = true;
                for waker in waiters.iter_mut() {
                    waker.wake();
                }
            }
        }
    }

    struct Waker;

    impl Waker {
        fn new() -> Self {
            Self {}
        }

        async fn wake(&self) {
            // Simulate waking a thread by yielding control
            thread::yield_now().await;
        }
    }
}
```