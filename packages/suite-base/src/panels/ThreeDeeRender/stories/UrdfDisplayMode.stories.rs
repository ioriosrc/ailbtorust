```rust
use crate::{fixture::Fixture, panel_setup::PanelSetup};
use lichtenblick::suite_base::{
    components::{ThreeDeePanel},
    topics::Topic,
};

const GREEN: &str = "#4caf50";
const BLUE: &str = "#2196f3";
const RED: &str = "#f44336";

fn make_color_attribute(hex: &str, alpha: f32) -> String {
    format!("{} {} {}", hex.split(' ').nth(0).unwrap(), hex.split(' ').nth(1).unwrap(), alpha)
}

pub fn three_dee_panel() -> PanelSetup<Fixture> {
    let topics = vec![Topic::new("/tf_static", "tf2_msgs/TFMessage")];

    let mesh_T_robot = Fixture::Transform {
        frame_id: "mesh-no-material",
        child_frame_id: "",
        transform: Fixture::TransformValue {
            rotation: Fixture::QuaternionValue {
                w: 1.0,
            },
        },
    };

    let urdf_param_name = "/some_ns/robot_description";
    let urdf_displays = vec![
        ("urdf1", "auto"),
        ("urdf2", "visual"),
        ("urdf3", "collision"),
        ("urdf4", "collision colored"),
    ]
    .iter()
    .map(|(&name, display_mode)| {
        Fixture::Display {
            source_type: Fixture::SourceType::Param,
            parameter: urdf_param_name.to_string(),
            layer_id: "foxglove.Urdf",
            frame_prefix: format!("display_{}", name),
            display_mode: *display_mode,
            translation: vec![0.0, 0.0, 0.5],
        }
    })
    .collect();

    let fixture = Fixture {
        topics,
        capabilities: Vec::new(),
        frame: Fixture::Frame {
            "/tf_static": vec![
                Fixture::Transform {
                    topic: "/tf_static",
                    schema_name: "tf2_msgs/TFMessage",
                    receive_time: Fixture::TimeValue { sec: 0, nsec: 0 },
                    size_in_bytes: 0,
                    message: Fixture::TransformValue {
                        rotation: Fixture::QuaternionValue {
                            w: 1.0,
                        },
                    },
                },
            ],
        },
        active_data: Fixture::ActiveData {
            current_time: Fixture::TimeValue { sec: 0, nsec: 0 },
            parameters: std::collections::HashMap::from([
                (urdf_param_name.to_string(), URDF.into()),
            ]),
        },
    };

    PanelSetup::new(fixture)
}

fn main() {
    three_dee_panel().show();
}
```