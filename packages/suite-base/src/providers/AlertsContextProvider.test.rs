```rust
use wasm_bindgen::prelude::*;
use web_sys::{self as window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console::log_1(&JsValue::from("Starting application"));
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    pub fn set_alert(tag: &str, alert: &web_sys::JsonValue);
    pub fn clear_alerts();
}

fn main() -> Result<(), JsValue> {
    window().set_alert("primary", web_sys::JsonValue::from("{\"severity\": \"info\", \"message\": \"Main Alert\"}"))?;
    window().set_alert("secondary", web_sys::JsonValue::from("{\"severity\": \"warning\", \"message\": \"Secondary Alert\"}"))?;

    while !window().alerts().is_empty() {
        window().clear_alerts()?;
    }

    Ok(())
}
```