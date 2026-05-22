```rust
use std::time::{Duration, Instant};
use tokio;

async fn promise_timeout<T>(promises: Vec<Promise<T>>, max_time_ms: Duration) -> Result<(), PromiseTimeoutError> {
    let start = Instant::now();
    while Instant::now() - start < max_time_ms {
        let mut finished = false;
        for promise in &promises {
            if let Ok(_) = promise.await {
                finished = true;
            }
        }
        if finished {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await; // Sleep for a short period
    }

    if !finished {
        Err(PromiseTimeoutError)
    } else {
        Ok(())
    }
}

pub async fn pause_frame_for_promises(promises: Vec<Promise<void>>) -> Result<(), PromiseTimeoutError> {
    let start = Instant::now();
    while Instant::now() - start < MAX_PROMISE_TIMEOUT_TIME_MS {
        let mut finished = false;
        for promise in &promises {
            if let Ok(_) = promise.await {
                finished = true;
            }
        }
        if finished {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await; // Sleep for a short period
    }

    if !finished {
        send_notification("Player ", "Promise timed out", "app", "error");
        Err(PromiseTimeoutError)
    } else {
        Ok(())
    }
}
```