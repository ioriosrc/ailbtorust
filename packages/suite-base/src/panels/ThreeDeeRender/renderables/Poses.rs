```rust
use crate::{Axis, Arrow, MessageRenderer, PoseRenderable};
use chrono::NaiveTime;
use i18next::ResourceBundle;
use log::info;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::HashMap;

pub enum DisplayType {
    Axis,
    Arrow,
}

const DEFAULT_TYPE: DisplayType = DisplayType::Axis;
const DEFAULT_AXIS_SCALE: f32 = 10.0; // Example value for axis scale
const DEFAULT_ARROW_SCALE: [f32; 3] = [1.0, 0.15, 0.15];
const DEFAULT_COLOR: (u8, u8, u8) = (255, 255, 255); // Example color for axis
const DEFAULT_COVARIANCE_COLOR: (u8, u8, u8) = (0, 0, 255); // Example color for covariance sphere

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn normalize_pose_stamped(pose: &PoseStamped) -> PoseStamped {
    pose.header = normalize_header(&pose.header);
    pose.pose = normalize_pose(&pose.pose);
    pose
}

fn normalize_pose_in_frame_to_pose_stamped(pose: &PoseInFrame) -> PoseStamped {
    pose.header = normalize_header(&pose.header);
    pose.pose = normalize_pose(&pose.pose);
    pose
}

fn normalize_pose_with_covariance(pose: &PoseWithCovariance) -> PoseWithCovariance {
    if let Some(covariance) = pose.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        PoseWithCovariance { pose: pose.pose, covariance }
    } else {
        pose.clone()
    }
}

fn normalize_pose_with_covariance_stamped(pose_with_covariance: &PoseWithCovarianceStamped) -> PoseWithCovarianceStamped {
    pose_with_covariance.header = normalize_header(&pose_with_covariance.header);
    pose_with_covariance.pose = normalize_pose_with_covariance(&pose_with_covariance.pose);
    pose_with_covariance
}

fn normalize_header(header: &Header) -> Header {
    header.clone()
}

fn normalize_pose(pose: &Pose) -> Pose {
    pose.clone()
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}

fn create_arrow_marker(arrow_scale: [f32; 3], color: (u8, u8, u8)) -> Arrow {
    let scale = arrow_scale.map(|v| v * 10.0); // Convert scale to meters
    Arrow::new(scale, Some(color))
}

fn create_sphere_marker(pose_with_covariance: &PoseWithCovarianceStamped, settings: &LayerSettings) -> Option<Arrow> {
    if let Some(covariance) = pose_with_covariance.covariance {
        let covariance = covariance[..6].to_vec(); // Extract upper-left 3x1 diagonal
        let scale = [covariance[0], covariance[7], covariance[14]]; // Convert to meters
        Some(Arrow::new(scale, Some(settings.covariance_color)))
    } else {
        None
    }
}

struct LayerSettings {
    type: DisplayType,
    axis_scale: f32,
    arrow_scale: [f32; 3],
    color: (u8, u8, u8),
    covariance_color: (u8, u8, u8),
}