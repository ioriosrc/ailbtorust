```rust
use crate::suite::{MessageEvent, Topic};
use crate::suite_base::players::types::PoseStamped;
use crate::suite_base::stories::PanelSetup;
use crate::ThreeDeePanel;

pub fn geometry_msgs_PoseStamped() -> PanelSetup {
    let topics: Vec<Topic> = vec![
        Topic {
            name: "/tf".to_string(),
            schema_name: "geometry_msgs/TransformStamped".to_string(),
        },
        Topic {
            name: "/pose1".to_string(),
            schema_name: "geometry_msgs/PoseStamped".to_string(),
        },
        Topic {
            name: "/pose2".to_string(),
            schema_name: "geometry_msgs/PoseStamped".to_string(),
        },
        Topic {
            name: "/pose3".to_string(),
            schema_name: "geometry_msgs/PoseStamped".to_string(),
        },
    ];

    let tf1 = MessageEvent {
        topic: "/tf".to_string(),
        receive_time: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
        message: PoseStamped {
            header: crate::suite_base::messages::Header {
                seq: 0,
                stamp: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
                frame_id: "map".to_string(),
            },
            child_frame_id: "base_link".to_string(),
            transform: crate::suite::math::Transform {
                translation: crate::suite::math::Vec3::new(1e7 as f64, 0.0, 0.0),
                rotation: crate::suite::math::Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let tf2 = MessageEvent {
        topic: "/tf".to_string(),
        receive_time: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
        message: PoseStamped {
            header: crate::suite_base::messages::Header {
                seq: 0,
                stamp: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
                frame_id: "base_link".to_string(),
            },
            child_frame_id: "sensor".to_string(),
            transform: crate::suite::math::Transform {
                translation: crate::suite::math::Vec3::new(0.0, -5.0, 0.0),
                rotation: crate::suite::math::Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/TransformStamped".to_string(),
        size_in_bytes: 0,
    };

    let pose1 = MessageEvent {
        topic: "/pose1".to_string(),
        receive_time: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
        message: PoseStamped {
            header: crate::suite_base::messages::Header {
                seq: 0,
                stamp: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
                frame_id: "base_link".to_string(),
            },
            pose: crate::suite::math::Pose {
                position: crate::suite::math::Vec3::new(2.0, 0.0, 0.0),
                orientation: crate::suite::math::Quaternion::identity(),
            },
        },
        schema_name: "geometry_msgs/PoseStamped".to_string(),
        size_in_bytes: 0,
    };

    let pose2 = MessageEvent {
        topic: "/pose2".to_string(),
        receive_time: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
        message: PoseStamped {
            header: crate::suite_base::messages::Header {
                seq: 0,
                stamp: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
                frame_id: "sensor".to_string(),
            },
            pose: crate::suite::math::Pose {
                position: crate::suite::math::Vec3::new(0.0, 3.0, 0.0),
                orientation: crate::suite::math::Quaternion::from_euler_angles(0.0, 0.0, std::f64::consts::PI / 2.0),
            },
        },
        schema_name: "geometry_msgs/PoseStamped".to_string(),
        size_in_bytes: 0,
    };

    let pose3 = MessageEvent {
        topic: "/pose3".to_string(),
        receive_time: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
        message: PoseStamped {
            header: crate::suite_base::messages::Header {
                seq: 0,
                stamp: crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
                frame_id: "base_link".to_string(),
            },
            pose: crate::suite::math::Pose {
                position: crate::suite::math::Vec3::new(0.0, 2.0, 0.0),
                orientation: crate::suite::math::Quaternion::from_euler_angles(0.0, 0.0, std::f64::consts::PI / 4.0),
            },
        },
        schema_name: "geometry_msgs/PoseStamped".to_string(),
        size_in_bytes: 0,
    };

    let fixture = crate::suite_base::utils::use_delayed_fixture(
        topics,
        Some(crate::suite_base::players::types::Transforms {
            "/tf": vec![tf1, tf2],
            "/pose1": vec![pose1],
            "/pose2": vec![pose2],
            "/pose3": vec![pose3],
        }),
        vec![],
        crate::suite_base::times::Times::from_seconds_and_nanoseconds(0, 0),
    );

    PanelSetup {
        fixture,
        override_config: Some(crate::ThreeDeePanelConfig {
            follow_tf: "base_link".to_string(),
            topics: crate::ThreeDeePanelTopics {
                "/pose1": {
                    visible: true,
                    type: "arrow",
                },
                "/pose2": {
                    visible: true,
                    type: "arrow",
                    arrow_scale: vec![2.0, 1.0, 1.0],
                    color: "rgba(0, 255, 0, 0.3)",
                },
                "/pose3": {
                    visible: true,
                    axis_scale: std::f64::consts::SQRT_8,
                },
            },
            layers: crate::ThreeDeePanelLayers {
                grid: { layer_id: "foxglove.Grid" },
            },
            camera_state: crate::ThreeDeePanelCameraState {
                distance: 15.0,
                perspective: false,
                phi: std::f64::consts::PI / 2.0,
                target_offset: [-0.6, 0.5, 0.0],
                theta_offset: std::f64::consts::PI / 2.0,
                fovy: std::f64::consts::PI / 3.0,
                near: 0.01,
                far: 5000.0,
                target: crate::suite::math::Vec3::new(0.0, 0.0, 0.0),
                target_orientation: crate::suite::math::Quaternion::identity(),
            },
        }),
    }
}
```