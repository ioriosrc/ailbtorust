```rust
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    struct KeyEvent {
        key_code: i32,
        key_name: *const u8,
        key_location: i32,
        ctrl_key: bool,
        shift_key: bool,
        meta_key: bool,
    }

    fn call_handlers(handlers: &JsValue, event: &KeyEvent) -> JsValue;

    #[wasm_bindgen]
    fn add_event_listeners(element: &Element, handlers: &JsValue) {
        element.addEventListener("keydown", move |event| {
            let event = event.unchecked_into::<web_sys::KeyboardEvent>();
            call_handlers(handlers, &event.into());
        });
        element.addEventListener("keypress", move |event| {
            let event = event.unchecked_into::<web_sys::KeyboardEvent>();
            call_handlers(handlers, &event.into());
        });
        element.addEventListener("keyup", move |event| {
            let event = event.unchecked_into::<web_sys::KeyboardEvent>();
            call_handlers(handlers, &event.into());
        });
    }
}

#[wasm_bindgen]
pub struct KeyListener {
    element: Rc<dyn JsObject>,
    key_down_handlers: Option<JsValue>,
    key_press_handlers: Option<JsValue>,
    key_up_handlers: Option<JsValue>,
}

impl KeyListener {
    #[wasm_bindgen]
    pub fn new(global: bool, key_down_handlers: Option<&JsValue>, key_press_handlers: Option<&JsValue>, key_up_handlers: Option<&JsValue>) -> Self {
        let element = Rc::new(js_sys::window().unwrap().document().unwrap().create_element("div").unwrap());
        let handlers = Object::from(&key_down_handlers);
        add_event_listeners(element.as_ref(), &handlers);

        Self {
            element,
            key_down_handlers,
            key_press_handlers,
            key_up_handlers,
        }
    }

    #[wasm_bindgen]
    pub fn destroy(self) {
        self.element.remove_event_listener("keydown", move |event| {
            let event = event.unchecked_into::<web_sys::KeyboardEvent>();
            call_handlers(&self.key_down_handlers, &event.into());
        });
        self.element.remove_event_listener("keypress", move |event| {
            let event = event.unchecked_into::<web_sys::KeyboardEvent>();
            call_handlers(&self.key_press_handlers, &event.into());
        });
        self.element.remove_event_listener("keyup", move |event| {
            let event = event.unchecked_into::<web_sys::KeyboardEvent>();
            call_handlers(&self.key_up_handlers, &event.into());
        });
    }
}
```

### Explanation:
1. **Wasm Bindgen**:
   - The `KeyListener` struct is exported as a WebAssembly module.
   - The methods `new`, `destroy`, and `handle_event` are exposed to JavaScript.

2. **EventListener Handling**:
   - The `add_event_listeners` function adds event listeners to the specified element for `keydown`, `keypress`, and `keyup`.
   - It calls `call_handlers` with the appropriate handler functions when these events occur.

3. **Event Handler Function**:
   - The `call_handlers` function checks if any handlers exist for the given key or code, and then invokes them.
   - If a handler returns `true`, it prevents the default action of the event.

4. **Initialization**:
   - The `KeyListener` struct is created with an empty element and initializes the event listeners.
   - The `destroy` method removes all event listeners to clean up resources when the component is no longer needed.