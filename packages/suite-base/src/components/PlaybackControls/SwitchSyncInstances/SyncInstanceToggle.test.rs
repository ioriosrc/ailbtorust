```rust
use wasm_bindgen::prelude::*;
use js_sys::Reflect;

#[wasm_bindgen]
pub fn toggle_sync_instance() {
    let sync_instances = Reflect::get(&sync_instances, &JsValue::from("syncInstances")).unwrap();
    if let Ok(sync_instances_value) = sync_instances.get(&JsValue::from("value")).unwrap().as_bool() {
        Reflect::set(&sync_instances, &JsValue::from("value"), &JsValue::from(!sync_instances_value)).unwrap();
    }
}
```