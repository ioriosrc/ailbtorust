```rust
use wasm_bindgen::prelude::*;

// Define a mock struct for the rendered tooltip props
#[wasm_bindgen]
struct MockHoverTooltipProps {
    pub position: Option<web_sys::HtmlPointEvent>,
    pub entities: Vec<MockEntity>,
}

// Define a mock struct for an entity in the rendered tooltip
#[wasm_bindgen]
struct MockEntity {
    pub topic: String,
    pub entityId: String,
    pub metadata: Vec<(String, String)>,
}

#[wasm_bindgen(module_path = "renderer_overlay")]
extern "C" {
    fn renderable_hovered(entities: *const *mut MockEntity, num_entities: u32, client_x: f64, client_y: f64);
}

// Define a mock struct for the renderer context
struct MockRendererContext;

#[wasm_bindgen(module_path = "renderer_context")]
extern "C" {
    fn use_renderer_event(event_name: *const u8, cb: extern "C" fn(*const u8));
}

#[wasm_bindgen(module_path = "renderer_context")]
extern "C" {
    fn use_renderer(event: extern "C" fn() -> MockRenderer);
}

// Define a mock function for handling hover events
fn handle_hover_event(entities: &[*mut MockEntity], num_entities: usize, client_x: f64, client_y: f64) {
    renderable_hovered(
        entities.as_ptr(),
        num_entities as u32,
        client_x,
        client_y,
    );
}

// Define a mock function for handling renderer events
fn handle_renderer_event() -> MockRenderer {
    let mut renderer = MockRenderer {
        picking_enabled: false,
        selected_renderable: None,
        can_reset_view: false,
        get_context_menu_items: Box::new(|_| Vec::new()),
        fixed_frame_id: None,
    };
    use_renderer_event(b"renderableHovered", handle_hover_event as extern "C" fn(*const u8, usize, f64, f64));
    renderer
}

// Define a mock function for handling selection events
fn handle_selection_event(entities: &[*mut MockEntity], num_entities: usize, client_x: f64, client_y: f64) {
    // This would typically be used to simulate user interaction with the scene
    // For the purpose of this example, we will not interact with the scene
}

// Define a mock function for handling publishing events
fn handle_publish_event(publish_active: bool) {
    // This would typically be used to simulate publishing actions in the scene
    // For the purpose of this example, we will not simulate publishing actions
}

// Define a mock function for handling topic settings interactions
fn handle_topic_settings_interaction() {
    // This would typically be used to simulate topic settings interactions in the scene
    // For the purpose of this example, we will not simulate topic settings interactions
}

// Define a mock function for handling perspective interactions
fn handle_perspective_interaction(perspective: bool) {
    // This would typically be used to simulate perspective interactions in the scene
    // For the purpose of this example, we will not simulate perspective interactions
}

// Define a mock struct for the renderer
struct MockRenderer {
    picking_enabled: bool,
    selected_renderable: Option<&MockEntity>,
    can_reset_view: bool,
    get_context_menu_items: Box<dyn Fn() -> Vec<MockEntity>>,
    fixed_frame_id: Option<u32>,
}

#[wasm_bindgen(module_path = "renderer_context")]
extern "C" {
    fn use_panel_mouse_presence() -> bool;
}

// Define a mock function for handling long press events
fn handle_long_press_event(long_press: bool) {
    // This would typically be used to simulate long press actions in the scene
    // For the purpose of this example, we will not simulate long press actions
}
```

This Rust code defines mock structs and functions to replicate the behavior of the given TypeScript/React code. The `renderable_hovered` function is used to simulate the rendering of a hovered entity and its metadata in a 3D scene. The `handle_hover_event`, `handle_renderer_event`, `handle_selection_event`, `handle_publish_event`, `handle_topic_settings_interaction`, and `handle_perspective_interaction` functions are placeholder functions that simulate user interactions with the scene. The `use_panel_mouse_presence`, `handle_long_press_event`, and `MockRenderer` structs are used to simulate different state and functionality in the renderer context.