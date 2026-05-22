```rust
use std::vec::Vec;

use three::{BufferAttribute, BufferGeometry, Material, Program, RenderTarget, Texture2D};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use three::{Camera, Mesh, Scene};
use wasm_bindgen::JsValue;

pub struct LineMaterialWithAlphaVertex {
  // Implementation of LineMaterialWithAlphaVertex
}

pub struct ColorRGBA {
  r: f32,
  g: f32,
  b: f32,
  a: f32,
}

pub enum MarkerType {
  ARROW,
  CUBE,
  SPHERE,
  CYLINDER,
  TEXT_VIEW_FACING,
  MESH_RESOURCE,
  LINE_STRIP,
  LINE_LIST,
  CUBE_LIST,
  SPHERE_LIST,
  POINTS,
  TRIANGLE_LIST,
}

#[derive(Default)]
pub struct Marker {
  scale: Vec3<f32>,
  color: ColorRGBA,
  points: Vec<[f32; 3]>,
  colors: Vec<ColorRGBA>,
  // Other marker properties
}

impl LineOptions {
  fn new(resolution: Vec2<f32>) -> Self {
    Self { resolution }
  }

  fn world_units(&self) -> bool {
    true // Default value for demonstration purposes
  }
}

fn make_standard_material(color: ColorRGBA) -> Material {
  let mut material = Material::new();
  material.color = color.to_color4();
  material.metalness = 0.0;
  material.roughness = 1.0;
  material.dithering = true;
  material.opacity = color.a;
  material.transparent = color.a < 1.0;
  material.depth_write = color.a == 1.0;
  material.clone()
}

fn make_standard_vertex_color_material(marker: Marker) -> Material {
  let transparent = marker_has_transparency(&marker);
  let mut material = Material::new();
  material.metalness = 0.0;
  material.roughness = 1.0;
  material.dithering = true;
  material.vertex_colors = true;
  material.side = MeshSide::DoubleSide;
  material.opacity = 1.0;
  material.transparent = transparent;
  material.depth_write = !transparent;
  material.clone()
}

fn make_standard_instanced_material(marker: Marker) -> Material {
  let transparent = marker_has_transparency(&marker);
  let mut material = Material::new();
  material.metalness = 0.0;
  material.roughness = 1.0;
  material.dithering = true;
  material.opacity = 1.0;
  material.transparent = transparent;
  material.depth_write = !transparent;
  material.clone()
}

fn make_line_prepass_material(marker: Marker, options: LineOptions) -> LineMaterialWithAlphaVertex {
  let width = marker.scale.x;
  let transparent = marker_has_transparency(&marker);
  let mut material = LineMaterialWithAlphaVertex::new();
  material.world_units = options.world_units.unwrap_or(true);
  material.color_write = false;
  material.transparent = transparent;
  material.depth_write = !transparent;
  material.linewidth = width;
  material.resolution = options.resolution.clone();

  material.stencil_write = true;
  material.stencil_ref = 1;
  material.stencil_z_pass = StencilOp::Replace;

  material.clone()
}

fn make_line_material(marker: Marker, options: LineOptions) -> LineMaterialWithAlphaVertex {
  let width = marker.scale.x;
  let transparent = marker_has_transparency(&marker);
  let mut material = LineMaterialWithAlphaVertex::new();
  material.world_units = options.world_units.unwrap_or(true);
  material.vertex_colors = true;
  material.linewidth = width;
  material.transparent = transparent;
  material.depth_write = !transparent;
  material.resolution = options.resolution.clone();

  material.stencil_write = true;
  material.stencil_ref = 0;
  material.stencil_func = StencilFunc::NotEqual;
  material.stencil_fail = StencilOp::Replace;

  material.clone()
}

fn make_line_picking_material(line_width: f32, options: LineOptions) -> ShaderMaterial {
  let mut material = ShaderMaterial::new();
  material.vertex_shader = vertex_shader().to_string();
  material.fragment_shader = fragment_shader().to_string();
  material.clipping = true;
  material.uniforms.insert("objectId".into(), JsValue::from(marker.id()));
  material.uniforms.insert("linewidth".into(), line_width);
  material.uniforms.insert("resolution".into(), options.resolution.clone());
  material.defines.insert("WORLD_UNITS".into(), "".to_string());

  material.clone()
}

fn make_points_material(marker: Marker) -> PointsMaterial {
  let transparent = marker_has_transparency(&marker);
  let mut material = PointsMaterial::new();
  material.vertex_colors = true;
  material.size = marker.scale.x;
  material.size_attenuation = true;
  material.transparent = transparent;
  material.depth_write = !transparent;
  material.clone()
}

fn vertex_shader() -> &str {
  r#"
    varying vec4 color;

    void main() {
      gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
      color = vertexColor;
    }
  "#}
  .trim_start()
}

fn fragment_shader() -> &str {
  r#"
    uniform vec4 objectId;
    uniform float linewidth;

    void main() {
      gl_FragColor = objectId;
      if (linewidth > 1.0) {
        gl_FragColor.a *= linewidth;
      }
    }
  "#}
  .trim_start()
}

fn marker_has_transparency(marker: &Marker) -> bool {
  let num_colors = marker.colors.len();
  for i in 0..num_colors {
    if marker.colors[i].a < 1.0 {
      return true;
    }
  }
  false
}
```