// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Full configuration for the 3D panel, matching Lichtblick Node.js settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDeeConfig {
    /// Title displayed in panel header (default "3D").
    pub title: String,
    /// Selected display frame (coordinate reference).
    pub display_frame: String,
    /// Follow mode: "pose", "position", or "fixed".
    pub follow_mode: String,
    /// Scene rendering settings.
    pub scene: SceneConfig,
    /// Camera/view settings.
    pub view: ViewConfig,
    /// Transform tree display settings.
    pub transforms: TransformsConfig,
    /// Per-topic display settings.
    pub topics: HashMap<String, TopicDisplayConfig>,
    /// Custom layers (grids, URDFs).
    pub custom_layers: CustomLayersConfig,
    /// Publish settings (click-to-publish).
    pub publish: PublishConfig,
}

impl Default for ThreeDeeConfig {
    fn default() -> Self {
        Self {
            title: "3D".into(),
            display_frame: "Global".into(),
            follow_mode: "pose".into(),
            scene: SceneConfig::default(),
            view: ViewConfig::default(),
            transforms: TransformsConfig::default(),
            topics: HashMap::new(),
            custom_layers: CustomLayersConfig::default(),
            publish: PublishConfig::default(),
        }
    }
}

/// Scene group settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneConfig {
    /// Show render stats overlay (FPS, draw calls).
    pub enable_stats: bool,
    /// Background clear color (hex).
    pub background_color: String,
    /// Scale factor for all 3D text labels.
    pub label_scale: f64,
    /// Ignore COLLADA <up_axis> tag in DAE models.
    pub ignore_collada_up_axis: bool,
    /// Default mesh up axis: "y_up" or "z_up".
    pub mesh_up_axis: String,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            enable_stats: false,
            background_color: String::new(), // empty = use theme default (#15151a dark, #f4f4f5 light)
            label_scale: 1.0,
            ignore_collada_up_axis: false,
            mesh_up_axis: "y_up".into(),
        }
    }
}

/// View/camera group settings (orbit camera).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewConfig {
    /// Sync camera across multiple 3D panels.
    pub sync_camera: bool,
    /// Orbit distance from target.
    pub distance: f64,
    /// Perspective (true) or Orthographic (false).
    pub perspective: bool,
    /// Camera focal point (world-space target, usually [0,0,0]).
    pub target: [f64; 3],
    /// Camera orbit center offset from frame origin (user pan).
    /// Equivalent to Node.js cameraState.targetOffset.
    pub target_offset: [f64; 3],
    /// Azimuthal angle (degrees) - thetaOffset in Node.js convention.
    pub theta: f64,
    /// Polar pitch angle from zenith (degrees) - phi in Node.js convention.
    pub phi: f64,
    /// Vertical field of view (degrees, perspective only).
    pub fovy: f64,
    /// Near clipping plane.
    pub near: f64,
    /// Far clipping plane.
    pub far: f64,
}

impl Default for ViewConfig {
    fn default() -> Self {
        Self {
            sync_camera: false,
            distance: 50.0,
            perspective: true,
            target: [0.0, 0.0, 0.0],
            target_offset: [0.0, 0.0, 0.0],
            theta: 45.0,
            phi: 60.0,
            fovy: 45.0,
            near: 0.5,
            far: 5000.0,
        }
    }
}

/// Transforms group settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformsConfig {
    /// Allow manual TF offset editing.
    pub editable: bool,
    /// Show frame name labels in viewport.
    pub show_labels: bool,
    /// Font size of frame labels.
    pub label_size: f64,
    /// Scale of RGB coordinate axes.
    pub axis_scale: f64,
    /// Width of parent-child connection lines.
    pub line_width: f64,
    /// Color of connection lines (hex).
    pub line_color: String,
    /// Buffer TF messages ahead of playhead.
    pub enable_preloading: bool,
    /// Max preload buffer size.
    pub max_preload_messages: u32,
    /// Per-frame manual offsets.
    pub offsets: HashMap<String, TransformOffset>,
    /// Frames hidden from 3D viewport.
    pub hidden_frames: HashSet<String>,
}

impl Default for TransformsConfig {
    fn default() -> Self {
        Self {
            editable: false,
            show_labels: true,
            label_size: 0.2,
            axis_scale: 1.0,
            line_width: 2.0,
            line_color: "#ffff00".into(),
            enable_preloading: false,
            max_preload_messages: 10000,
            offsets: HashMap::new(),
            hidden_frames: HashSet::new(),
        }
    }
}

/// Manual offset for a single TF frame.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformOffset {
    /// Translation offset [x, y, z].
    pub translation: [f64; 3],
    /// Rotation offset [roll, pitch, yaw] in radians.
    pub rotation: [f64; 3],
}

impl Default for TransformOffset {
    fn default() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
        }
    }
}

/// Per-topic display configuration (matches extension panelSettings).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicDisplayConfig {
    /// Whether this topic is visible.
    pub visible: bool,
    /// Custom render color (hex).
    pub color: Option<String>,
    /// Draw outlines on meshes.
    pub show_outlines: bool,
    /// Enable caching of computed geometry.
    pub caching: bool,
    /// Render coordinate axes on topic entities.
    pub show_axes: bool,
    /// Render physical lane boundaries.
    pub show_physical_lanes: bool,
    /// Render logical lane boundaries.
    pub show_logical_lanes: bool,
    /// Render reference lines.
    pub show_reference_lines: bool,
    /// Render bounding boxes.
    pub show_bounding_box: bool,
    /// Render 3D vehicle models.
    pub show_3d_models: bool,
    /// Path to default 3D model files.
    pub default_model_path: String,
}

impl Default for TopicDisplayConfig {
    fn default() -> Self {
        Self {
            visible: true,
            color: None,
            show_outlines: true,
            caching: true,
            show_axes: true,
            show_physical_lanes: true,
            show_logical_lanes: false,
            show_reference_lines: true,
            show_bounding_box: true,
            show_3d_models: false,
            default_model_path: "/opt/models/vehicles/".into(),
        }
    }
}

/// Custom layers configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomLayersConfig {
    /// Grid layers.
    pub grids: Vec<GridLayer>,
    /// URDF model layers.
    pub urdfs: Vec<UrdfLayer>,
}

impl Default for CustomLayersConfig {
    fn default() -> Self {
        Self {
            grids: vec![GridLayer::default()],
            urdfs: Vec::new(),
        }
    }
}

/// A grid helper layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridLayer {
    pub visible: bool,
    pub size: f64,
    pub divisions: u32,
    pub color: String,
    pub frame_id: String,
}

impl Default for GridLayer {
    fn default() -> Self {
        Self {
            visible: true,
            size: 10.0,
            divisions: 10,
            color: "#248eff33".into(),
            frame_id: "Global".into(),
        }
    }
}

/// A URDF model layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrdfLayer {
    pub visible: bool,
    pub url: String,
    pub frame_id: String,
}

/// Publish group settings (click-to-publish).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishConfig {
    /// Message type: "point", "pose", or "pose_estimate".
    pub publish_type: String,
    /// Topic to publish to.
    pub topic: String,
}

impl Default for PublishConfig {
    fn default() -> Self {
        Self {
            publish_type: "point".into(),
            topic: "/clicked_point".into(),
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
