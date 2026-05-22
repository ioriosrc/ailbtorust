```rust
use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, EventTarget};

#[wasm_bindgen]
pub fn use_panel_mouse_presence(ref: &web_sys::ElementRef) -> bool {
    let mut present = false;

    let listener = Closure::new(move |ev: MouseEvent| {
        if !ref.is_null() {
            if ev.type_().as_str() == "mouseenter" {
                present = true;
            } else {
                present = false;
            }
        }
    });

    web_sys::event_target(ref.as_ref())
        .add_event_listener_with_callback("mouseenter", listener.clone_unchecked(), true)
        .unwrap();
    web_sys::event_target(ref.as_ref())
        .add_event_listener_with_callback("mouseleave", listener.clone_unchecked(), true)
        .unwrap();

    move || {
        web_sys::event_target(ref.as_ref())
            .remove_event_listener_with_callback("mouseenter", listener.clone_unchecked())
            .unwrap();
        web_sys::event_target(ref.as_ref())
            .remove_event_listener_with_callback("mouseleave", listener.clone_unchecked())
            .unwrap();
    }
}
```

Note: This is a simplified version of the original TypeScript code. Rust's event handling mechanism is different from JavaScript's, so some parts may require adjustments or changes to work properly in Rust. Additionally, this code assumes that `ref` and `PANEL_ROOT_CLASS_NAME` are properly defined and accessible within the context where this function is used.