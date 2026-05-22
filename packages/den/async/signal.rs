```rust
pub type Signal<T> = Box<dyn std::future::Future<Item = T>>;

pub fn signal<T>() -> Signal<T> {
    let (resolve, reject) = futures::sync::oneshot();

    let promise = futures::executor::block_on(async move {
        resolve();
        reject().await.unwrap()
    });

    Box::new(promise)
}
```