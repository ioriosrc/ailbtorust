```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use three::{
    BufferAttribute, BufferGeometry, MaterialWithAlphaVertex, Mesh, Object3D,
    Raycaster, Vector3, Camera, PerspectiveCamera, Plane, WebGLRenderer,
};

struct LinePrimitiveRenderable {
    geometry: Option<LineSegmentsGeometry | LineGeometry>,
    position_buffer: Option<Vec<f32>>,
    color_buffer: Option<Vec<f32>>,
    material: MaterialWithAlphaVertex,
    picking_material: PickingMaterial,
    transparent: bool,
    line_type: Option<&str>,
    primitive_changed: bool,
    primitive: Option<LinePrimitive>,
}

impl LinePrimitiveRenderable {
    pub fn new(primitive: &LinePrimitive, canvas_size: Vector3) -> Self {
        let mut material = MaterialWithAlphaVertex::new(&primitive.thickness, !primitive.scale_invariant);
        material.line_width = primitive.thickness; // Fix for THREE.js type annotations

        let picking_material = PickingMaterial::new();
        picking_material.resolution.set(canvas_size.x(), canvas_size.y());
        picking_material.world_units = !primitive.scale_invariant;
        // make sure thin, scale_invariant lines are still pickable
        picking_material.line_width = primitive.scale_invariant
            .map_or(MIN_PICKING_LINE_WIDTH_PX, |width| width.max(primitive.thickness));
        picking_material.needs_update = true;

        Self {
            geometry: None,
            position_buffer: None,
            color_buffer: None,
            material,
            picking_material,
            transparent: true,
            line_type: Some(&primitive.type),
            primitive_changed: true,
            primitive: Some(primitive.clone()),
        }
    }

    pub fn update(&mut self, primitive: &LinePrimitive) {
        if !self.primitive.is_equal(primitive) || self.primitive_changed {
            self.primitive = Some(primitive.clone());
            self.primitive_changed = false;
            self.geometry = None;
            self.position_buffer = None;
            self.color_buffer = None;

            let num_vertices =
                (primitive.indices.len() > 0) as usize + if primitive.type == "LINE_LOOP" { 1 } else { 0 };
            let necessary_position_buffer_length = num_vertices * 3;

            if let Some(positions_out) = &mut self.position_buffer {
                assert!(positions_out.len() >= necessary_position_buffer_length, "Position buffer must have a length ({:?})  >= to primitive.points.length ({:?}) * 3", positions_out.len(), necessary_position_buffer_length);
                for &point in &primitive.points {
                    let [x, y, z] = point.to_array();
                    positions_out.extend(vec![x, y, z]);
                }
            }

            let is_loop = primitive.type == "LINE_LOOP";
            if is_loop && positions_out.len() > 3 {
                positions_out.copy_within(0, 3);
            }
        }

        if let Some(color) = primitive.color {
            self.color_buffer = Some(vec![
                SRGBToLinear(color.r()),
                SRGBToLinear(color.g()),
                SRGBToLinear(color.b()),
                color.a(),
            ]);

            self.update_material();
        } else {
            let single_color = primitive.colors
                .iter()
                .next()
                .map(|c| SRGBToLinear(c.to_array()))
                .unwrap_or([0.0, 0.0, 0.0, 1.0]);

            if let Some(colors_out) = &mut self.color_buffer {
                assert!(colors_out.len() >= primitive.colors.len() * 4, "Colors buffer must have a length ({:?}) >= to the primitive.colors.length ({:?}) * 4", colors_out.len(), primitive.colors.len());
                for color in primitive.colors {
                    let [r, g, b, a] = color.to_array();
                    colors_out.extend(vec![r, g, b, a]);
                }
            }

            self.update_material();
        }

        if let Some(line_type) = &self.line_type {
            self.material.line_width = match line_type {
                "LINE_LOOP" => primitive.thickness,
                _ => primitive.thickness,
            };
        } else {
            self.material.line_width = primitive.thickness;
        }
    }

    fn update_material(&mut self) {
        if let Some(primitive) = &self.primitive {
            self.material.transparent = primitive.scale_invariant();
            self.material.world_units = !primitive.scale_invariant();

            // Update material uniforms
            self.material.uniforms.object_id.value = [0.0, 0.0, 0.0, 0.0];
        }
    }

    pub fn dispose(&mut self) {
        if let Some(geometry) = &self.geometry.take() {
            geometry.dispose();
        }

        if let Some(position_buffer) = self.position_buffer.take() {
            position_buffer.clear();
        }

        if let Some(color_buffer) = self.color_buffer.take() {
            color_buffer.clear();
        }
    }
}

struct PickingMaterial {
    object_id: Vec<f32>,
}

impl PickingMaterial {
    pub fn new() -> Self {
        PickingMaterial { object_id: vec![std::f32::NAN; 4] }
    }

    pub fn on_before_compile(&mut self, shader: &mut Shader, renderer: &WebGLRenderer) {
        shader.fragment_shader = /* glsl */ "
            uniform vec4 objectId;
            void main() {
                gl_FragColor = objectId;
            }
        ";
    }
}

fn SRGBToLinear(color: Vec<f32>) -> f32 {
    color[0] / 255.0 + (color[1] - 0.5) * 128.0 / 255.0 + (color[2] - 0.5) * 64.0 / 255.0
}

#[derive(Debug)]
struct LinePrimitive {
    points: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    type_: &str,
    scale_invariant: bool,
    thickness: f32,
}
```