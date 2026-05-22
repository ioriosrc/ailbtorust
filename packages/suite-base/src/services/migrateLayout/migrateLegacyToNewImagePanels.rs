```rust
use std::collections::{HashMap, VecDeque};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use lichtblick_suite_base::context::CurrentLayoutContext;
use lichtblick_suite_base::panels::ThreeDeeRender::IRenderer;
use lichtblick_suite_base::panels::ThreeDeeRender::camera::DEFAULT_CAMERA_STATE;
use lichtblick_suite_base::panels::ThreeDeeRender::renderables::PublishSettings;
use lichtblick_suite_base::util::{getAllPanelIds, getPanelIdForType, getPanelTypeFromId};

type LegacyImageConfig = {
  camera_topic: String;
  enabled_marker_topics: Vec<String>;
  synchronize: bool;
  flip_horizontal: Option<bool>;
  flip_vertical: Option<bool>;
  maxValue: Option<f64>;
  minValue: Option<f64>;
  mode: Option<&'static str>;
  pan: Option<(f32, f32)>;
  rotation: Option<i8>;
  smooth: Option<bool>;
  transform_markers: bool;
  zoom: Option<f32>;
  zoom_percentage: Option<f32>;
};

fn migrate_legacy_to_new_image_config(legacy_config: LegacyImageConfig) -> IRenderer {
    IRenderer {
        camera_state: DEFAULT_CAMERA_STATE,
        follow_mode: "follow-pose",
        follow_tf: None,
        scene: HashMap::new(),
        transforms: HashMap::new(),
        topics: HashMap::new(),
        layers: HashMap::new(),
        publish: PublishSettings::default(),
        image_mode: ImageMode {
            image_topic: legacy_config.camera_topic,
            calibration_topic: None,
            synchronize: legacy_config.synchronize,
            rotation:
                if [0, 90, 180, 270].contains(&legacy_config.rotation) {
                    legacy_config.rotation as i8
                } else {
                    0
                },
            flip_horizontal: legacy_config.flip_horizontal.unwrap_or(false),
            flip_vertical: legacy_config.flip_vertical.unwrap_or(false),
            minValue: legacy_config.min_value,
            maxValue: legacy_config.max_value,
            annotations: annotations_from_legacy_markers(legacy_config.enabled_marker_topics),
        },
    }
}

fn annotations_from_legacy_markers(enabled_marker_topics: Vec<String>) -> HashMap<String, ImageAnnotation> {
    enabled_marker_topics
        .iter()
        .map(|topic_name| (topic_name.to_string(), ImageAnnotation { visible: true }))
        .collect()
}

pub fn migrate_legacy_to_new_image_panels(layout_data: &CurrentLayoutContext) -> CurrentLayoutContext {
    if layout_data.layout.is_none() {
        return layout_data.clone();
    }

    let legacy_image_panels = getAllPanelIds(&layout_data.layout, &layout_data.config_by_id).filter(|id| get_panel_type_from_id(id) == "ImageViewPanel");

    let mut new_state = layout_data.clone();

    for id in legacy_image_panels {
        let legacy_config = layout_data.config_by_id.get(&id).unwrap() as &LegacyImageConfig;

        new_state = replace_panel(
            &mut new_state,
            id,
            get_panel_id_for_type("Image"),
            migrate_legacy_to_new_image_config(*legacy_config),
        );
    }

    new_state
}

struct ImageMode {
    image_topic: String;
    calibration_topic: Option<String>;
    synchronize: bool;
    rotation: i8;
    flip_horizontal: bool;
    flip_vertical: bool;
    minValue: Option<f64>;
    maxValue: Option<f64>;
    annotations: HashMap<String, ImageAnnotation>,
}

struct ImageAnnotation {
    visible: bool;
}
```