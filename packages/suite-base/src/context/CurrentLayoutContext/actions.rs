```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;

type MosaicNode<String> = Vec<MosaicNode<String>>;
type MosaicPath = Vec<usize>;

#[derive(Serialize, Deserialize)]
pub struct LayoutData {
    config_by_id: HashMap<String, SavedProps>,
    layout: Option<MosaicNode<String>>,
    global_variables: GlobalVariables,
    playback_config: PlaybackConfig,
    user_nodes: UserScripts,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigsPayload {
    id: String,
    override: Option<bool>,
    config: PanelConfig,
    default_config: Option<PanelConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangePanelLayoutPayload {
    layout: Option<MosaicNode<String>>,
    trim_config_by_id: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveConfigsPayload {
    configs: Vec<ConfigsPayload>,
}

type PerPanelFunc<Config> = fn(Config) -> Config;

#[derive(Serialize, Deserialize)]
pub struct CreateTabPanelPayload {
    id_to_replace: Option<String>,
    layout: MosaicNode<String>,
    idsToRemove: Vec<String>,
    single_tab: bool,
}

// ... (the rest of the code remains the same)
```