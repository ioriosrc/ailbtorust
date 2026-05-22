```rust
use bevy::prelude::*;
use lichtblick::{suite, suite_base::players, suite_base::stories::PanelSetup, ThreeDeePanel};
use ros::*;

#[derive(Component)]
struct PoseStamped {
    header: Header,
    pose: Pose,
}

#[derive(Component)]
struct PoseWithCovarianceStamped {
    header: Header,
    pose: Pose,
    covariance: Covariance,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(MessageEvent::<TransformStamped>::default());
    app.insert_resource(MessageEvent::<PoseStamped>::default());
    app.insert_resource(MessageEvent::<PoseWithCovarianceStamped>::default());

    app.add_systems(
        setup,
        (
            add_tf_transforms,
            add_pose_stamped_messages,
            add_pose_with_covariance_messages,
        )
            .chain()
            .after(ReceiveMessages),
    );

    app.add_systems(update_camera_state, update_camera_state_system);

    app.add_systems(draw_panels, draw_panels_system);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_empty();
}

fn add_tf_transforms(mut transforms: Commands) {
    let tf1 = TransformStamped::new(
        Header {
            seq: 0,
            stamp: Time::from_nanos(100000000),
            frame_id: players::FIXED_FRAME_ID.to_string(),
            child_frame_id: players::BASE_LINK_FRAME_ID.to_string(),
        },
        transform::Transform {
            translation: Vector3::new(1e7, 0.0, 0.0),
            rotation: Quaternion::identity(),
        },
    );
    transforms.spawn(Transform::from_transform(tf1.transform));

    let tf2 = TransformStamped::new(
        Header {
            seq: 0,
            stamp: Time::from_nanos(100000000),
            frame_id: players::FIXED_FRAME_ID.to_string(),
            child_frame_id: players::SENSOR_FRAME_ID.to_string(),
        },
        transform::Transform {
            translation: Vector3::new(0.0, 0.0, 1.0),
            rotation: Quaternion::from_euler_angles(0.383, 0.0, 0.0),
        },
    );
    transforms.spawn(Transform::from_transform(tf2.transform));
}

fn add_pose_stamped_messages(mut messages: Commands) {
    let pose1 = PoseStamped::new(
        Header {
            seq: 0,
            stamp: Time::from_nanos(100000000),
            frame_id: players::SENSOR_FRAME_ID.to_string(),
        },
        Pose {
            position: Vector3::new(0.0, 0.0, -1.0),
            orientation: Quaternion::identity(),
        },
    );
    messages.spawn(PoseStamped::from_pose(pose1));

    // Add more pose stamped messages as needed
}

fn add_pose_with_covariance_messages(mut messages: Commands) {
    let pose2 = PoseWithCovarianceStamped::new(
        Header {
            seq: 0,
            stamp: Time::from_nanos(100000000),
            frame_id: players::BASE_LINK_FRAME_ID.to_string(),
        },
        Pose {
            position: Vector3::new(0.0, 0.0, 0.0),
            orientation: Quaternion::identity(),
        },
        Covariance {
            covariance_matrix: vec![
                // prettier-ignore
                2.0 * 2.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.15 * 0.15, 0.0, 0.0, 0.0, 0.0,
                0.0, 0, 0.3 * 0.3, 0.0, 0.0, 0.0,
                0.0, 0, 0, 0.0, 0.0, 0.0,
                0.0, 0, 0, 0.0, 0.0, 0.0,
                0.0, 0, 0, 0.0, 0.0, 0.0,
            ],
        },
    );
    messages.spawn(PoseWithCovarianceStamped::from_pose_with_covariance(pose2));

    // Add more pose with covariance messages as needed
}

fn update_camera_state_system(mut camera_state: ResMut<CameraState>) {
    camera_state.camera_state = CameraState {
        distance: 4.0,
        perspective: true,
        phi: std::f32::consts::PI / 6.0, // 30 degrees in radians
        target_offset: Vec3::new(-0.6, 0.5, 0.0),
        theta_offset: -std::f32::consts::PI / 4.0, // 45 degrees in radians
        fovy: std::f32::consts::FRAC_PI_8, // 45 degrees in radians
        near: 0.01,
        far: 5000.0,
        target: Vec3::new(0.0, 0.0, 0.0),
        target_orientation: Quaternion::identity(),
    };
}

fn draw_panels_system(mut commands: Commands) {
    let mut panel_setup = PanelSetup::default();
    panel_setup
        .add_panel(
            ThreeDeePanel {
                override_config: Some(ThreeDeePanel {
                    follow_tf: "base_link".to_string(),
                    camera_state: Some(CameraState {
                        distance: 4.0,
                        perspective: true,
                        phi: std::f32::consts::PI / 6.0, // 30 degrees in radians
                        target_offset: Vec3::new(-0.6, 0.5, 0.0),
                        theta_offset: -std::f32::consts::PI / 4.0, // 45 degrees in radians
                        fovy: std::f32::consts::FRAC_PI_8, // 45 degrees in radians
                        near: 0.01,
                        far: 5000.0,
                        target: Vec3::new(0.0, 0.0, 0.0),
                        target_orientation: Quaternion::identity(),
                    }),
                    topics: Some(vec![
                        Topic {
                            name: "/pose".to_string(),
                            visible: true,
                            type: "arrow".to_string(),
                            color: Color::rgba(107, 220, 255, 0.5),
                        },
                        Topic {
                            name: "/pose_with_covariance".to_string(),
                            visible: true,
                            type: "arrow".to_string(),
                        },
                        Topic {
                            name: "/pose_with_hidden_covariance".to_string(),
                            visible: true,
                            type: "arrow",
                            show_covariance: false,
                            covariance_color: Color::rgba(255, 0, 0, 1),
                        },
                        Topic {
                            name: "/pose_axis_with_covariance".to_string(),
                            visible: true,
                        },
                    ]),
                }),
            }
        )
        .insert(&mut commands);
}
```