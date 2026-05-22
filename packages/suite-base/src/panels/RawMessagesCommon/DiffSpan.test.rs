```rust
use wasm_bindgen::prelude::*;
use js_sys::WebAssemblyModule;
use std::error::Error;

#[wasm_bindgen(start)]
fn main() -> Result<(), Box<dyn Error>> {
    console_log!("Hello, world!");
    Ok(())
}

#[wasm_bindgen]
pub fn diff_span(content: &str) -> String {
    let content_with_spaces = content.trim();
    format!("DiffSpan: {content_with_spaces}")
}
```