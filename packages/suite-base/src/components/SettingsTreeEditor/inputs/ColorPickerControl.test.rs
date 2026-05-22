```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    js_sys::console::log(&JsValue::from_str("Hello from Rust!"));
}
```

Este código usa WebAssembly Bindgen para criar um módulo JavaScript que imprime "Hello from Rust!" no console.