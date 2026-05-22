```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EmptyWrapper {
    pub children: JsValue,
    pub enable_new_top_nav: bool,
}

impl EmptyWrapper {
    #[wasm_bindgen]
    pub fn new(children: JsValue, enable_new_top_nav: bool) -> Self {
        Self {
            children,
            enable_new_top_nav,
        }
    }

    #[wasm_bindgen]
    pub async fn render(self: &Self) {
        let t = async move {
            use crate::translations;
            translations::panel_settings()
        }.await;

        if self.enable_new_top_nav {
            EmptyState::new(&self.children).render().await
        } else {
            SidebarContent::new(
                t.clone(),
                Some(children.clone()),
                "panelSettings".to_string(),
            )
            .render()
            .await;
        }
    }
}
```