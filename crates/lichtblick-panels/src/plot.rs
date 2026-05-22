// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Plot panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotConfig {
    /// Data series to plot.
    pub paths: Vec<PlotPath>,
    /// X-axis range in seconds (None = auto).
    pub x_axis_range: Option<f64>,
    /// Whether to show legend.
    pub show_legend: bool,
    /// Y-axis min (None = auto).
    pub y_min: Option<f64>,
    /// Y-axis max (None = auto).
    pub y_max: Option<f64>,
    /// Whether to follow playback time.
    pub follow_playback: bool,
}

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            paths: Vec::new(),
            x_axis_range: Some(30.0),
            show_legend: true,
            y_min: None,
            y_max: None,
            follow_playback: true,
        }
    }
}

/// A single data series in the plot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotPath {
    /// Message path expression (e.g., "/topic.field").
    pub value: String,
    /// Display label.
    pub label: Option<String>,
    /// Line color (CSS color string).
    pub color: Option<String>,
    /// Whether this series is enabled.
    pub enabled: bool,
    /// Timestamp source: "receiveTime" or "headerStamp".
    pub timestamp_method: TimestampMethod,
}

/// Timestamp method for plotting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimestampMethod {
    ReceiveTime,
    HeaderStamp,
}

impl Default for TimestampMethod {
    fn default() -> Self {
        Self::ReceiveTime
    }
}

/// A single data point in a plot series.
#[derive(Debug, Clone, Copy)]
pub struct PlotDataPoint {
    pub x: f64, // Time in seconds
    pub y: f64, // Value
}

/// Accumulated plot data for a series.
#[derive(Debug, Clone, Default)]
pub struct PlotSeriesData {
    pub points: Vec<PlotDataPoint>,
    pub label: String,
    pub color: String,
}
