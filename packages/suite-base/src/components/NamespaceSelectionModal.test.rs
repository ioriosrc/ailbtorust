```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(message: *const u8);
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console.log("Hello, World!");
    Ok(())
}

// Your Rust code here
```