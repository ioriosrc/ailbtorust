```rust
use web_sys::HtmlElement;

async fn click_panel_menu() {
    let panel_menu_element = document().query_selector("#panel-menu").unwrap();
    if let Ok(panel_menu) = panel_menu_element.downcast::<HtmlElement>() {
        panel_menu.click().await.unwrap();
    }
}
```