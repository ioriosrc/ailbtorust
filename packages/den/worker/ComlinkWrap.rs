```rust
use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};

/// Wraps an instantiated `Worker` and exposes its API in the same way that `Comlink.wrap` does
/// but it also provides a `dispose` function to terminate the worker and release the comlink proxy.
/// This can help prevent memory leaks when the comlink proxy is unable to garbage collect itself due to
/// unresolved promises which can occur if the worker is terminated while processing a request.
///
/// @param worker - worker to be wrapped by comlink
/// @returns remote - API for worker wrapped by comlink. What is normally received from Comlink.wrap
/// @returns dispose - function to release the comlink proxy and to terminate the worker
pub fn ComlinkWrap<T>(worker: Worker): { remote: Arc<dyn T>; dispose: Arc<Mutex<()>> } {
    let remote = Arc::new(Comlink::wrap(worker));
    let dispose = Arc::new(Mutex::new(()));

    thread::spawn(move || {
        worker.join().unwrap();
        drop(dispose.lock());
    });

    (remote, dispose)
}
```