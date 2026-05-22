```rust
// SPDX-FileCopyrightText: Copyright (C) 2023 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::ops::{Add, Div, Mul};
use three::{
    materials::ShaderMaterial,
    uniforms::{Uniform, UniformsSet},
    vec3::Vec3,
};

#[derive(Clone, Debug)]
pub struct LineMaterialParameters {
    pub alpha_to_coverage: Option<bool>;
    pub color: Option<Vec4>;
    pub dashed: Option<bool>;
    pub dash_scale: Option<f64>;
    pub dash_size: Option<f64>;
    pub dash_offset: Option<f64>;
    pub gap_size: Option<f64>;
    pub linewidth: Option<f64>;
    pub resolution: Option<(f64, f64)>;
    pub wireframe: Option<bool>;
    pub world_units: Option<bool>;
}

pub struct LineMaterialWithAlphaVertex {
    material: ShaderMaterial,
}

impl LineMaterialWithAlphaVertex {
    pub fn new(parameters: LineMaterialParameters) -> Self {
        let mut uniforms = UniformsSet::new();
        uniforms.insert("type", &"LineMaterial".to_string());
        uniforms.insert("uniforms", &ShaderMaterialUniforms {
            diffuse: parameters.color,
            linewidth: parameters.linewidth,
            dashed: parameters.dashed,
            dash_scale: parameters.dash_scale,
            dash_size: parameters.dash_size,
            dash_offset: parameters.dash_offset,
            gap_size: parameters.gap_size,
            resolution: parameters.resolution.map(|(x, y)| (x as f32, y as f32)),
        });

        let mut defines = std::collections::HashMap::new();
        if let Some(world_units) = parameters.world_units {
            if world_units {
                defines.insert("WORLD_UNITS", "");
            }
        }

        LineMaterialWithAlphaVertex {
            material: ShaderMaterial {
                type_: "LineMaterial",
                uniforms,
                defines,
                clipping: true, // required for clipping support
            },
        }
    }

    pub fn get_color(&self) -> Option<Vec4> {
        self.material.uniforms.get("diffuse").map(|v| v.to_vec4())
    }

    pub fn set_color(&mut self, value: Vec4) {
        self.material.uniforms.insert("diffuse", &value);
    }

    pub fn get_world_units(&self) -> bool {
        self.material.defines.contains_key("WORLD_UNITS")
    }

    pub fn set_world_units(&mut self, value: bool) {
        if value {
            self.material.defines.insert("WORLD_UNITS", "");
        } else {
            self.material.defines.remove("WORLD_UNITS");
        }
    }

    pub fn get_linewidth(&self) -> Option<f64> {
        self.material.uniforms.get("linewidth").map(|v| v.to_f64())
    }

    pub fn set_linewidth(&mut self, value: f64) {
        self.material.uniforms.insert("linewidth", &value);
    }

    pub fn get_dashed(&self) -> bool {
        self.material.defines.contains_key("USE_DASH")
    }

    pub fn set_dashed(&mut self, value: bool) {
        if value {
            self.material.defines.insert("USE_DASH", "");
        } else {
            self.material.defines.remove("USE_DASH");
        }
    }

    pub fn get_dash_scale(&self) -> Option<f64> {
        self.material.uniforms.get("dash_scale").map(|v| v.to_f64())
    }

    pub fn set_dash_scale(&mut self, value: f64) {
        self.material.uniforms.insert("dash_scale", &value);
    }

    pub fn get_dash_size(&self) -> Option<f64> {
        self.material.uniforms.get("dash_size").map(|v| v.to_f64())
    }

    pub fn set_dash_size(&mut self, value: f64) {
        self.material.uniforms.insert("dash_size", &value);
    }

    pub fn get_dash_offset(&self) -> Option<f64> {
        self.material.uniforms.get("dash_offset").map(|v| v.to_f64())
    }

    pub fn set_dash_offset(&mut self, value: f64) {
        self.material.uniforms.insert("dash_offset", &value);
    }

    pub fn get_gap_size(&self) -> Option<f64> {
        self.material.uniforms.get("gap_size").map(|v| v.to_f64())
    }

    pub fn set_gap_size(&mut self, value: f64) {
        self.material.uniforms.insert("gap_size", &value);
    }
}

#[derive(Debug)]
struct ShaderMaterialUniforms<'a> {
    diffuse: Option<&'a Vec4>,
    linewidth: Option<&'a f32>,
    dashed: Option<&'a bool>,
    dash_scale: Option<&'a f64>,
    dash_size: Option<&'a f64>,
    dash_offset: Option<&'a f64>,
    gap_size: Option<&'a f64>,
    resolution: Option<&'a (f32, f32)>,
}

#[derive(Debug)]
pub struct LineMaterialWithAlphaVertex uniforms {
    diffuse: Uniform<Vec4>,
    linewidth: Uniform<f32>,
    dashed: Uniform<bool>,
    dash_scale: Uniform<f64>,
    dash_size: Uniform<f64>,
    dash_offset: Uniform<f64>,
    gap_size: Uniform<f64>,
    resolution: Uniform<(f32, f32)>,
}
```