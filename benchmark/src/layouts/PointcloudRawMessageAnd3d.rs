```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConfigById {
    pub "RawMessages!41teyny": RawMessageConfig,
    pub "3D!2cny167": Pointcloud3dConfig,
}

#[derive(Deserialize, Serialize)]
pub struct RawMessageConfig {
    pub diff_enabled: bool;
    pub diff_method: String;
    pub diff_topic_path: String;
    pub show_full_message_for_diff: bool;
    pub topic_path: String;
}

#[derive(Deserialize, Serialize)]
pub struct Pointcloud3dConfig {
    pub camera_state: CameraState,
    pub scene: Option<Scene>,
    pub transforms: Option<Transforms>,
    pub topics: Topics,
    pub layers: Option<Layers>,
    pub publish: PublishConfig,
    pub follow_tf: String,
}

#[derive(Deserialize, Serialize)]
pub struct CameraState {
    // Define the fields of your camera state
}

#[derive(Deserialize, Serialize)]
pub struct Scene {
    // Define the fields of your scene
}

#[derive(Deserialize, Serialize)]
pub struct Transforms {
    // Define the fields of your transforms
}

#[derive(Deserialize, Serialize)]
pub struct Topics {
    pub "pointcloud_0": PointcloudTopicConfig,
}

#[derive(Deserialize, Serialize)]
pub struct PointcloudTopicConfig {
    pub visible: bool;
    pub color_field: String;
    pub color_mode: String;
    pub color_map: String;
}

#[derive(Deserialize, Serialize)]
pub struct Layers {
    // Define the fields of your layers
}

#[derive(Deserialize, Serialize)]
pub struct PublishConfig {
    // Define the fields of your publish config
}

#[derive(Deserialize, Serialize)]
pub struct PlaybackConfig {
    pub speed: f64;
}

#[derive(Deserialize, Serialize)]
pub struct Layout {
    pub first: String,
    pub second: String,
    pub direction: String;
}
```