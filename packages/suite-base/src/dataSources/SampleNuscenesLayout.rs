```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ImageMode {
    image_topic: String,
    calibration_topic: String,
    synchronize: bool,
    rotation: i32,
    annotations: Annotations,
}

#[derive(Serialize, Deserialize)]
struct Annotations {
    #[serde(default)]
    visible: bool,

    // Add more fields as needed
}

#[derive(Serialize, Deserialize)]
struct CameraState {
    distance: f64,
    perspective: bool,
    phi: f64,
    target: [f64; 3],
    target_offset: [f64; 3],
    target_orientation: [f64; 4],
    theta_offset: f64,
    fovy: f64,
    near: f64,
    far: f64,
}

#[derive(Serialize, Deserialize)]
struct FollowMode {
    type_: String,
}

#[derive(Serialize, Deserialize)]
struct Scene {
    // Define the fields for the scene
}

#[derive(Serialize, Deserialize)]
struct Transforms {
    // Define the fields for the transforms
}

#[derive(Serialize, Deserialize)]
struct Topics {
    #[serde(default)]
    visible: bool,

    // Add more fields as needed
}

#[derive(Serialize, Deserialize)]
struct Layers {
    // Define the fields for the layers
}

#[derive(Serialize, Deserialize)]
struct Publish {
    type_: String,
    pose_topic: String,
    point_topic: String,
    pose_estimate_topic: String,
    pose_estimate_x_deviation: f64,
    pose_estimate_y_deviation: f64,
    pose_estimate_theta_deviation: f64,
}

#[derive(Serialize, Deserialize)]
struct Plot {
    paths: Vec<Path>,
    min_y_value: Option<f64>,
    max_y_value: Option<f64>,
    show_legend: bool,
    is_synced: bool,
    x_axis_val: String,
    show_x_axis_labels: bool,
    show_y_axis_labels: bool,
    legend_display: String,
    show_plot_values_in_legend: bool,
    sidebar_dimension: i32,
   lichtblick_panel_title: String,
}

#[derive(Serialize, Deserialize)]
struct Path {
    value: String,
    enabled: bool,
    timestamp_method: String,
}

#[derive(Serialize, Deserialize)]
struct StateTransitions {
    paths: Vec<Path>,
    is_synced: bool,
}

#[derive(Serialize, Deserialize)]
struct DiagnosticSummary {
    min_level: i32,
    pinned_ids: Vec<String>,
    hardware_id_filter: String,
    topic_to_render: String,
    sort_by_level: bool,
}

#[derive(Serialize, Deserialize)]
struct DiagnosticStatusPanel {
    topic_to_render: String,
    collapsed_sections: Vec<String>,
    selected_hardware_id: String,
    split_fraction: f64,
}

#[derive(Serialize, Deserialize)]
struct SourceInfo;

#[derive(Serialize, Deserialize)]
struct RawMessages {
    topic_path: String,
    diff_topic_path: String,
    diff_method: String,
    diff_enabled: bool,
    show_full_message_for_diff: bool,
    auto_expand_mode: String,
}

#[derive(Serialize, Deserialize)]
struct Tab {
    active_tab_idx: i32,
    tabs: Vec<TabItem>,
}

#[derive(Serialize, Deserialize)]
struct TabItem {
    title: String,
    layout: Layout,
}

#[derive(Serialize, Deserialize)]
struct Layout {
    first: Vec<Pane>,
    second: Vec<Pane>,
}

#[derive(Serialize, Deserialize)]
struct Pane {
    first: Vec<Node>,
    second: Vec<Node>,
    direction: String,
    split_percentage: f64,
}

#[derive(Serialize, Deserialize)]
struct Node {
    title: String,
    layout: Layout,
}

fn main() {
    // Load the configuration from the provided JSON string
    let json_string = r#"
    {
        "globalVariables": {},
        "userNodes": {},
        "playbackConfig": {
            "speed": 1
        },
        "layout": "Tab!1xyw5ix"
    }
    "#;

    // Deserialize the JSON string into a Rust struct
    let config: Config = serde_json::from_str(json_string).unwrap();

    // Access and print specific fields from the configuration
    println!("Global Variables: {:?}", config.global_variables);
    println!("User Nodes: {:?}", config.user_nodes);
    println!("Playback Configuration: {:?}", config.playback_config);
    println!("Layout: {:?}", config.layout);

    // Process or use the configuration as needed
}
```