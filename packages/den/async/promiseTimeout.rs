```rust
use std::time::Duration;
use std::future::Future;
use std::pin::Pin;

/// Error for promise timeouts from `promise_timeout()`
#[derive(Debug)]
pub struct PromiseTimeoutError(String);

/// Executes a promise with a timeout.
///
/// If the promise takes longer than the specified timeout duration (in milliseconds), it will be
/// rejected with a timeout error.
///
/// Note: Make sure the input promise resolves, rejects, or otherwise go out of scope. A long-lived
/// promise that never resolves holds onto its resolution callbacks.
///
/// @param promise The promise to execute
/// @param ms The timeout duration in milliseconds.
/// @returns A future that resolves with the result of the input promise or rejects with a
/// PromiseTimeoutError.
pub async fn promise_timeout<T>(
    promise: Pin<Box<dyn Future<Output = Result<T, std::io::Error>>>>,
    ms: u32,
) -> Result<T, PromiseTimeoutError> {
    let mut timeout_timer = Timeout::new(Duration::from_millis(ms))?;

    // We avoid using `Promise.race` here since it is susceptible to memory leaks for unresolved promises
    // https://github.com/nodejs/node/issues/17469
    //
    // With `Promise.race` you might be tempted to race the input promise against a promise that resolve
    // after a timeout. However, if you clear the timeout when the input promise resolves, you'll be
    // left with a promise that never resolves passed as a contender to `Promise.race`.
    match pin!(promise).await {
        Ok(result) => Ok(result),
        Err(err) => {
            timeout_timer.cancel();
            Err(PromiseTimeoutError(format!("Promise timed out after {}ms", ms)))
        }
    }
}

// Helper to create a future that resolves in a given amount of time
fn sleep(ms: u32) -> impl Future<Output = ()> {
    async move {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    }
}
```