```rust
use std::collections::VecDeque;
use std::future::{Future, Poll};
use std::pin::Pin;

struct RequestQueue {
    max_concurrent: usize,
    active_count: usize,
    queue: VecDeque<Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()>>>>,
}

impl RequestQueue {
    fn new(max_concurrent: usize) -> Self {
        Self {
            max_concurrent,
            active_count: 0,
            queue: VecDeque::new(),
        }
    }

    async fn run<T>(&mut self, fn_: Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()>>>>) -> Result<T, Box<dyn std::error::Error>> {
        while self.active_count >= self.max_concurrent {
            if let Some(next) = self.queue.pop_front() {
                next.await;
            } else {
                break;
            }
        }

        self.active_count += 1;

        Ok(fn_.await)
    }
}

// Global queue for all HTTP requests
pub static GLOBAL_REQUEST_QUEUE_MAX_CONCURRENT: usize = 256;
pub static mut global_request_queue: RequestQueue = RequestQueue::new(GLOBAL_REQUEST_QUEUE_MAX_CONCURRENT);
```