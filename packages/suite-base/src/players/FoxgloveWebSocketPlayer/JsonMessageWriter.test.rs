```rust
use std::vec::Vec;
use wasm_bindgen::{JsValue, unwrap};

#[wasm_bindgen]
extern "C" {
    fn write_message(json: &JsValue) -> Vec<u8>;
}

#[wasm_bindgen]
pub struct JsonMessageWriter;

impl JsonMessageWriter {
    pub fn write_message(&self, message: JsValue) -> Vec<u8> {
        let result = write_message(&message);
        unwrap(result)
    }
}
```

No Rust, a maioria das implementações do TypeScript/React seria equivalente à um código semelhante, mas o uso de bibliotecas externas como `wasm-bindgen` é necessário para se comunicar com os componentes web.