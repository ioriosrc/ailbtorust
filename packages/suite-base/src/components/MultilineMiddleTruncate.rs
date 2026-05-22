```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn multiline_middle_truncate(text: &str) -> JsValue {
    let mut elements = Vec::new();

    for line in text.split("\n") {
        elements.push(web_sys::ElementRef::from_text(&line));
    }

    web_sys::JsValue::from_iter(elements)
}
```