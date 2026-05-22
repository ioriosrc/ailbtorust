```rust
use std::ops::{Add, Sub};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

pub type ColorRGBA = [f32; 4];

pub type Vector3 = [f32; 3];

#[derive(Debug)]
pub struct Pose {
    pub position: Vector3,
    pub rotation: Vector3,
}

pub type BaseShape = {
    pose: Pose,
    scale: Vector3,
    color: Option<ColorRGBA>,
};

pub type MouseEventObject = {
    object: BaseShape;
    instance_index: Option<usize>;
};

pub type CameraState = {
    distance: f32,
    perspective: bool,
    phi: f32,
    target: [f32; 3],
    target_offset: [f32; 3],
    target_orientation: [f32; 4],
    theta_offset: f32,
    fovy: f32,
    near: f32,
    far: f32,
};

pub const DEFAULT_CAMERA_STATE: CameraState = {
    distance: 20.0,
    perspective: true,
    phi: 60.0,
    target: [0.0, 0.0, 0.0],
    target_offset: [0.0, 0.0, 0.0],
    target_orientation: [0.0, 0.0, 0.0, 1.0],
    theta_offset: 45.0,
    fovy: 45.0,
    near: 0.5,
    far: 5000.0,
};

pub type OrbitControlsConfig = {
    screen_space_panning: bool,
    mouse_buttons: {
        LEFT: u32;
        RIGHT: u32;
    },
    touches: {
        ONE: u32;
        TWO: u32;
    },
    keys: {
        LEFT: char;
        RIGHT: char;
        UP: char;
        BOTTOM: char;
    },
};

pub const DEFAULT_ORBIT_CONTROLS_CONFIG: OrbitControlsConfig = {
    screen_space_panning: false,
    mouse_buttons: {
        LEFT: THREE.MOUSE.PAN as u32,
        RIGHT: THREE.MOUSE.ROTATE as u32,
    },
    touches: {
        ONE: THREE.TOUCH.PAN as u32,
        TWO: THREE.TOUCH.DOLLY_ROTATE as u32,
    },
    keys: {
        LEFT: 'A',
        RIGHT: 'D',
        UP: 'W',
        BOTTOM: 'S',
    },
};
```