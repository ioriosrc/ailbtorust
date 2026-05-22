```rust
use std::time::{Duration, Instant};

use alitoken::prelude::*;
use alitoken::roscpp::MessageEvent;
use alitoken::rosidl_runtime_msgs::geometry_msgs::PoseArray as GeometryMsgs_PoseArray;
use alitoken::rosidl_runtime_msgs::geometry_msgs::TransformStamped as TransformStamped;
use alitoken::utils::quaternion::{vec4_to_orientation, Quaternion};

#[derive(Component)]
struct ThreeDeePanel;

#[derive(Component, Clone)]
struct MessageEventWrapper {
    topic: String,
    receive_time: Instant,
    message: MessageEvent<TransformStamped>,
}

impl MessageEventWrapper {
    fn new(topic: String, receive_time: Instant, message: MessageEvent<TransformStamped>) -> Self {
        Self { topic, receive_time, message }
    }
}

#[derive(Component, Clone)]
struct PoseArrayWrapper {
    header: Header,
    poses: Vec<Pose>,
}

#[derive(Component, Clone)]
struct Header {
    seq: u64,
    stamp: Time,
    frame_id: String,
}

#[derive(Component, Clone)]
struct Pose {
    position: Position,
    orientation: Quaternion,
}

#[derive(Component, Clone)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

fn main() -> Result<(), Error> {
    let topics = vec![
        Topic::new("baselink_path", "geometry_msgs/PoseArray"),
        Topic::new("sensor_path", "geometry_msgs/PoseArray"),
        Topic::new("sensor_path2", "geometry_msgs/PoseArray"),
        Topic::new("tf", "geometry_msgs/TransformStamped"),
    ];

    let tf1 = MessageEventWrapper {
        topic: "/tf".to_string(),
        receive_time: Instant::now(),
        message: MessageEvent {
            header: Header {
                seq: 0,
                stamp: Time { sec: 10, nsec: 0 },
                frame_id: "map".to_string(),
            },
            child_frame_id: "base_link".to_string(),
            transform: TransformStamped {
                translation: Position {
                    x: 1e7f64,
                    y: 0.0f64,
                    z: 0.0f64,
                },
                rotation: Quaternion::IDENTITY,
            },
            schema_name: "geometry_msgs/TransformStamped".to_string(),
            size_in_bytes: 0,
        },
    };

    let tf2 = MessageEventWrapper {
        topic: "/tf".to_string(),
        receive_time: Instant::now(),
        message: MessageEvent {
            header: Header {
                seq: 0,
                stamp: Time { sec: 10, nsec: 0 },
                frame_id: "base_link".to_string(),
            },
            child_frame_id: "sensor",
            transform: TransformStamped {
                translation: Position {
                    x: 0.0f64,
                    y: 0.0f64,
                    z: 1.0f64,
                },
                rotation: vec4_to_orientation(
                    Quaternion::rotate_z(&Quaternion::IDENTITY, &Quaternion::create(), std::f64::consts::PI / 2),
                ),
            },
            schema_name: "geometry_msgs/TransformStamped".to_string(),
            size_in_bytes: 0,
        },
    };

    let tf3 = MessageEventWrapper {
        topic: "/tf".to_string(),
        receive_time: Instant::now(),
        message: MessageEvent {
            header: Header {
                seq: 0,
                stamp: Time { sec: 10, nsec: 0 },
                frame_id: "base_link".to_string(),
            },
            child_frame_id: "sensor",
            transform: TransformStamped {
                translation: Position {
                    x: 0.0f64,
                    y: 5.0f64,
                    z: 1.0f64,
                },
                rotation: Quaternion::IDENTITY,
            },
            schema_name: "geometry_msgs/TransformStamped".to_string(),
            size_in_bytes: 0,
        },
    };

    let base_link_path = PoseArrayWrapper {
        header: Header {
            seq: 3,
            stamp: Time { sec: 0, nsec: 0 },
            frame_id: "base_link".to_string(),
        },
        poses: (0..10)
            .map(|i| Pose {
                position: Position {
                    x: 3.0f64,
                    y: i as f64 / 4.0f64,
                    z: 1.0f64,
                },
                orientation: make_orientation(i),
            })
            .collect(),
    };

    let sensor_path = PoseArrayWrapper {
        header: Header {
            seq: 3,
            stamp: Time { sec: 0, nsec: 0 },
            frame_id: "sensor".to_string(),
        },
        poses: (0..10)
            .map(|i| Pose {
                position: Position {
                    x: 2.0f64,
                    y: i as f64 / 4.0f64,
                    z: 0.0f64,
                },
                orientation: make_orientation(i),
            })
            .collect(),
    };

    let sensor_path2 = PoseArrayWrapper {
        header: Header {
            seq: 3,
            stamp: Time { sec: 0, nsec: 0 },
            frame_id: "sensor".to_string(),
        },
        poses: (0..10)
            .map(|i| Pose {
                position: Position {
                    x: -(i as f64) / 4.0f64,
                    y: 2.0f64,
                    z: 0.0f64,
                },
                orientation: make_orientation(i),
            })
            .collect(),
    };

    let fixture = Fixture::new(
        topics,
        HashMap::from([
            ("baselink_path", vec![base_link_path]),
            ("sensor_path", vec![sensor_path]),
            ("sensor_path2", vec![sensor_path2]),
            ("tf", vec![tf1, tf2, tf3]),
        ]),
        Vec::<Capability>::new(),
        ActiveData {
            current_time: Time::now() + Duration::from_secs(3),
        },
    );

    let panel_setup = PanelSetup {
        fixture,
    };

    panel_setup.run();
}
```