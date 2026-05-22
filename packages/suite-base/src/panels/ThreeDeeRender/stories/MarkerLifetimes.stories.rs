```rust
use rustic::prelude::*;
use rustic::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct TransformStamped {
    header: Header,
    transform: Transform,
}

#[derive(Debug)]
struct Header {
    seq: u32,
    stamp: Time,
    frame_id: String,
}

#[derive(Debug)]
struct Transform {
    translation: Vector3<f64>,
    rotation: Quaternion<f64>,
}

impl From<geometry_msgs::msg::TransformStamped> for TransformStamped {
    fn from(msg: geometry_msgs::msg::TransformStamped) -> Self {
        Self {
            header: Header {
                seq: msg.header.seq,
                stamp: Time {
                    sec: msg.header.stamp.sec,
                    nsec: msg.header.stamp.nanosec,
                },
                frame_id: msg.header.frame_id.clone(),
            },
            transform: Transform {
                translation: Vector3::new(msg.transform.translation.x, msg.transform.translation.y, msg.transform.translation.z),
                rotation: Quaternion::from_euler_angles(
                    msg.transform.rotation.roll,
                    msg.transform.rotation.pitch,
                    msg.transform.rotation.yaw,
                ),
            },
        }
    }
}

struct MessageEvent<T> {
    topic: String,
    receive_time: Time,
    message: T,
    schema_name: String,
    size_in_bytes: usize,
}

impl From<geometry_msgs::msg::TransformStamped> for MessageEvent<TransformStamped> {
    fn from(msg: geometry_msgs::msg::TransformStamped) -> Self {
        Self {
            topic: msg.topic.clone(),
            receive_time: Time {
                sec: msg.header.stamp.sec,
                nsec: msg.header.stamp.nanosec,
            },
            message: TransformStamped::from(msg),
            schema_name: msg.schema_name.clone(),
            size_in_bytes: 0, // Assuming the size of the TransformStamped is 0
        }
    }
}

struct TestColors {
    MARKER_GREEN1: String,
    MARKER_GREEN2: String,
    MARKER_GREEN3: String,
    MARKER_RED1: String,
    MARKER_RED2: String,
    MARKER_RED3: String,
}

impl Default for TestColors {
    fn default() -> Self {
        Self {
            MARKER_GREEN1: "#ff0000".to_string(),
            MARKER_GREEN2: "#00ff00".to_string(),
            MARKER_GREEN3: "#0000ff".to_string(),
            MARKER_RED1: "#ff00ff".to_string(),
            MARKER_RED2: "#00ff00".to_string(),
            MARKER_RED3: "#ff0000".to_string(),
        }
    }
}

struct VEC3_ZERO;

impl From<geometry_msgs::msg::Point> for VEC3_ZERO {
    fn from(msg: geometry_msgs::msg::Point) -> Self {
        Self
    }
}

fn rad2deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

fn makePass(id: i32, frame_id: String, stamp: Time, color_hex: String, pose: Transform) -> MessageEvent<TransformStamped> {
    let transform_stamped = TransformStamped {
        header: Header {
            seq: id as u32,
            stamp,
            frame_id,
        },
        transform,
    };

    MessageEvent {
        topic: "markers".to_string(),
        receive_time: stamp,
        message: transform_stamped,
        schema_name: "visualization_msgs/Marker".to_string(),
        size_in_bytes: 0, // Assuming the size of the TransformStamped is 0
    }
}

fn makeFail(id: i32, frame_id: String, stamp: Time, color_hex: String, lifetime: Duration, pose: Transform, description: String) -> MessageEvent<TransformStamped> {
    let transform_stamped = TransformStamped {
        header: Header {
            seq: id as u32,
            stamp,
            frame_id,
        },
        transform,
    };

    MessageEvent {
        topic: "markers".to_string(),
        receive_time: stamp,
        message: transform_stamped,
        schema_name: "visualization_msgs/Marker".to_string(),
        size_in_bytes: 0, // Assuming the size of the TransformStamped is 0
    }
}

fn useDelayedFixture(topics: Vec<(String, String)>, frame: HashMap<String, Vec<MessageEvent<TransformStamped>>>) -> Fixture {
    let mut fixture = Fixture::new();

    for topic in topics {
        fixture.add_topic(topic.0.clone(), vec![topic.1]);
    }

    fixture.add_capabilities([]);

    fixture.set_active_data(ActiveData {
        current_time: Time {
            sec: 2,
            nsec: 0,
        },
    });

    fixture
}

#[derive(Clone, Debug)]
struct PanelSetup {
    fixture: Fixture,
}

impl PanelSetup {
    fn new(fixture: Fixture) -> Self {
        Self { fixture }
    }

    fn render(&self) -> Component {
        // Implement the rendering logic here
        unimplemented!()
    }
}

#[derive(Debug)]
struct ThreeDeePanel {
    override_config: OverrideConfig,
}

#[derive(Debug, Default)]
struct OverrideConfig {
    follow_tf: String,
    camera_state: CameraState,
    topics: HashMap<String, Topic>,
}

#[derive(Debug, Default)]
struct Topic {
    visible: bool,
}

#[derive(Debug, Default)]
struct CameraState {
    distance: f64,
    perspective: bool,
    phi: f64,
    target_offset: Vector3<f64>,
    theta_offset: f64,
    fovy: f64,
    near: f64,
    far: f64,
    target: Vector3<f64>,
    target_orientation: Quaternion<f64>,
}

fn main() {
    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let fail1 = makeFail(1, "missing".to_string(), Time::from_nanos(1), "#ff0000".to_string(), Duration::from_secs(1), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    }, "No transform(s) for coordinate frame \"missing\"");

    let fail2 = makeFail(2, "sensor".to_string(), Time::from_nanos(1), "#ff0000".to_string(), Duration::from_secs(1), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    }, "No transform(s) for coordinate frame \"sensor\"");

    let fail3 = makeFail(3, "sensor".to_string(), Time::from_nanos(1), "#ff0000".to_string(), Duration::from_secs(1), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    }, "No transform(s) for coordinate frame \"sensor\"");

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(0),
                frame_id: "base_link".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pass1 = makePass(1, "base_link".to_string(), Time::from_nanos(10_000_000_000), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass2 = makePass(2, "sensor".to_string(), Time::from_nanos(2), "#00ff00".to_string(), Transform {
        translation: Vector3::new(1.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass3 = makePass(3, "sensor".to_string(), Time::from_nanos(1), "#0000ff".to_string(), Transform {
        translation: Vector3::new(2.0, 0.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass4 = makePass(4, "base_link".to_string(), Time::from_nanos(0), "#ff0000".to_string(), Transform {
        translation: Vector3::new(-1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass5 = makePass(5, "base_link".to_string(), Time::from_nanos(3), "#00ff00".to_string(), Transform {
        translation: Vector3::new(0.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let pass6 = makePass(6, "base_link".to_string(), Time::from_nanos(2), "#0000ff".to_string(), Transform {
        translation: Vector3::new(1.0, -1.0, 0.0),
        rotation: Quaternion::identity(),
    });

    let topics = vec![
        ("/markers".to_string(), "visualization_msgs/Marker".to_string()),
        ("/tf".to_string(), "geometry_msgs/TransformStamped".to_string()),
    ];

    let tf1 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),
        receive_time: Time::from_nanos(10_000_000_000),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_nanos(10_000_000_000),
                frame_id: "map".to_string(),
            },
            transform: Transform {
                translation: Vector3::new(-1.0, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent<TransformStamped> {
        topic: "/tf".to_string(),