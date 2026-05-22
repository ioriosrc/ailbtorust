```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CameraState {
    perspective: bool,
    distance: f64,
    phi: f64,
    theta_offset: f64,
    target_offset: Vec3<f64>,
    target: Vec3<f64>,
    target_orientation: Quaternion<f64>,
    fovy: f64,
    near: f64,
    far: f64,
}

#[derive(Serialize, Deserialize)]
struct Layer {
    visible: bool,
    frame_locked: bool,
    label: String,
    instance_id: String,
    layer_id: String,
    size: i32,
    divisions: i32,
    lineWidth: f64,
    color: String,
    position: Vec3<f64>,
    rotation: Vec3<f64>,
    order: i32,
}

#[derive(Serialize, Deserialize)]
struct Camera {
    layers: HashMap<String, Layer>,
    camera_state: CameraState,
    follow_mode: String,
    scene: HashMap<String, Vec3<f64>>,
    transforms: HashMap<String, Quaternion<f64>>,
    topics: HashMap<String, String>,
    publish: PublishConfig,
    image_mode: ImageMode,
}

#[derive(Serialize, Deserialize)]
struct PublishConfig {
    type_: String,
    pose_topic: String,
    point_topic: String,
    pose_estimate_topic: String,
    pose_estimate_x_deviation: f64,
    pose_estimate_y_deviation: f64,
    pose_estimate_theta_deviation: f64,
}

#[derive(Serialize, Deserialize)]
struct ImageMode {
    // Define the image mode properties
}

fn main() {
    let config = Config {
        config_by_id: ConfigById {
            "3D!18i6zy7": Camera {
                layers: HashMap::new(),
                camera_state: CameraState {
                    perspective: true,
                    distance: 19.99999999999993,
                    phi: 59.99999999999997,
                    theta_offset: 45.00000000000001,
                    target_offset: Vec3::new(0.047509182912022815, -0.009501662599111366, -3.851859888774472e-34),
                    target: Vec3::new(0, 0, 0),
                    target_orientation: Quaternion::identity(),
                    fovy: 45,
                    near: 0.5,
                    far: 5000,
                },
                follow_mode: "follow-pose",
                scene: HashMap::new(),
                transforms: HashMap::new(),
                topics: HashMap::new(),
                publish: PublishConfig {
                    type_: String::from("point"),
                    pose_topic: String::from("/move_base_simple/goal"),
                    point_topic: String::from("/clicked_point"),
                    pose_estimate_topic: String::from("/initialpose"),
                    pose_estimate_x_deviation: 0.5,
                    pose_estimate_y_deviation: 0.5,
                    pose_estimate_theta_deviation: 0.26179939,
                },
                image_mode: ImageMode {},
            },
            "RawMessages!os6rgs": Camera {
                layers: HashMap::new(),
                camera_state: CameraState {
                    perspective: true,
                    distance: 20,
                    phi: 60,
                    target: Vec3::new(0, 0, 0),
                    target_offset: Vec3::new(0, 0, 0),
                    target_orientation: Quaternion::identity(),
                    fovy: 45,
                    near: 0.5,
                    far: 5000,
                },
                follow_mode: "follow-pose",
                scene: HashMap::new(),
                transforms: HashMap::new(),
                topics: HashMap::new(),
                publish: PublishConfig {
                    type_: String::from("point"),
                    pose_topic: String::from("/move_base_simple/goal"),
                    point_topic: String::from("/clicked_point"),
                    pose_estimate_topic: String::from("/initialpose"),
                    pose_estimate_x_deviation: 0.5,
                    pose_estimate_y_deviation: 0.5,
                    pose_estimate_theta_deviation: 0.26179939,
                },
                image_mode: ImageMode {},
            },
        },
        global_variables: HashMap::new(),
        user_nodes: HashMap::new(),
        playback_config: PlaybackConfig {
            speed: 1.0,
        },
        layout: Layout {
            first: String::from("3D!18i6zy7"),
            second: {
                first: String::from("Image!3mnp456"),
                second: String::from("RawMessages!os6rgs"),
                direction: Direction::Column,
                split_percentage: 30.0,
            },
            direction: Direction::Row,
            split_percentage: 70.0,
        },
    };

    // Serialize the config to JSON
    let serialized_config = serde_json::to_string(&config).unwrap();
    println!("{}", serialized_config);
}
```