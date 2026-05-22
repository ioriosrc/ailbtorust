```rust
use std::vec::Vec;

use serde_json::{Value, Map};
use serde_derive::Serialize;
use serde_json::Deserializer;
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
struct MessageEvent {
    topic: String,
    receiveTime: Map<String, Value>,
    message: Map<String, Value>,
    schemaName: String,
    sizeInBytes: i64,
}

#[derive(Serialize, Deserialize)]
struct TransformStamped {
    header: Map<String, Value>,
    child_frame_id: String,
    transform: Map<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct PolygonStamped {
    header: Map<String, Value>,
    polygon: Map<String, Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topics: Vec<Topic> = vec![
        Topic { name: "/polygon".to_string(), schema_name: "geometry_msgs/PolygonStamped".to_string() },
        Topic { name: "/tf".to_string(), schema_name: "geometry_msgs/TransformStamped".to_string() },
    ];

    let tf1: MessageEvent<TransformStamped> = MessageEvent {
        topic: "/tf".to_string(),
        receiveTime: serde_json::from_str("{\"sec\": 10, \"nsec\": 0}")?,
        message: serde_json::from_str(r#"{\"header\":{\"seq\": 0,\"stamp\":{\"sec\": 0,\"nsec\": 0},\"frame_id\":\"map\"},\"child_frame_id\":\"base_link\",\"transform\":{\"translation\":{\"x\": 1e7,\"y\": 0,\"z\": 0},\"rotation\":{\"x\": 0.0, \"y\": 0.0, \"z\": 0.0, \"w\": 1.0}}}"#)?;
        schemaName: "geometry_msgs/TransformStamped".to_string(),
        sizeInBytes: 0,
    };

    let tf2: MessageEvent<TransformStamped> = MessageEvent {
        topic: "/tf".to_string(),
        receiveTime: serde_json::from_str("{\"sec\": 10, \"nsec\": 0}")?,
        message: serde_json::from_str(r#"{\"header\":{\"seq\": 0,\"stamp\":{\"sec\": 0,\"nsec\": 0},\"frame_id\":\"base_link\"},\"child_frame_id\":\"sensor\",\"transform\":{\"translation\":{\"x\": 0.0, \"y\": 0.0, \"z\": 1.0},\"rotation\":{\"x\": 0.0, \"y\": 0.0, \"z\": 0.0, \"w\": 1.0}}}"#)?;
        schemaName: "geometry_msgs/TransformStamped".to_string(),
        sizeInBytes: 0,
    };

    let polygon: MessageEvent<PolygonStamped> = MessageEvent {
        topic: "/polygon".to_string(),
        receiveTime: serde_json::from_str("{\"sec\": 10, \"nsec\": 0}")?,
        message: serde_json::from_str(r#"{\"header\":{\"seq\": 0,\"stamp\":{\"sec\": 0,\"nsec\": 0},\"frame_id\":\"sensor\"},\"polygon\":{\"points\":[{\"x\": -1.0, \"y\": -1.0, \"z\": 0.0},{\"x\": 0.0, \"y\": 0.0, \"z\": 2.0},{\"x\": 1.0, \"y\": 1.0, \"z\": 0.0}]}]}"#)?;
        schemaName: "geometry_msgs/PolygonStamped".to_string(),
        sizeInBytes: 0,
    };

    let fixture = use_delayed_fixture({
        topics,
        frame: {
            "/polygon": vec![polygon],
            "/tf": vec![tf1, tf2],
        },
        capabilities: Vec::new(),
        active_data: Map::from([
            (String::from("currentTime"), serde_json::to_value(&Value::Number(Number::from(0)))?)
        ]),
    });

    let config = ThreeDeePanelConfig {
        override_config: ThreeDeePanelConfigOverrideConfig {
            ..ThreeDeePanelConfigOverrideConfig::default()
        },
        topics: Some(ThreeDeePanelConfigTopics {
            "/polygon": ThreeDeePanelTopic {
                visible: true,
            }
        }),
    };

    let result = ThreeDeePanel::render(&fixture, &config);
    println!("{:?}", result);

    Ok(())
}
```