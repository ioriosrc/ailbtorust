// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Map panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapConfig {
    pub topic: Option<String>,
    pub center: [f64; 2],
    pub zoom: f64,
    pub max_points: usize,
    pub follow_topic: bool,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            topic: None,
            center: [0.0, 0.0],
            zoom: 14.0,
            max_points: 10000,
            follow_topic: true,
        }
    }
}
