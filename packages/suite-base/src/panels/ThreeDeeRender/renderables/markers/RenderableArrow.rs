```rust
use std::f64;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use three::{geometry, material, utils::Color, BufferAttribute};

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

#[derive(Clone)]
pub struct RenderableArrow {
    shaft_mesh: mesh::Mesh,
    head_mesh: mesh::Mesh,
    shaft_outline: mesh::LineSegments,
    head_outline: mesh::LineSegments,
}

impl RenderableArrow {
    pub fn new(
        topic: &str,
        marker: Marker,
        receive_time: f64,
        renderer: &IRenderer,
    ) -> Self {
        let shaft_geometry = renderer.shared_geometry.get_or_compute(
            format!("{}-shaft-{}", topic, renderer.max_lod),
            || create_shaft_geometry(renderer.max_lod),
        );
        let head_geometry = renderer.shared_geometry.get_or_compute(
            format!("{}-head-{}", topic, renderer.max_lod),
            || create_head_geometry(renderer.max_lod),
        );
        let shaft_edges_geometry = renderer.shared_geometry.get_or_compute(
            format!("{}-shaftedges-{}", topic, renderer.max_lod),
            || create_shaft_edges_geometry(shaft_geometry),
        );
        let head_edges_geometry = renderer.shared_geometry.get_or_compute(
            format!("{}-headedges-{}", topic, renderer.max_lod),
            || create_head_edges_geometry(head_geometry),
        );

        // Shaft mesh
        let shaft_mesh = mesh::Mesh::new(
            geometry::CylinderGeometry {
                radius_top: 0.5,
                radius_bottom: 0.5,
                height: 1.0,
                radial_segments: arrow_shaft_subdivisions(renderer.max_lod),
                baseSegments: 1,
                open: false,
            },
            material::MeshStandardMaterial::new(Color::rgb(marker.color.r, marker.color.g, marker.color.b), Some(marker.color.a)),
        );
        shaft_mesh.cast_shadow = true;
        shaft_mesh.receive_shadow = true;

        // Head mesh
        let head_mesh = mesh::Mesh::new(
            geometry::ConeGeometry {
                radius_top: 0.5,
                radius_bottom: 1.0,
                height: 1.0,
                radial_segments: arrow_head_subdivisions(renderer.max_lod),
                baseSegments: 1,
                open: false,
            },
            material::MeshStandardMaterial::new(Color::rgb(marker.color.r, marker.color.g, marker.color.b), Some(marker.color.a)),
        );
        head_mesh.cast_shadow = true;
        head_mesh.receive_shadow = true;

        // Shaft outline
        let shaft_outline = mesh::LineSegments::new(
            geometry::EdgesGeometry::new(shaft_geometry.clone()),
            renderer.outline_material.clone(),
        );
        shaft_outline.set_user_data("picking", false);

        // Head outline
        let head_outline = mesh::LineSegments::new(
            geometry::EdgesGeometry::new(head_geometry.clone()),
            renderer.outline_material.clone(),
        );
        head_outline.set_user_data("picking", false);

        shaft_mesh.add(&shaft_outline);
        head_mesh.add(&head_outline);

        Self {
            shaft_mesh,
            head_mesh,
            shaft_outline,
            head_outline,
        }
    }

    pub fn dispose(&mut self) {
        self.shaft_mesh.material.dispose();
        self.head_mesh.material.dispose();
    }

    pub fn update(&mut self, marker: Marker, receive_time: f64) {
        super::update_marker!(self, marker, receive_time);

        let transparent = marker.color.a < 1.0;
        if transparent != self.shaft_mesh.material.transparent() {
            self.shaft_mesh.material.transparent = transparent;
            self.shaft_mesh.material.depth_write = !transparent;
            self.shaft_mesh.material.needs_update();
            self.head_mesh.material.transparent = transparent;
            self.head_mesh.material.depth_write = !transparent;
            self.head_mesh.material.needs_update();
        }

        let color = Color::rgb(marker.color.r, marker.color.g, marker.color.b);
        self.shaft_mesh.material.color.set(color);
        self.shaft_mesh.material.opacity = marker.color.a;

        if marker.points.len() == 2 {
            let point_a = marker.points[0].unwrap();
            let point_b = marker.points[1].unwrap();

            let mut temp_start = point_a;
            let mut temp_end = point_b;

            temp_direction.sub_vectors(&temp_end, &temp_start);
            let distance = temp_direction.magnitude();

            let head_length = HEAD_LENGTH_PROPORTION * distance;
            if marker.scale.z != 0.0 {
                let length = marker.scale.z;
                head_length = f64::clamp(length, 0.0, distance);
            }
            let shaft_length = distance - head_length;
            let shaft_diameter = marker.scale.x;
            let head_diameter = marker.scale.y;

            self.shaft_mesh.scale.set(shaft_length, shaft_diameter, shaft_diameter);
            self.head_mesh.scale.set(head_length, head_diameter, head_diameter);
            self.scale.set(1.0, 1.0, 1.0);

            let rotation = super::get_rotation_to(&UNIT_X, &temp_direction);
            self.shaft_mesh.rotation_from_quaternion(rotation);
            self.head_mesh.rotation.copy_from(&self.shaft_mesh.rotation);
        } else {
            self.shaft_mesh.scale.set(SHAFT_LENGTH, SHAFT_DIAMETER, SHAFT_DIAMETER);
            self.head_mesh.scale.set(HEAD_LENGTH, HEAD_DIAMETER, HEAD_DIAMETER);
            self.scale.set(marker.scale.x, marker.scale.y, marker.scale.z);

            let half_shaft_length = SHAFT_LENGTH / 2.0;
            let half_head_length = HEAD_LENGTH / 2.0;
            self.shaft_mesh.position.set(half_shaft_length, 0.0, 0.0);
            self.head_mesh.position.set(half_shaft_length * 2.0 + half_head_length, 0.0, 0.0);
            self.shaft_mesh.rotation.set(0.0, 0.0, 0.0);
            self.head_mesh.rotation.set(0.0, 0.0, 0.0);
        }
    }
}

fn create_shaft_geometry(lod: i32) -> geometry::CylinderGeometry {
    let subdivs = arrow_shaft_subdivisions(lod);
    geometry::CylinderGeometry {
        radius_top: 0.5,
        radius_bottom: 0.5,
        height: 1.0,
        radial_segments: subdivs,
        baseSegments: 1,
        open: false,
    }
}

fn create_head_geometry(lod: i32) -> geometry::ConeGeometry {
    let subdivs = arrow_head_subdivisions(lod);
    geometry::ConeGeometry {
        radius_top: 0.5,
        radius_bottom: 1.0,
        height: 1.0,
        radial_segments: subdivs,
        baseSegments: 1,
        open: false,
    }
}

fn create_shaft_edges_geometry(shaft_geometry: &geometry::CylinderGeometry) -> geometry::EdgesGeometry {
    let shaft_edges_geometry = geometry::EdgesGeometry::new(shaft_geometry.clone());
    let positions_attrib = shaft_edges_geometry.attribute("position").unwrap();
    let mut positions = Vec::from(positions_attrib.data().clone());

    let new_count = (positions.len() / 3) * 2;
    let new_vertices = &mut positions[new_count..];

    let new_positions_attrib = BufferAttribute::new(&new_vertices, 3);
    shaft_edges_geometry.set_attribute("position", new_positions_attrib);
    shaft_edges_geometry.compute_bounding_sphere();
    shaft_edges_geometry
}

fn create_head_edges_geometry(head_geometry: &geometry::ConeGeometry) -> geometry::EdgesGeometry {
    let head_edges_geometry = geometry::EdgesGeometry::new(head_geometry.clone());
    head_edges_geometry.compute_bounding_sphere();
    head_edges_geometry
}
```