```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = MuiMenuItem)]
    fn createMuiMenuItem();
}

#[wasm_bindgen]
struct OverrideComponentReturn;

#[wasm_bindgen]
impl OverrideComponentReturn for i32 {
    fn to_js(self) -> JsValue {
        self as f64.into()
    }
}
```