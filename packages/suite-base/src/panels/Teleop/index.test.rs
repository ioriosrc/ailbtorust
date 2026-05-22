```rust
use wasm_bindgen::prelude::*;
use js_sys::{self, Array};

#[wasm_bindgen]
pub struct TeleopPanelAdapter {
    config: Object,
    save_config: Closure<dyn Fn(Vec<u8>)>,
}

impl TeleopPanelAdapter {
    pub fn new(config: Object, save_config: Closure<dyn Fn(Vec<u8>)>) -> Self {
        TeleopPanelAdapter { config, save_config }
    }

    // Other methods and properties of the TeleopPanelAdapter
}
```