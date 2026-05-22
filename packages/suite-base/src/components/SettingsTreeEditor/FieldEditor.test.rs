```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console_log!("Hello, world!");
    Ok(())
}
```