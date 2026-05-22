```rust
use std::future::{Future, Pin};
use std::pin::PinMut;
use std::task::{Context, Poll};

type DebouncedFn<Args> = Box<dyn Fn(&mut Context<'_>, &mut [Args]) -> Poll<(), ()>>;

// debouncePromise returns a function which wraps calls to `fn`.
// The returned debounceFn ensures that only one `fn` call is executing at a time.
// If debounceFn is called while `fn` is still executing, it will queue the call until the
// current invocation is complete.
// If debounceFn is called multiple times while `fn` is still executing, then only the last
// call's arguments will be saved for the next execution of `fn`.
pub fn debouncePromise<Args>(
  fn_: Box<dyn Fn(&mut Context<'_>, &mut [Args]) -> Poll<(), ()>>,
) -> DebouncedFn<Args> {
  let mut current_future: Option<PinMut<Box<dyn Future<Output = ()>>>> = None;
  let mut call_pending = Vec::new();

  let debounced_fn: DebouncedFn<Args> = move |ctx, args| {
    if current_future.is_none() && !call_pending.is_empty() {
      current_future = Some(PinMut::new(Box::pin(fn_(ctx, &mut call_pending.last().unwrap()))));
    } else if current_future.is_none() {
      current_future = Some(PinMut::new(Box::pin(fn_(ctx, args))));
    }

    match current_future.as_mut() {
      Some(pin) => pin.poll(ctx),
      None => Poll::Pending,
    }
  };

  debounced_fn
}
```