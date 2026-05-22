```rust
use bevy::prelude::*;
use lichtblick::{suite::MessageEvent, suite_base::players::Topic};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

#[derive(Debug)]
struct PanelFixture {
    topics: Vec<Topic>,
    frame: HashMap<String, Vec<MessageEvent<TransformStamped>>>,
    capabilities: Vec<Capability>,
    active_data: ActiveData,
}

#[derive(Debug)]
struct Capability {
    // Define the capability fields
}

#[derive(Debug)]
struct ActiveData {
    currentTime: f64,
}

fn setup_system(mut commands: Commands) {
    let topics = vec![Topic {
        name: "/tf".to_string(),
        schema_name: "geometry_msgs/TransformStamped".to_string(),
    }];
    let tf1 = MessageEvent {
        topic: "/tf".to_string(),
        receive_time: Time::from_seconds_f64(10.0),
        message: TransformStamped {
            header: Header {
                seq: 0,
                stamp: Time::from_seconds_f64(0.0),
                frame_id: "map".to_string(),
            },
            child_frame_id: "base_link".to_string(),
            transform: Transform {
                translation: Vector3::new(1e7, 0.0, 0.0),
                rotation: Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };
    let fixture = PanelFixture {
        topics,
        frame: HashMap::from([("tf".to_string(), vec![tf1])]),
        capabilities: Vec::new(),
        active_data: ActiveData {
            currentTime: 0.0,
        },
    };

    commands.insert_one(fixture);
}
```