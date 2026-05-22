```rust
// SPDX-License-Identifier: MIT

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mui_button_group() -> JsValue {
    let props = js_sys::Object::new();
    props.set_property("disableRipple", true);
    props.set_property("disableFocusRipple", true);

    serde_json::to_string(&props).unwrap().into()
}
```