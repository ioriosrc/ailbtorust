```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn document_drop_listener() {
    let wrapper = web_sys::window().unwrap()
        .document()
        .unwrap()
        .body()
        .expect("could not find body");

    // Create root using create_root macro from wasm_bindgen
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn main() -> Result<(), JsValue> {
        let root = web_sys::window().unwrap()
            .document()
            .unwrap()
            .body()
            .expect("could not find body");

        // Create a div and append it to the wrapper
        let div = web_sys::ElementRef::new().expect("Failed to create ElementRef");
        div.set_inner_html("<div>Document Drop Listener</div>");
        wrapper.append_child(&div);

        Ok(())
    }
}
```