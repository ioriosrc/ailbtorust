```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KeyListener {
    handler: Option<Box<dyn Fn(&str) + 'static>>,
}

#[wasm_bindgen]
impl KeyListener {
    #[wasm_bindgen(constructor)]
    pub fn new(handler: Option<Box<dyn Fn(&str) + 'static>>) -> Self {
        Self { handler }
    }

    pub fn on_key_down(mut self, key: &str) {
        if let Some(ref handler) = self.handler {
            handler(key);
        }
    }

    pub fn on_key_up(mut self, key: &str) {
        // Placeholder for handling key up events
    }

    pub fn on_key_press(mut self, key: &str) {
        if let Some(ref handler) = self.handler {
            handler(key);
        }
    }

    #[wasm_bindgen(js_name = "addEventListener")]
    pub fn add_event_listener(&mut self, event_type: &str, callback: js_sys::Function) -> Result<(), JsValue> {
        use web_sys::{EventTarget, EventListenerOptions};

        let target = if let Some(ref handler) = self.handler {
            // Wrap the Rust closure in a JavaScript function
            let rust_callback = Closure::wrap(Box::new(move |event| {
                if let EventTarget::Element(target) = event.target() {
                    match target.name().as_ref() {
                        "input" | "textarea" | "contenteditable" => {}
                        _ => {
                            if let Some(ref handler) = self.handler {
                                handler(event.key());
                            }
                        },
                    }
                }
            }) as Box<dyn FnMut(web_sys::Event)>);

            EventTarget::Element(target.clone())
        } else {
            target.clone()
        };

        let options = EventListenerOptions::new().capture(false).once(true);
        target.add_event_listener_with_callback(event_type, callback.as_ref(), &options)?;

        Ok(())
    }

    #[wasm_bindgen(js_name = "removeEventListener")]
    pub fn remove_event_listener(&mut self, event_type: &str) -> Result<(), JsValue> {
        if let Some(ref handler) = self.handler {
            // Find the corresponding JavaScript closure and remove it
            // This is a simplified example and may require more complex logic depending on your specific use case
        }

        Ok(())
    }
}
```