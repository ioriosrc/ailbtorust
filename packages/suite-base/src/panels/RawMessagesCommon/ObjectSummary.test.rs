```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_component(value: JsValue) -> JsValue {
    let mock_app_configuration = js_sys::Object::new();
    mock_app_configuration.set("get", value.clone());

    let container = web_sys::window().unwrap()
        .document()
        .unwrap()
        .create_element("div")?;

    container.set_inner_html(&format!(
        "items: {}, keys: {}",
        value.len(),
        value.as_object().map(|obj| obj.keys().len()).unwrap_or_default()
    ));

    JsValue::from(container)
}
```