```rust
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(msg: &str);
}

struct VisibilityState {
    visibility_state: &'static str,
    visibility_change_callback: Option<Callback<(), ()>>,
}

impl VisibilityState {
    pub fn new() -> Self {
        Self {
            visibility_state: "hidden",
            visibility_change_callback: None,
        }
    }

    pub fn set_visibility_state(&mut self, state: &str) {
        if state != self.visibility_state {
            self.visibility_state = state;
            if let Some(callback) = self.visibility_change_callback.take() {
                callback.call(&[]);
            }
        }
    }
}

#[wasm_bindgen]
pub struct UseVisibilityState {
    state: Rc<VisibilityState>,
    on_visibility_change: Option<Box<dyn FnMut(&str)>>,
}

#[wasm_bindgen]
impl UseVisibilityState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let visibility_state = VisibilityState::new();
        let state = Rc::new(visibility_state);
        let on_visibility_change = None;
        Self { state, on_visibility_change }
    }

    #[wasm_bindgen(getter)]
    pub fn current(&self) -> String {
        self.state.visibility_state.to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_on_visibility_change<F: FnMut(&str)>(&mut self, callback: F) {
        let callback_box = Box::new(callback);
        self.on_visibility_change = Some(Box::new(move |_| {
            if let Some(on_visibility_change) = self.on_visibility_change.take() {
                on_visibility_change(&self.state.visibility_state);
            }
        }));
    }

    pub fn update_visibility_state(&mut self, state: &str) {
        self.state.set_visibility_state(state);
        if let Some(on_visibility_change) = self.on_visibility_change.take() {
            on_visibility_change(&state);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=visibilitychange)]
    fn visibilitychange_callback() -> i32;
}

#[wasm_bindgen]
impl UseVisibilityState {
    pub fn add_visibility_change_listener<F: FnMut(&str)>(&mut self, callback: F) {
        let callback_box = Box::new(callback);
        unsafe { visibilitychange_callback().set(Some(Box::new(move |_| {
            if let Some(on_visibility_change) = self.on_visibility_change.take() {
                on_visibility_change(&visibilitychange_callback());
            }
        }))); }
    }
}
```