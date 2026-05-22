```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Layer {
    layerId: String,
    color: String,
}

#[derive(Serialize, Deserialize)]
struct CameraState {
    perspective: bool,
    distance: f64,
    phi: f64,
    thetaOffset: f64,
    targetOffset: [f64; 3],
    fovy: f64,
}

#[derive(Serialize, Deserialize)]
struct GridLayer {
    layerId: String,
    color: String,
}

#[derive(Serialize, Deserialize)]
struct CameraViewConfig {
    id: String,
    layers: Vec<GridLayer>,
    followTf: String,
    cameraState: CameraState,
}

#[derive(Serialize, Deserialize)]
struct LayoutSection {
    first: String,
    second: String,
    direction: String,
}

#[derive(Serialize, Deserialize)]
struct Layout {
    first: LayoutSection,
    second: LayoutSection,
    direction: String,
    splitPercentage: f64,
}

fn main() {
    let config = include_str!("config.json");
    let parsed_config: serde_json::Value = serde_json::from_str(config).unwrap();

    // Process the config data here
}
```

Note: The `include_str!` macro is used to read the JSON file into a string, and then it's deserialized into a `serde_json::Value`. You would typically replace this with your actual configuration logic in Rust.