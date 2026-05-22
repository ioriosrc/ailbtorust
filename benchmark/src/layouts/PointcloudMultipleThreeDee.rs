```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CameraState {
    perspective: bool,
    distance: f64,
    phi: f64,
    theta_offset: f64,
    target_offset: [f64; 3],
    target: [f64; 3],
    target_orientation: [f64; 4],
    fovy: f64,
    near: f64,
    far: f64,
}

#[derive(Serialize, Deserialize)]
struct Scene;

#[derive(Serialize, Deserialize)]
struct Transforms;

#[derive(Serialize, Deserialize)]
struct Topics {
    pointcloud_0: PointCloudConfig;
}

#[derive(Serialize, Deserialize)]
struct PointCloudConfig {
    visible: bool,
    color_field: String,
    color_mode: String,
    color_map: String,
}

#[derive(Serialize, Deserialize)]
struct Layers;

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
struct FollowTf;

#[derive(Serialize, Deserialize)]
struct GlobalVariables;

#[derive(Serialize, Deserialize)]
struct UserNodes;

#[derive(Serialize, Deserialize)]
struct PlaybackConfig {
    speed: f64,
}

#[derive(Serialize, Deserialize)]
struct Layout {
    first: FirstLayout;
    second: SecondLayout;
}

#[derive(Serialize, Deserialize)]
struct FirstLayout {
    first: String;
    second: String;
    direction: Direction;
}

#[derive(Serialize, Deserialize)]
struct SecondLayout {
    first: FirstLayout;
    second: FirstLayout;
    direction: Direction;
}

#[derive(Serialize, Deserialize)]
enum Direction {
    Column,
    Row,
}

fn main() {
    // Your Rust code here
}
```