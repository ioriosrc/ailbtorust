```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct LegacyCameraState {
    distance: f64,
    perspective: bool,
    phi: f64,
    target: [f64; 3],
    target_offset: [f64; 3],
    target_orientation: [f64; 4],
    theta_offset: f64,
    fovy: f64,
    near: f64,
    far: f64,
}

#[derive(Serialize, Deserialize)]
struct Legacy3DConfig {
    camera_state: Option<LegacyCameraState>,
    checked_keys: Vec<String>,
    click_to_publish_pose_topic: String,
    click_to_publish_point_topic: String,
    click_to_publish_pose_estimate_topic: String,
    click_to_publish_pose_estimate_x_deviation: f64,
    click_to_publish_pose_estimate_y_deviation: f64,
    click_to_publish_pose_estimate_theta_deviation: f64,
    follow_mode: Option<String>,
    follow_tf: Option<String>,
}

fn migrate_legacy_to_new3d_config(legacy_config: Legacy3DConfig) -> RendererConfig {
    let mut camera_state = DEFAULT_CAMERA_STATE.clone();
    if let Some(state) = legacy_config.camera_state {
        camera_state.distance = state.distance * 180.0 / std::f64::PI;
        camera_state.phi = state.phi * 180.0 / std::f64::PI;
        camera_state.theta_offset = state.theta_offset * 180.0 / std::f64::PI;
        camera_state.fovy = state.fovy * 180.0 / std::f64::PI;
    }

    let mut publish_settings = DEFAULT_PUBLISH_SETTINGS.clone();
    if legacy_config.click_to_publish_pose_topic != "" {
        publish_settings.pose_topic = legacy_config.click_to_publish_pose_topic.to_string();
    }
    if legacy_config.click_to_publish_point_topic != "" {
        publish_settings.point_topic = legacy_config.click_to_publish_point_topic.to_string();
    }
    if legacy_config.click_to_publish_pose_estimate_topic != "" {
        publish_settings.pose_estimate_topic = legacy_config.click_to_publish_pose_estimate_topic.to_string();
    }

    RendererConfig {
        follow_tf: legacy_config.follow_tf,
        follow_mode: match legacy_config.follow_mode {
            Some("follow-orientation") => "follow-pose",
            Some("follow") => "follow-position",
            None => "follow-none",
        },
        camera_state,
        publish: publish_settings,
        topics: HashMap::from_iter(
            legacy_config
                .checked_keys
                .iter()
                .filter(|key| key.starts_with("t:"))
                .map(|key| {
                    let id = key[2..].to_string();
                    (id, true)
                }),
        ),
        scene: HashMap::new(),
        transforms: HashMap::new(),
        layers: HashMap::new(),
        image_mode: HashMap::new(),
    }
}

fn migrate_legacy_to_new3d_panels(layout_data: LayoutData) -> LayoutData {
    if let Some(layout) = layout_data.layout {
        let legacy_3d_panels = layout
            .panel_ids()
            .iter()
            .filter(|id| id.starts_with("t:"))
            .map(|id| format!("{}:{}", id, get_panel_type_from_id(id)))
            .collect::<Vec<_>>();

        for panel_id in legacy_3d_panels {
            let legacy_config = layout.config(panel_id).unwrap();
            let new_config = migrate_legacy_to_new3d_config(legacy_config);
            layout_data = replace_layout_item(layout_data, &panel_id, panel_id, new_config);
        }
    }

    layout_data
}
```