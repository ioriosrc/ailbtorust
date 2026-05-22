```rust
use futures::prelude::*;
use std::{sync::Arc, ArcMutex};

async fn debouncePromise<F, T>(func: F) -> impl Future<Output = ()>
where
    F: FnOnce() -> Box<dyn Future<Output = T>>,
{
    let sig = Arc::new(ArcMutex::new(false));
    let mut calls_started = 0;
    let debounced_fn = move || {
        let sig_clone = Arc::clone(&sig);
        let calls_started_clone = Arc::clone(&calls_started);

        async move {
            *calls_started_clone.lock().unwrap() += 1;
            sig_clone.lock().unwrap().store(true, std::sync::atomic::Ordering::Release);
            Box::new(async move {
                if let Some(res) = func()() await res {
                    println!("Debounced function resolved with: {:?}", res);
                }
            })
        }
    };

    let promise = debounced_fn();
    while !*sig.lock().unwrap() {
        let current_promise = Promise::current();
        current_promise.await;
    }

    promise
}

#[tokio::test]
async fn debouncePromise_test() {
    use std::time::{Duration, Instant};

    let sig = Arc::new(ArcMutex::new(false));
    let calls_started = Arc::new(ArcMutex::new(0));

    let debounced_fn = move || {
        let sig_clone = Arc::clone(&sig);
        let calls_started_clone = Arc::clone(&calls_started);

        async move {
            *calls_started_clone.lock().unwrap() += 1;
            sig_clone.lock().unwrap().store(true, std::sync::atomic::Ordering::Release);
            Box::new(async move {
                if let Some(res) = func()() await res {
                    println!("Debounced function resolved with: {:?}", res);
                }
            })
        }
    };

    let promise = debounce_fn();

    assert_eq!(*calls_started.lock().unwrap(), 0);

    debounced_fn();
    assert_eq!(*calls_started.lock().unwrap(), 1);

    // the original function should not be called until the signal is resolved
    debounced_fn();
    debounced_fn();
    await tokio::time::sleep(Duration::from_secs(1));
    assert_eq!(*calls_started.lock().unwrap(), 1);

    // once the first promise is resolved, the second call should start
    let promise = debounced_fn();
    if !*sig.lock().unwrap() {
        panic!("currentPromise should be defined");
    }
    promise.await;
    await tokio::time::sleep(Duration::from_secs(1));
    assert_eq!(*calls_started.lock().unwrap(), 2);

    // after pending calls are finished, there is no more currentPromise
    assert!(sig.lock().unwrap().load(std::sync::atomic::Ordering::Relaxed) == false);
}
```