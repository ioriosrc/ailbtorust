```rust
use bevy::{prelude::*};
use std::time::Duration;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use three::core::{Matrix4, Quaternion};
use three::prelude::*;

use crate::{
    renderable_primitive::{Color, CubePrimitive, RenderablePrimitive},
    shared_geometry,
};

const MAX_INSTANCES: u32 = 16;

pub struct RenderableCubes {
    geometry: BoxGeometry,
    instance_opacity: InstanceBufferAttribute<Float32>,
    material: MeshStandardMaterialWithInstanceOpacity,
    max_instances: u32,
}

impl RenderableCubes {
    pub fn new(renderer: &Renderer) -> Self {
        let shared_geometry = renderer.shared_geometry();
        let cube_geometry = shared_geometry.get_geometry("cube", create_cube_geometry).clone();
        let outline_geometry = shared_geometry.get_geometry("edges", || create_edges_geometry(&cube_geometry));

        let material = MeshStandardMaterialWithInstanceOpacity {
            metalness: 0.,
            roughness: 1.,
            dithering: true,
        };

        Self {
            geometry: cube_geometry,
            instance_opacity: InstanceBufferAttribute::new(Float32, MAX_INSTANCES),
            material,
            max_instances,
        }
    }

    fn ensure_capacity(&mut self, num_cubes: u32) {
        if num_cubes > self.max_instances {
            let new_capacity = (num_cubes as f64 * 1.5).ceil() as u32 + MAX_INSTANCES;
            self.max_instances = new_capacity;

            self.clear();
            self.geometry.dispose();
            self.instance_opacity.clear();
            self.material.dispose();

            let cube_geometry = shared_geometry.get_geometry("cube", create_cube_geometry).clone();
            let outline_geometry = shared_geometry.get_geometry("edges", || create_edges_geometry(&cube_geometry));

            let material = MeshStandardMaterialWithInstanceOpacity {
                metalness: 0.,
                roughness: 1.,
                dithering: true,
            };

            self.geometry = cube_geometry;
            self.instance_opacity = InstanceBufferAttribute::new(Float32, new_capacity);
            self.material = material;
        }
    }

    fn update_mesh(&mut self, cubes: &[CubePrimitive]) {
        let is_transparent = cubes.iter().any(|cube| cube.color.a < 1.);

        self.ensure_capacity(cubes.len());

        let override_color = if let Some(color) = self.settings.color {
            Color::from_str(color).unwrap()
        } else {
            Color::new(cube.color.r, cube.color.g, cube.color.b)
        };

        for (i, cube) in cubes.iter().enumerate() {
            self.mesh.setColorAt(i, color.rgb());
            self.instance_opacity.set_x(i, color.a);
            let matrix = Matrix4::compose(
                cube.pose.position(),
                Quaternion::from_rotation_euler(cube.pose.orientation()),
                Vec3::new(cube.size.x, cube.size.y, cube.size.z),
            );
            self.mesh.set_matrix_at(i, matrix);
        }

        if self.material.transparent != is_transparent {
            self.material.transparent = is_transparent;
            self.material.depth_write = !is_transparent;
            self.material.needs_update();
        }

        if self.mesh.count() == 0 && cubes.len() > 0 {
            // needed to make colors work: https://discourse.threejs.org/t/instancedmesh-color-doesnt-work-when-initial-count-is-0/41355
            self.material.needs_update();
        }
        self.mesh.count = cubes.len();
        self.outline_geometry.instance_count = cubes.len();
        self.mesh.instance_matrix.needs_update();
        self.instance_opacity.needs_update();

        if self.mesh.instance_color.is_some() {
            self.mesh.instance_color.needs_update();
        }
    }

    fn clear(&mut self) {
        // Clear and dispose existing resources
        self.geometry.clear();
        self.instance_opacity.clear();
        self.material.dispose();
        self.outline_geometry.clear();
    }

    pub fn update(
        &mut self,
        topic: Option<&str>,
        entity: Option<SceneEntity>,
        settings: LayerSettingsEntity,
        receive_time: Duration,
    ) {
        super::RenderablePrimitive::update(self, topic, entity, settings, receive_time);
        if let Some(cubes) = entity.and_then(|e| e.cubes) {
            self.update_mesh(&cubes);
            self.outline.visible = settings.show_outlines.unwrap_or(true);
        }
    }

    pub fn update_settings(&mut self, settings: LayerSettingsEntity) {
        self.update(self.settings.topic.clone(), None, settings, self.receive_time);
    }
}

fn create_cube_geometry() -> BoxGeometry {
    let cube_geometry = BoxGeometry::new(1.0, 1.0, 1.0);
    cube_geometry.computeBoundingSphere();
    cube_geometry
}

fn create_edges_geometry(cube_geometry: &BoxGeometry) -> EdgesGeometry {
    let cube_edges_geometry = EdgesGeometry::new(cube_geometry, 40);
    cube_edges_geometry.computeBoundingSphere();
    cube_edges_geometry
}
```