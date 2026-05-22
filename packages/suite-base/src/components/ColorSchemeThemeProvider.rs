```rust
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console.log("Hello from Rust!");
    Ok(())
}

#[wasm_bindgen(module = "src/js/main.js")]
extern "C" {
    pub fn set_color_scheme(value: &str);
}

#[wasm_bindgen(module = "src/js/main.js")]
extern "C" {
    pub fn get_color_scheme() -> String;
}

pub struct ColorSchemeThemeProvider {
    is_dark: Arc<Mutex<bool>>,
}

impl ColorSchemeThemeProvider {
    pub fn new() -> Self {
        let is_dark = Arc::new(Mutex::new(false));
        std::thread::spawn(move || loop {
            if let Ok(current_color_scheme) = get_color_scheme() {
                *is_dark.lock().unwrap() = current_color_scheme == "dark";
            }
            std::thread::sleep(Duration::from_secs(1));
        });
        ColorSchemeThemeProvider { is_dark }
    }

    pub fn render(&self, children: web_sys::HtmlElement) {
        let is_dark_ref = &*self.is_dark;
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .insert_adjacent_html(
                "afterbegin",
                format!(
                    r#"<div class="color-scheme-provider" data-is-dark="{}"></div>"#,
                    is_dark_ref.lock().unwrap() as &str
                ),
            );
    }
}

#[wasm_bindgen]
pub fn set_color_scheme_js(value: JsValue) {
    set_color_scheme(&value.as_string().unwrap());
}
```