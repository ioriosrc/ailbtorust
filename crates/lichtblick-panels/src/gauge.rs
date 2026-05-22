// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Gauge panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugeConfig {
    pub path: Option<String>,
    pub min_value: f64,
    pub max_value: f64,
    pub color_map: GaugeColorMap,
}

impl Default for GaugeConfig {
    fn default() -> Self {
        Self {
            path: None,
            min_value: 0.0,
            max_value: 100.0,
            color_map: GaugeColorMap::GreenYellowRed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GaugeColorMap {
    GreenYellowRed,
    RedYellowGreen,
    BlueRedBright,
}
