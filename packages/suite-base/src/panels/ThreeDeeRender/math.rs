```rust
use std::f32;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use cgmath::{Vector3, Zero};

pub fn get_rotation_to(src: Vector3<f32>, dest: Vector3<f32>) -> f32 {
    // Adapted from <https://www.ogre3d.org/docs/api/1.8/_ogre_vector3_8h_source.html>
    // Based on Stan Melax's article in Game Programming Gems
    let mut q = Vector3::zero();
    let v0 = src.normalize();
    let v1 = dest.normalize();

    let d = v0.dot(v1);
    if d >= 1.0 {
        return q.into_inner();
    }
    if d < 1e-6 - 1.0 {
        // Generate an axis
        let mut axis = Vector3::unit_x().cross(src).normalize();
        if is_zero_length(axis) {
            // Pick another if collinear
            axis = Vector3::unit_y().cross(src).normalize();
        }
        axis.normalize();
        q.set_axis_angle(axis, std::f32::PI);
    } else {
        let s = f32::sqrt((1 + d) * 2.0);
        let invs = 1.0 / s;

        let c = v0.cross(v1);

        q.x = c.x * invs;
        q.y = c.y * invs;
        q.z = c.z * invs;
        q.w = s * 0.5;
        q.normalize();
    }
    q.into_inner()
}

fn is_zero_length(vec: &Vector3<f32>) -> bool {
    vec.distance_squared() < f32::EPSILON * f32::EPSILON
}

pub fn approx_equals(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

pub fn vec3_tuple_approx_equals(
    a: [f32; 3],
    b: [f32; 3],
    epsilon: f32,
) -> bool {
    approx_equals(a[0], b[0], epsilon) &&
        approx_equals(a[1], b[1], epsilon) &&
        approx_equals(a[2], b[2], epsilon)
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
```