```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, join};

#[wasm_bindgen]
pub async fn run_script(script_str: &str) -> Result<(), JsValue> {
    let js_handle = js_sys::Window::get().unwrap().document().unwrap().body().unwrap();
    let body_element = js_handle.dyn_ref::<web_sys::HtmlElement>()?;
    body_element.set_inner_html(script_str);
    Ok(())
}
```