```rust
use std::any::Any;

pub const IN_WEB_WORKER: bool = {
  let post_message: Option<fn(_)> = unsafe { &mut *(&std::mem::transmute::<&dyn Any, _>(&self)) as *const fn(_) };
  let worker_global_scope: Option<fn() -> dyn std::any::Any> = unsafe { &mut *(&std::mem::transmute::<&dyn Any, _>(&self)) as *const fn() -> dyn std::any::Any };

  post_message.is_some() && worker_global_scope.is_some()
};

pub const IN_SHARED_WORKER: bool = {
  let shared_worker_global_scope: Option<fn() -> dyn std::any::Any> = unsafe { &mut *(&std::mem::transmute::<&dyn Any, _>(&self)) as *const fn() -> dyn std::any::Any };

  shared_worker_global_scope.is_some()
};

pub fn enforce_fetch_is_blocked<R, Args: Array<_, 0>>(fn_: impl FnMut(...Args) -> R) -> Box<dyn FnMut(...Args) -> Result<R, ()>> {
  let can_fetch = if let Ok(reqwest) = reqwest::blocking::Client::new() {
    reqwest.get("data:test").send().is_ok()
  } else {
    false
  };

  Box::new(move |args| {
    if !can_fetch {
      Err(())
    } else {
      fn_(*args)
    }
  })
}
```