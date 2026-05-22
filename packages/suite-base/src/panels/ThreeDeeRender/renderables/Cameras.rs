```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

/// A TypeScript/React-like camera information object
#[derive(Serialize, Deserialize)]
struct CameraInfo {
    /// The header of the camera information message
    pub header: Header,
    /// The frame ID of the camera information message
    pub frame_id: String,
    /// The matrix P representing the camera calibration
    pub p: Matrix4<f64>,
}

/// A TypeScript/React-like header object
#[derive(Serialize, Deserialize)]
struct Header {
    /// The stamp of the timestamp in the message
    pub stamp: std::time::Instant,
    /// The namespace of the message
    pub ns: String,
    /// The ID of the message
    pub id: u32,
}

/// A TypeScript/React-like matrix object
#[derive(Serialize, Deserialize)]
struct Matrix4<T> {
    a11: T,
    a12: T,
    a13: T,
    a14: T,
    a21: T,
    a22: T,
    a23: T,
    a24: T,
    a31: T,
    a32: T,
    a33: T,
    a34: T,
    a41: T,
    a42: T,
    a43: T,
    a44: T,
}

/// A TypeScript/React-like renderable object
#[derive(Serialize, Deserialize)]
struct Renderable {
    // Implement the Renderable struct here
}

// Define similar structures and types as in the original TypeScript code for other components

fn main() {
    // Example usage of the CameraInfoRenderable
    let mut camera_info_renderable = CameraInfoRenderable::new("example_topic");
    camera_info_renderable.update(CameraInfo {
        header: Header {
            stamp: std::time::Instant::now(),
            ns: "example_namespace".to_string(),
            id: 1,
        },
        frame_id: "example_frame_id".to_string(),
        p: Matrix4 {
            a11: 1.0, // Example values
            a12: 0.0,
            a13: 0.0,
            a14: 0.0,
            a21: 0.0,
            a22: 1.0,
            a23: 0.0,
            a24: 0.0,
            a31: 0.0,
            a32: 0.0,
            a33: 1.0,
            a34: 0.0,
            a41: 0.0,
            a42: 0.0,
            a43: 0.0,
            a44: 1.0,
        },
    });
}
```