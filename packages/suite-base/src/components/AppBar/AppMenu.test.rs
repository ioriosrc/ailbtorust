```rust
use wasm_bindgen::prelude::*;
use js_sys::{Promise, Window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Initialize your application here
    Ok(())
}

#[wasm_bindgen]
pub fn handle_file_menu_click() {
    // Handle File menu actions here
}

#[wasm_bindgen]
pub fn handle_view_menu_click(side: &str) {
    // Handle View menu actions here
}

#[wasm_bindgen]
pub fn handle_help_menu_click(menu_name: &str) {
    if menu_name == "documentation" {
        open_documentation_link();
    } else if menu_name == "preferences" {
        // Handle preferences action here
    }
}

fn open_documentation_link() {
    let window = Window::get_global_object()?;
    let document = window.document().unwrap();
    let url = URL::parse(LICHTBLICK_DOCUMENTATION_LINK).unwrap();
    let anchor = web_sys::ElementRef::new("a").unwrap();
    anchor.set_attribute("href", &url.as_string())?;
    anchor.set_attribute("target", "_blank")?;
    document.body().unwrap().append_child(&anchor)?;
}

#[wasm_bindgen]
pub fn handle_recent_source_click(source_id: &str) {
    // Handle recent source action here
}
```