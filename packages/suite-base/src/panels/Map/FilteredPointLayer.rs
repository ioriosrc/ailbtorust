```rust
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

// Define the CircleMarker struct and related components

struct PointMarker {
    message_event: Option<MessageEvent<NavSatFixMsg>>,
    // Add other necessary fields for a leaflet marker
}

fn get_accuracy(message_event: &NavSatFixMsg) -> Option<f64> {
    // Implement the logic to determine the accuracy based on the message event
    unimplemented!()
}

#[derive(Debug)]
struct NavSatFixMsg {
    // Define the fields of NavSatFixMsg
    latitude: f64,
    longitude: f64,
    // Other fields...
}

type MessageEvent<T> = Box<dyn Fn() -> T>;

fn main() {
    // Implement the FilteredPointLayer function in Rust, using the above structures and components

    // Example usage of FilteredPointLayer
    let points: Vec<MessageEvent<NavSatFixMsg>> = vec![];
    let bounds = BoundingBox::new();
    let map = Map::new();

    let marker_layer = FilteredPointLayer {
        nav_sat_message_events: points,
        bounds,
        map,
        color: "#ff0000",
        hover_color: "#ffff00",
        show_accuracy: true,
        on_hover: Some(|event| println!("Hovered: {:?}", event)),
        on_click: Some(|event| println!("Clicked: {:?}", event)),
    };
}
```