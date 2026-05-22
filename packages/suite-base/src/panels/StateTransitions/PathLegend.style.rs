```rust
use web_sys::ButtonRef;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(message: &str);
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    alert("Hello, World!");
    Ok(())
}
```