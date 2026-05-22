```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AddPanelMenu {
    open: bool,
}

impl AddPanelMenu {
    pub fn new() -> Self {
        Self { open: false }
    }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn close(&mut self) {
        self.open = false;
    }
}
```