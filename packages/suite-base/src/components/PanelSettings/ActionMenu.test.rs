```rust
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&JsValue::from("Hello, World!"));
    Ok(())
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    // Initialize DOM here if needed
    Ok(())
}
```