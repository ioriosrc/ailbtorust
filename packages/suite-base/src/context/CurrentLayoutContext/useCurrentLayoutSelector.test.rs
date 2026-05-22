```rust
use wasm_bindgen::prelude::*;
use js_sys::{Array, Object};

#[wasm_bindgen(module = "./src/hooks/use_current_layout.rs")]
extern {
    fn use_current_layout(selector: &Object) -> JsValue;
}

#[wasm_bindgen(module = "./src/hooks/use_current_layout.rs")]
extern {
    fn save_panel_configs(configs: &[Object]) -> Result<(), JsValue>;
}

async fn update_state() {
    let selector = json!({ configById: { foo: { value: 1 } } });
    let layout_state = use_current_layout(&selector);

    if let Ok(layout_state) = layout_state {
        save_panel_configs(vec![json!({ id: "foo", config: { value: 2 } })]);
    }
}

async fn check_value(selector: &Object, expected_value: u32) {
    let layout_state = use_current_layout(&selector);

    if let Ok(layout_state) = layout_state {
        assert_eq!(layout_state.config_by_id["foo"].value as u32, expected_value);
    } else {
        panic!("Failed to get layout state");
    }
}

#[wasm_bindgen(module = "./src/hooks/use_current_layout.rs")]
extern {
    fn subscribe(selector: &Object, callback: extern "C" fn(JsValue)) -> Result<(), JsValue>;
}

#[wasm_bindgen(module = "./src/hooks/use_current_layout.rs")]
extern {
    fn unsubscribe(callback: extern "C" fn(JsValue));
}

#[wasm_bindgen(module = "./src/hooks/use_current_layout.rs")]
extern {
    fn update_panel_configs(id: &str, callback: extern "C" fn(Result<(), JsValue>)) -> Result<(), JsValue>;
}

async fn test_use_current_layout() {
    update_state().await;
    check_value(json!({ configById: { foo: { value: 1 } } }), 42).await;

    let selector = json!({ configById: { bar: { otherValue: 0 } } });
    update_state().await;
    check_value(selector, 0).await;

    let callback = move |state| {
        println!("Updated layout state: {:?}", state);
    };

    subscribe(json!({ configById: { foo: { value: 1 } } }), callback);
    update_state().await; // This will call the callback and update the state

    check_value(json!({ configById: { foo: { value: 2 } } }), 43).await;
}
```