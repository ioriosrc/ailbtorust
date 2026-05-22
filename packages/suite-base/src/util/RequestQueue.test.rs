```rust
use std::collections::{HashMap, VecDeque};
use std::future::{self, ready};
use std::pin::Pin;

struct RequestQueue {
    max_concurrent: usize,
    queue: VecDeque<Pin<Box<dyn FnOnce(&mut HashMap<String, String>) -> Pin<Box<dyn Future<Output = ()>>>>>,
}

impl RequestQueue {
    fn new(max_concurrent: usize) -> Self {
        RequestQueue {
            max_concurrent,
            queue: VecDeque::new(),
        }
    }

    async fn run<F>(&mut self, func: F)
    where
        F: FnOnce(&mut HashMap<String, String>) -> Pin<Box<dyn Future<Output = ()>>>,
    {
        if self.queue.len() < self.max_concurrent {
            let (key, value) = BasicBuilder.string().build();
            let mock_fn = Box::pin(move |map| func(map));
            let mut map = HashMap::new();
            map.insert(key.to_string(), value.to_string());
            let future = Box::pin(func(&mut map));
            self.queue.push_back(Pin::from(future));
        }
    }
}

#[tokio::test]
async fn test_request_queue_single_request() {
    let value = BasicBuilder.string().build();
    let mut queue = RequestQueue::new(2);
    let mock_fn = Box::pin(move |map| async move {
        map.insert("key".to_string(), "value".to_string());
    });
    let result = queue.run(mock_fn).await;
    assert_eq!(result, value);
}

#[tokio::test]
async fn test_request_queue_multiple_requests() {
    const max_concurrent: usize = 3;
    const total_requests: usize = 4;
    let mut queue = RequestQueue::new(max_concurrent);

    let mocks = create_mock_with_promises(total_requests);

    for mock in &mocks[..max_concurrent] {
        let (key, value) = BasicBuilder.string().build();
        let mock_fn = Box::pin(move |map| async move {
            map.insert(key.to_string(), value.to_string());
        });
        let future = Box::pin(func(&mut map));
        queue.queue.push_back(Pin::from(future));
    }

    for mock in &mocks[max_concurrent..] {
        let resolve = mock.0();
        resolve.await;
    }

    for mock in &mocks[..max_concurrent] {
        assert_eq!(mock.0().await, ());
    }
}
```