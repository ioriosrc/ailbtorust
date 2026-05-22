```rust
use three::{
    geometry::{LineSegmentsGeometry, LineSegments2},
    material::LineMaterialWithAlphaVertex,
    scene::Scene,
    webgl::{WebGLRenderer, WebGLRenderTarget},
};
use super::{Marker, MarkerColorsToLinear};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct RenderableLineList {
    geometry: LineSegmentsGeometry,
    line_prepass: LineSegments2,
    line: LineSegments2,
}

impl RenderableLineList {
    pub fn new(
        topic: &str,
        marker: Marker,
        receive_time: Option<u64>,
        renderer: WebGlRenderer,
        options: Option<WebGLRenderTargetOptions>,
    ) -> Self {
        let world_units = options.unwrap_or_default().world_units;

        let line_options = LineRenderOptions {
            resolution: renderer.input.canvas_size,
            world_units,
        };

        let mat_line_prepass = make_line_prepass_material(marker, &line_options);
        let mat_line = make_line_material(marker, &line_options);

        let mut scene = Scene::new();
        let mut line_prepass = LineSegments2::new(&self.geometry, &mat_line_prepass);
        let mut line = LineSegments2::new(&self.geometry, &mat_line);

        line_prepass.render_order = 1;
        line_prepass.user_picking = false;
        scene.add(&line_prepass);

        line.render_order = 2;
        let pickingLineWidth = marker.scale.x * 1.2;
        let mut picking_material = make_line_picking_material(pickingLineWidth, &line_options);
        line.set_picking_material(&picking_material);

        self.update(marker, receive_time);
        Self { geometry, line_prepass, line }
    }

    pub fn dispose(&mut self) {
        self.line_prepass.material.dispose();
        self.line.material.dispose();

        let picking_material = self.line.user_picking_material().unwrap() as &LineMaterialWithAlphaVertex;
        picking_material.dispose();

        self.geometry.dispose();
    }

    pub fn update(&mut self, marker: Marker, receive_time: Option<u64>) {
        let prev_marker = self.marker.as_ref();
        super::update(marker, receive_time);
        let marker = self.marker.as_mut().unwrap();

        let points_length = marker.points.len();
        if points_length % 2 != 0 {
            points_length -= 1;
        }

        let lineWidth = marker.scale.x;
        let transparent = marker_has_transparency(marker);

        if transparent != marker_has_transparency(prev_marker) {
            self.line_prepass.material.transparent = transparent;
            self.line_prepass.material.depth_write = !transparent;
            self.line_prepass.material.needs_update = true;
            self.line.material.transparent = transparent;
            self.line.material.depth_write = !transparent;
            self.line.material.needs_update = true;
        }

        let mat_line_prepass = self.line_prepass.material as &LineMaterialWithAlphaVertex;
        mat_line_prepass.lineWidth = lineWidth;

        let mat_line = self.line.material as &LineMaterialWithAlphaVertex;
        mat_line.lineWidth = lineWidth;

        if points_length > self.position_buffer.len() {
            self.position_buffer.resize(points_length * 3, 0.0);
        }
        let positions = &mut self.position_buffer[..points_length * 3];

        for i in 0..points_length {
            let point = marker.points[i].unwrap();
            let offset = i * 3;
            positions[offset + 0] = point.x;
            positions[offset + 1] = point.y;
            positions[offset + 2] = point.z;
        }

        self.geometry.set_positions(positions);
        self.geometry.instance_count = points_length / 2;

        if points_length > self.color_buffer.len() {
            self.color_buffer.resize(points_length * 4, 0.0);
            let instance_color_buffer = THREE::InstancedInterleavedBuffer::new(
                &self.color_buffer,
                8,
                1,
            );
            self.geometry.setAttribute(
                "instanceColorStart",
                THREE::InterleavedBufferAttribute::new(
                    &instance_color_buffer,
                    4,
                    0,
                    true,
                ),
            );
            self.geometry.setAttribute(
                "instanceColorEnd",
                THREE::InterleavedBufferAttribute::new(
                    &instance_color_buffer,
                    4,
                    4,
                    true,
                ),
            );
        } else {
            self.geometry.setAttribute("instanceColorStart").needs_update = true;
            self.geometry.setAttribute("instanceColorEnd").needs_update = true;
        }

        let color_buffer = &mut self.color_buffer[..points_length * 4];

        MarkerColorsToLinear::new(marker, points_length, |color, i| {
            let offset = i * 4;
            color_buffer[offset + 0] = (255.0 * color[0]).floor() as f32;
            color_buffer[offset + 1] = (255.0 * color[1]).floor() as f32;
            color_buffer[offset + 2] = (255.0 * color[2]).floor() as f32;
            color_buffer[offset + 3] = (255.0 * color[3]).floor() as f32;
        });
    }
}
```