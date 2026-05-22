// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Image panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    /// Camera topic path.
    pub camera_topic: Option<String>,
    /// Whether to flip horizontally.
    pub flip_horizontal: bool,
    /// Whether to flip vertically.
    pub flip_vertical: bool,
    /// Rotation in degrees (0, 90, 180, 270).
    pub rotation: u16,
    /// Min value for normalization (for depth images).
    pub min_value: Option<f64>,
    /// Max value for normalization (for depth images).
    pub max_value: Option<f64>,
    /// Color map for single-channel images.
    pub color_map: ColorMap,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            camera_topic: None,
            flip_horizontal: false,
            flip_vertical: false,
            rotation: 0,
            min_value: None,
            max_value: None,
            color_map: ColorMap::Gray,
        }
    }
}

/// Available color maps for single-channel image rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorMap {
    Gray,
    Turbo,
    Rainbow,
    Jet,
    Viridis,
    Inferno,
    Plasma,
    Magma,
}

/// Decoded image data ready for rendering.
#[derive(Debug, Clone)]
pub struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub encoding: ImageEncoding,
    pub data: Vec<u8>,
}

/// Image encoding formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageEncoding {
    Rgb8,
    Rgba8,
    Bgr8,
    Bgra8,
    Mono8,
    Mono16,
    Jpeg,
    Png,
    CompressedDepth,
}

impl ImageEncoding {
    /// Parse from ROS encoding string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "rgb8" => Some(Self::Rgb8),
            "rgba8" => Some(Self::Rgba8),
            "bgr8" => Some(Self::Bgr8),
            "bgra8" => Some(Self::Bgra8),
            "mono8" | "8UC1" => Some(Self::Mono8),
            "mono16" | "16UC1" => Some(Self::Mono16),
            "jpeg" | "jpg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "compressedDepth" => Some(Self::CompressedDepth),
            _ => None,
        }
    }

    /// Bytes per pixel for uncompressed formats.
    pub fn bytes_per_pixel(&self) -> Option<usize> {
        match self {
            Self::Rgb8 | Self::Bgr8 => Some(3),
            Self::Rgba8 | Self::Bgra8 => Some(4),
            Self::Mono8 => Some(1),
            Self::Mono16 => Some(2),
            _ => None, // Compressed formats
        }
    }
}
