```rust
use wasm_bindgen::prelude::*;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    fn log(s: &str);
}

#[wasm_bindgen(module_path = "./src/verticalbars.tsx", namespace = "global")]
pub extern "C" {
    pub fn update_bars();
}

#[wasm_bindgen(module_path = "./src/verticalbars.tsx", namespace = "global")]
pub extern "C" {
    pub fn subscribe_to_x_scale_change(coordinator: Rc<dyn YourCoordinator>);
}

#[wasm_bindgen(module_path = "./src/verticalbars.tsx", namespace = "global")]
pub extern "C" {
    pub fn unsubscribe_from_x_scale_change(coordinator: Rc<dyn YourCoordinator>);
}

// Replace `YourCoordinator` with the actual type of your coordinator
struct YourCoordinator;

impl YourCoordinator {
    fn on(&self, event_name: &str, callback: impl Fn(&Scale) -> ()) {
        // Implementation to handle event notifications from the coordinator
    }

    fn off(&self, event_name: &str, callback: impl Fn(&Scale)) {
        // Implementation to remove event notifications from the coordinator
    }
}

#[wasm_bindgen(module_path = "./src/verticalbars.tsx", namespace = "global")]
pub struct VerticalBarsProps {
    coordinator: Rc<dyn YourCoordinator>,
    hover_component_id: String,
    xAxis_is_playback_time: bool,
}
```