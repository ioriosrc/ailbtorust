```rust
use std::future::{Future, Ready};
use std::pin::Pin;

pub async fn set_immediate<F: FnOnce(()) -> Future<()>>(
    callback: F,
    args: Vec<()>,
) -> Pin<Box<dyn Future<Output = ()>>> {
    let future = Box::pin(async move {
        // Simulate some delay or blocking operation
        std::thread::sleep(std::time::Duration::from_secs(1));

        callback();
    });

    future
}
```