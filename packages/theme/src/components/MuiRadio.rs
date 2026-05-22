```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn document() -> *mut web_sys::Document;
    #[wasm_bindgen(js_namespace = "document")]
    fn createElement(element_type: &str) -> *mut web_sys::Element;
}

#[wasm_bindgen]
pub struct MuiRadio {
    element: *mut web_sys::Element,
}

impl MuiRadio {
    pub fn new() -> Self {
        let document = unsafe { document() };
        let element = unsafe { createElement("button") as *mut web_sys::Element };

        // Set default properties
        unsafe {
            let set_attribute = |name: &str, value: &str| {
                let attribute_name = web_sys::JsValue::from(name);
                let attribute_value = web_sys::JsValue::from(value);
                document.as_ref().set_attribute(&attribute_name, &attribute_value);
            };

            set_attribute("disableRipple", "true");
            set_attribute("size", "small");
        }

        MuiRadio { element }
    }
}
```