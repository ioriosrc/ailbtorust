```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn open_panel_menu() {
    let buttons = document.query_selector_all(".panel-menu")?;
    buttons[0].click();
}

#[wasm_bindgen]
pub async fn go_full_screen() {
    open_panel_menu().await;
    // Assuming there's a way to interact with the panel menu's fullscreen toggle button
    // For example, by finding the element and clicking it programmatically.
}
```