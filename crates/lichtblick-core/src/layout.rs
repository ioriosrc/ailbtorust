// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Layout tree structure (equivalent to react-mosaic MosaicNode).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayoutNode {
    /// A leaf node containing a panel ID.
    Leaf(String),
    /// A split node containing two children.
    Split {
        direction: SplitDirection,
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
        /// Split percentage (0-100). Default is 50.
        split_percentage: Option<f64>,
    },
}

/// Direction of a panel split.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SplitDirection {
    Row,
    Column,
}

/// Playback configuration stored in a layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackConfig {
    /// Playback speed (1.0 = realtime).
    pub speed: f64,
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self { speed: 1.0 }
    }
}

/// Complete layout data persisted to storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutData {
    /// Panel configurations keyed by panel ID.
    pub config_by_id: HashMap<String, serde_json::Value>,
    /// Layout tree.
    pub layout: Option<LayoutNode>,
    /// Global variables.
    pub global_variables: HashMap<String, serde_json::Value>,
    /// Playback config.
    pub playback_config: PlaybackConfig,
    /// User scripts.
    pub user_nodes: HashMap<String, UserNode>,
}

impl Default for LayoutData {
    fn default() -> Self {
        Self {
            config_by_id: HashMap::new(),
            layout: None,
            global_variables: HashMap::new(),
            playback_config: PlaybackConfig::default(),
            user_nodes: HashMap::new(),
        }
    }
}

/// A user-defined transformation script.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserNode {
    pub name: String,
    pub source_code: String,
}

/// A saved layout in storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub id: String,
    pub name: String,
    pub data: LayoutData,
    pub saved_at: Option<String>,
}

/// Layout storage operations.
pub trait LayoutStorage: Send + Sync {
    fn get_layouts(&self) -> Vec<Layout>;
    fn get_layout(&self, id: &str) -> Option<Layout>;
    fn save_layout(&self, layout: Layout);
    fn delete_layout(&self, id: &str);
}
