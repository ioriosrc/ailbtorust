```rust
use std::any::{Any, TypeId};
use std::cell::RefCell;

pub struct OsContext {
    // Define your OsContext fields here
}

lazy_static! {
    static ref OS_CONTEXT: RefCell<Option<OsContext>> = RefCell::new(None);
}

pub fn get_os_context() -> &'static OsContext {
    let os_context = OS_CONTEXT.borrow();
    os_context.as_ref().expect("OS Context is not initialized")
}
```

Note that this Rust version uses `lazy_static` for lazy initialization and `RefCell` for thread-safe mutable access. The `OsContext` struct should be defined to match the TypeScript interface provided in the given code snippet.