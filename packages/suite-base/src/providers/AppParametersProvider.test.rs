```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn app_parameters_provider() -> JsValue {
    let default_layout = "defaultLayout";
    JsObject::new_with_properties(&[
        ("defaultLayout", JsString::from(default_layout)),
    ])
    .into()
}
```