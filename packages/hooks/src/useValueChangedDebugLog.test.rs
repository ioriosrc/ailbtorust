```rust
use std::cell::RefCell;
use wasm_bindgen::{JsValue, JsCast};
use log::debug;

#[wasm_bindgen]
pub fn use_value_changed_debug_log<T>(initial_val: T, msg: &str) {
    let ref_cell = RefCell::new(initial_val);

    // Simulate a subscription to the value change event
    let subscription = wasm_bindgen::EventTarget::from(&ref_cell).add_event_listener_with_callback(
        "change",
        move |event| {
            debug!("Value changed to {:?}", event.target().get_property::<JsValue>("value"));
            ref_cell.replace(event.target().get_property::<JsValue>("value").as_str());
        },
    );

    // Simulate a change to the value
    let val = 2;
    ref_cell.set(val);

    // Clean up the subscription when done
    subscription.remove_event_listener("change", move |_event| {});
}
```