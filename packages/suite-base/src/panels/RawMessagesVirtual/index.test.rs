```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RawMessagesVirtualPanel {
    pub panel_type: String,
    pub default_config: JSON,
}

#[wasm_bindgen]
impl RawMessagesVirtualPanel {
    #[wasm_bindgen(constructor)]
    pub fn new(panel_type: String, default_config: JSON) -> Self {
        Self {
            panel_type,
            default_config,
        }
    }

    pub fn panel_type(&self) -> &String {
        &self.panel_type
    }

    pub fn default_config(&self) -> &JSON {
        &self.default_config
    }
}
```