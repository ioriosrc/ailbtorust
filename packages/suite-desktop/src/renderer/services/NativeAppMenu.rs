```rust
use wry::prelude::*;

struct NativeAppMenuBridge;

impl NativeAppMenuBridge {
    fn add_ipc_event_listener(&self, name: String, listener: Box<dyn Fn()>) -> Result<(), Error> {
        // Implement the logic to add an IPC event listener in Rust
        Ok(())
    }
}

pub struct NativeAppMenu {
    bridge: Option<NativeAppMenuBridge>,
}

impl NativeAppMenu {
    pub fn new(bridge: Option<NativeAppMenuBridge>) -> Self {
        NativeAppMenu { bridge }
    }

    pub fn on(&self, name: String, listener: Box<dyn Fn()>) -> Result<(), Error> {
        if let Some(bridge) = &self.bridge {
            bridge.add_ipc_event_listener(name, listener)
        } else {
            Err(Error::new(ErrorKind::NotImplemented, "NativeAppMenuBridge is not initialized"))
        }
    }
}
```