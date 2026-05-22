```rust
use winit::event::{ElementState, Event, MouseButton};

use crate::{
    components::ThreeDeePanel,
    models::OccupancyGrid,
    services::TopicsService,
    utils::*,
};

/// Component that renders a 3D occupancy grid using the Lichtenblick suite.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let element = document().query_selector("body")?.unwrap();
    element.set_inner_html("");

    // Set up the topics service
    let topics_service = TopicsService::new();

    // Create a 3D occupancy grid panel
    let three_dee_panel = ThreeDeePanel::new(&topics_service, {
        follow_tf: "base_link",
        topics: {
            "/costmap": {
                visible: true,
                color_mode: ColorMode::Costmap,
            } as LayerSettingsOccupancyGrid,
            "/custom": {
                visible: true,
            } as LayerSettingsOccupancyGrid,
        },
        layers: {
            grid: { layer_id: "foxglove.Grid" },
        },
        camera_state: {
            distance: 13.5,
            perspective: true,
            phi: 0.1,
            target_offset: [0.25, -0.5, 0],
            theta_offset: 0,
            fovy: deg_to_rad(0.75),
            near: 0.01,
            far: 5000,
            target: [0, 0, 0],
            target_orientation: [0, 0, 0, 1],
        },
    });

    // Append the panel to the body
    document().body()?.append(&three_dee_panel);

    Ok(())
}

// Event handler for mouse events
fn on_mouse_event(event: Event) -> JsValue {
    match event {
        Event::MouseButtonInput {
            button: MouseButton::Left,
            state: ElementState::Pressed,
            ..
        } => handle_left_click(),
        _ => JsValue::null(),
    }
}

// Handle left click to inspect objects
fn handle_left_click() {
    // Add your logic here to inspect the objects in the 3D occupancy grid
}
```