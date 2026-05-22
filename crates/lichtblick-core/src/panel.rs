// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Panel type identifier used in the panel catalog.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PanelType(pub String);

/// Panel info for the panel catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelInfo {
    /// Unique panel type identifier.
    pub panel_type: PanelType,
    /// Display title.
    pub title: String,
    /// Optional description.
    pub description: Option<String>,
    /// Category for grouping in the catalog.
    pub category: PanelCategory,
    /// Whether this panel has custom settings.
    pub has_settings: bool,
}

/// Panel categories for organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelCategory {
    Visualization,
    Diagnostics,
    Teleop,
    Utility,
}

/// Configuration for a panel instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    /// Panel type.
    pub panel_type: PanelType,
    /// Panel-specific configuration (serialized as JSON).
    pub config: serde_json::Value,
}

/// Unique identifier for a panel instance in a layout.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PanelId(pub String);

impl PanelId {
    pub fn new(panel_type: &str) -> Self {
        Self(format!("{}!{}", panel_type, uuid::Uuid::new_v4()))
    }
}
