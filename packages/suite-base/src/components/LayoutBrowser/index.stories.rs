```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console.log("Hello, world!");
    Ok(())
}
```