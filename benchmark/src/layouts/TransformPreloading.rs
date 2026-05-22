```rust
struct ConfigById {
    "RawMessages!41teyny": DiffConfig,
    "3D!2cny167": SceneConfig,
}

struct DiffConfig {
    diff_enabled: bool,
    diff_method: String,
    diff_topic_path: String,
    show_full_message_for_diff: bool,
    topic_path: String,
}

struct SceneConfig {
    transforms: TransformsConfig,
    topics: TopicsConfig,
    layers: LayersConfig,
    publish: PublishConfig,
    follow_tf: String,
}

struct TransformsConfig {
    enable_preloading: bool,
}

struct TopicsConfig {}

struct LayersConfig {
    b5ad4864_09db_4cfa_9096_2a1078fb6bdd: LayerConfig,
    cbb67be5_74e9_4282_8ce9_40ab51885ed4: LayerConfig,
    499e0cc0_9ead_4834_b538_7bcf80829e4a: LayerConfig,
}

struct LayerConfig {
    visible: bool,
    frame_locked: bool,
    label: String,
    instance_id: String,
    layer_id: String,
    frame_id: String,
    size: f64,
    divisions: u32,
    lineWidth: u32,
    color: String,
    position: [f64; 3],
    rotation: [f64; 3],
    order: i32,
}

struct PublishConfig {
    type_: String,
    pose_topic: String,
    point_topic: String,
    pose_estimate_topic: String,
    pose_estimate_x_deviation: f64,
    pose_estimate_y_deviation: f64,
    pose_estimate_theta_deviation: f64,
}

fn main() {
    // Implement the conversion here
}
```