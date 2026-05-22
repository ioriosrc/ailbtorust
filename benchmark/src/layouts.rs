```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct LayoutData {
    pub config_by_id: HashMap<String, serde_json::Value>,
    pub global_variables: HashMap<String, serde_json::Value>,
    pub user_nodes: HashMap<String, serde_json::Value>,
    pub playback_config: PlaybackConfig,
}

#[derive(Debug, Clone)]
struct PlaybackConfig {
    speed: f32,
}

pub fn make_layout_data(partial_data: PartialLayoutData) -> LayoutData {
    LayoutData {
        config_by_id: partial_data.config_by_id,
        global_variables: HashMap::new(),
        user_nodes: HashMap::new(),
        playback_config: PlaybackConfig {
            speed: 1.0,
        },
    }
}

pub const LAYOUTS: HashMap<&'static str, LayoutData> = HashMap::from([
    ("multipleThreeDee", make_layout_data(DefaultMultipleThreeDee)),
    ("empty", make_layout_data(Empty)),
    ("sinewave", make_layout_data(SinewaveSinglePlot)),
    ("pointCloudRawMessage", make_layout_data(PointcloudRawMessageAnd3d)),
    ("pointCloudMultipleThreeDee", make_layout_data(PointcloudMultipleThreeDee)),
    ("transformPreloading", make_layout_data(TransformPreloading)),
]);
```