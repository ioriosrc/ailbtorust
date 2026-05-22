// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for the 3D panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeDeeConfig {
    /// Camera position and orientation.
    pub camera_state: CameraState,
    /// Topic configurations.
    pub topics: HashMap<String, TopicDisplayConfig>,
    /// Whether to show grid.
    pub show_grid: bool,
    /// Grid size.
    pub grid_size: f64,
    /// Background color.
    pub background_color: String,
    /// Fixed frame (TF frame to use as world frame).
    pub fixed_frame: Option<String>,
    /// Display frame (camera follows this frame).
    pub display_frame: Option<String>,
}

impl Default for ThreeDeeConfig {
    fn default() -> Self {
        Self {
            camera_state: CameraState::default(),
            topics: HashMap::new(),
            show_grid: true,
            grid_size: 10.0,
            background_color: "#1a1a2e".into(),
            fixed_frame: None,
            display_frame: None,
        }
    }
}

/// 3D camera state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraState {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub up: [f64; 3],
    pub fov: f64,
    pub near: f64,
    pub far: f64,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            position: [5.0, 5.0, 5.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 0.0, 1.0],
            fov: 45.0,
            near: 0.01,
            far: 10000.0,
        }
    }
}

/// Configuration for how a topic is displayed in 3D.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicDisplayConfig {
    pub visible: bool,
    pub color: Option<String>,
    pub opacity: f64,
    pub point_size: Option<f64>,
    pub line_width: Option<f64>,
}

impl Default for TopicDisplayConfig {
    fn default() -> Self {
        Self {
            visible: true,
            color: None,
            opacity: 1.0,
            point_size: None,
            line_width: None,
        }
    }
}

/// Supported 3D render types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderableType {
    Marker,
    MarkerArray,
    PointCloud2,
    LaserScan,
    Mesh,
    PoseStamped,
    PoseArray,
    Path,
    GridCells,
    OccupancyGrid,
    Image,
    TfTransform,
}

impl RenderableType {
    /// Determine renderable type from schema name.
    pub fn from_schema(schema: &str) -> Option<Self> {
        match schema {
            "visualization_msgs/Marker" | "visualization_msgs/msg/Marker" => Some(Self::Marker),
            "visualization_msgs/MarkerArray" | "visualization_msgs/msg/MarkerArray" => Some(Self::MarkerArray),
            "sensor_msgs/PointCloud2" | "sensor_msgs/msg/PointCloud2" => Some(Self::PointCloud2),
            "sensor_msgs/LaserScan" | "sensor_msgs/msg/LaserScan" => Some(Self::LaserScan),
            "geometry_msgs/PoseStamped" | "geometry_msgs/msg/PoseStamped" => Some(Self::PoseStamped),
            "geometry_msgs/PoseArray" | "geometry_msgs/msg/PoseArray" => Some(Self::PoseArray),
            "nav_msgs/Path" | "nav_msgs/msg/Path" => Some(Self::Path),
            "nav_msgs/GridCells" | "nav_msgs/msg/GridCells" => Some(Self::GridCells),
            "nav_msgs/OccupancyGrid" | "nav_msgs/msg/OccupancyGrid" => Some(Self::OccupancyGrid),
            "sensor_msgs/Image" | "sensor_msgs/msg/Image" => Some(Self::Image),
            "tf2_msgs/TFMessage" | "tf2_msgs/msg/TFMessage" => Some(Self::TfTransform),
            _ => None,
        }
    }
}
