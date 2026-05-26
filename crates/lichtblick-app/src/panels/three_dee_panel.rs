// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! 3D Panel (ThreeDeeRender) - WebGL2 based 3D scene viewer.
//! Renders: grid, axes, point clouds, markers, TF frames.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram, WebGlShader,
    WebGlVertexArrayObject,
};

use crate::state::app_state::{get_player, use_app_state, use_layout_state, NodeId};
use crate::extensions::manager::{js_has_converters, js_convert_message_with_object, js_convert_message_to_scene};
use super::tf_tree::{decode_tf_message_cdr, is_tf_schema, TfTree, StampedTransform, Quaternion, Vec3d};
use lichtblick_panels::three_dee::TopicDisplayConfig;

// ============ Math Types ============

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len < 1e-10 {
            return *self;
        }
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(&self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

/// 4x4 matrix in column-major order (OpenGL convention)
#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub data: [f32; 16],
}

impl Mat4 {
    pub fn identity() -> Self {
        let mut data = [0.0f32; 16];
        data[0] = 1.0;
        data[5] = 1.0;
        data[10] = 1.0;
        data[15] = 1.0;
        Self { data }
    }

    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y / 2.0).tan();
        let nf = 1.0 / (near - far);
        let mut m = [0.0f32; 16];
        m[0] = f / aspect;
        m[5] = f;
        m[10] = (far + near) * nf;
        m[11] = -1.0;
        m[14] = 2.0 * far * near * nf;
        Self { data: m }
    }

    pub fn orthographic(distance: f32, aspect: f32, near: f32, far: f32) -> Self {
        let half_h = distance * 0.5;
        let half_w = half_h * aspect;
        let mut m = [0.0f32; 16];
        m[0] = 1.0 / half_w;
        m[5] = 1.0 / half_h;
        m[10] = -2.0 / (far - near);
        m[14] = -(far + near) / (far - near);
        m[15] = 1.0;
        Self { data: m }
    }

    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let f = center.sub(&eye).normalize();
        let s = f.cross(&up).normalize();
        let u = s.cross(&f);

        let mut m = Self::identity();
        m.data[0] = s.x;
        m.data[4] = s.y;
        m.data[8] = s.z;
        m.data[1] = u.x;
        m.data[5] = u.y;
        m.data[9] = u.z;
        m.data[2] = -f.x;
        m.data[6] = -f.y;
        m.data[10] = -f.z;
        m.data[12] = -s.dot(&eye);
        m.data[13] = -u.dot(&eye);
        m.data[14] = f.dot(&eye);
        m
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let mut out = [0.0f32; 16];
        for col in 0..4 {
            for row in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.data[row + k * 4] * other.data[k + col * 4];
                }
                out[row + col * 4] = sum;
            }
        }
        Self { data: out }
    }
}

// ============ Orbit Camera ============

#[derive(Clone, Copy, Debug)]
pub struct OrbitCamera {
    pub target: Vec3,
    pub distance: f32,
    pub azimuth: f32,   // radians around Y axis
    pub elevation: f32, // radians up from XZ plane
    pub fov_y: f32,
    pub perspective: bool,
    pub near: f32,
    pub far: f32,
}

impl OrbitCamera {
    pub fn new() -> Self {
        Self {
            target: Vec3::new(0.0, 0.0, 0.0),
            distance: 15.0,
            azimuth: std::f32::consts::FRAC_PI_4,    // 45°
            elevation: std::f32::consts::FRAC_PI_6,  // 30°
            fov_y: std::f32::consts::FRAC_PI_4,      // 45° fov
            perspective: true,
            near: 0.5,
            far: 5000.0,
        }
    }

    pub fn eye_position(&self) -> Vec3 {
        let x = self.distance * self.elevation.cos() * self.azimuth.sin();
        let y = self.distance * self.elevation.sin();
        let z = self.distance * self.elevation.cos() * self.azimuth.cos();
        self.target.add(&Vec3::new(x, y, z))
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.eye_position(), self.target, Vec3::new(0.0, 1.0, 0.0))
    }

    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        if self.perspective {
            Mat4::perspective(self.fov_y, aspect, self.near, self.far)
        } else {
            Mat4::orthographic(self.distance, aspect, self.near, self.far)
        }
    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {
        self.azimuth -= dx * 0.01;
        self.elevation += dy * 0.01;
        self.elevation = self.elevation.clamp(-1.4, 1.4); // ~±80°
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance *= (1.0 + delta * 0.001).max(0.1);
        self.distance = self.distance.clamp(0.5, 500.0);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let right = Vec3::new(self.azimuth.cos(), 0.0, -self.azimuth.sin());
        let up = Vec3::new(0.0, 1.0, 0.0);
        let scale = self.distance * 0.002;
        self.target = self.target.add(&right.scale(-dx * scale)).add(&up.scale(dy * scale));
    }
}

// ============ Shader Sources ============

const GRID_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec4 a_color;
uniform mat4 u_viewProjection;
uniform mat4 u_modelMatrix;
out vec4 v_color;
void main() {
    gl_Position = u_viewProjection * u_modelMatrix * vec4(a_position, 1.0);
    v_color = a_color;
}
"#;

const GRID_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
in vec4 v_color;
out vec4 fragColor;
void main() {
    fragColor = v_color;
}
"#;

const POINT_CLOUD_VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec4 a_color;
uniform mat4 u_viewProjection;
uniform float u_pointSize;
out vec4 v_color;
void main() {
    gl_Position = u_viewProjection * vec4(a_position, 1.0);
    gl_PointSize = u_pointSize;
    v_color = a_color;
}
"#;

const POINT_CLOUD_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;
in vec4 v_color;
out vec4 fragColor;
void main() {
    // Circular point shape
    vec2 coord = gl_PointCoord - vec2(0.5);
    if (dot(coord, coord) > 0.25) discard;
    fragColor = v_color;
}
"#;

// ============ Line Shader (TF Axes) ============

const LINE_VERT_SHADER: &str = r#"#version 300 es
precision highp float;
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_color;
uniform mat4 u_viewProjection;
uniform mat4 u_modelMatrix;
out vec3 v_color;
void main() {
    v_color = a_color;
    gl_Position = u_viewProjection * u_modelMatrix * vec4(a_position, 1.0);
}
"#;

const LINE_FRAG_SHADER: &str = r#"#version 300 es
precision mediump float;
in vec3 v_color;
out vec4 outColor;
void main() {
    outColor = vec4(v_color, 1.0);
}
"#;

// ============ Uniform Color Shader (Cubes & Scene Lines) ============

const UNIFORM_COLOR_VERT: &str = r#"#version 300 es
precision highp float;
layout(location = 0) in vec3 a_position;
uniform mat4 u_viewProjection;
uniform mat4 u_modelMatrix;
void main() {
    gl_Position = u_viewProjection * u_modelMatrix * vec4(a_position, 1.0);
}
"#;

const UNIFORM_COLOR_FRAG: &str = r#"#version 300 es
precision mediump float;
uniform vec4 u_color;
out vec4 outColor;
void main() {
    outColor = u_color;
}
"#;

// ============ WebGL Helpers ============

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or("Failed to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let log = gl.get_shader_info_log(&shader).unwrap_or_default();
        gl.delete_shader(Some(&shader));
        Err(format!("Shader compile error: {}", log))
    }
}

fn link_program(gl: &GL, vs: &WebGlShader, fs: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("Failed to create program")?;
    gl.attach_shader(&program, vs);
    gl.attach_shader(&program, fs);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let log = gl.get_program_info_log(&program).unwrap_or_default();
        gl.delete_program(Some(&program));
        Err(format!("Program link error: {}", log))
    }
}

// ============ Grid + Axes Geometry ============

/// Generate grid lines on the XZ plane with coordinate axes.
fn generate_grid_and_axes(size: i32, spacing: f32, grid_color: [f32; 4]) -> (Vec<f32>, Vec<f32>) {
    let mut positions: Vec<f32> = Vec::new();
    let mut colors: Vec<f32> = Vec::new();

    let half = size as f32 * spacing;

    // Grid lines (use custom color)
    let cr = grid_color[0];
    let cg = grid_color[1];
    let cb = grid_color[2];
    let ca = grid_color[3];

    for i in -size..=size {
        let pos = i as f32 * spacing;
        let alpha = if i == 0 { 0.0 } else { ca }; // Skip center lines (axes go there)

        // Line along X
        positions.extend_from_slice(&[-half, 0.0, pos, half, 0.0, pos]);
        colors.extend_from_slice(&[cr, cg, cb, alpha, cr, cg, cb, alpha]);

        // Line along Z
        positions.extend_from_slice(&[pos, 0.0, -half, pos, 0.0, half]);
        colors.extend_from_slice(&[cr, cg, cb, alpha, cr, cg, cb, alpha]);
    }

    // X axis (red)
    positions.extend_from_slice(&[0.0, 0.0, 0.0, half, 0.0, 0.0]);
    colors.extend_from_slice(&[1.0, 0.2, 0.2, 1.0, 1.0, 0.2, 0.2, 1.0]);

    // Y axis (green) - up
    positions.extend_from_slice(&[0.0, 0.0, 0.0, 0.0, half * 0.5, 0.0]);
    colors.extend_from_slice(&[0.2, 1.0, 0.2, 1.0, 0.2, 1.0, 0.2, 1.0]);

    // Z axis (blue)
    positions.extend_from_slice(&[0.0, 0.0, 0.0, 0.0, 0.0, half]);
    colors.extend_from_slice(&[0.2, 0.2, 1.0, 1.0, 0.2, 0.2, 1.0, 1.0]);

    (positions, colors)
}

/// Parse a hex color string (e.g. "#248eff33" or "#ff0000") into [r, g, b, a] floats.
fn parse_hex_color(hex: &str) -> [f32; 4] {
    let h = hex.trim_start_matches('#');
    match h.len() {
        6 => {
            let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(128) as f32 / 255.0;
            let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(128) as f32 / 255.0;
            let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(128) as f32 / 255.0;
            [r, g, b, 1.0]
        }
        8 => {
            let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(128) as f32 / 255.0;
            let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(128) as f32 / 255.0;
            let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(128) as f32 / 255.0;
            let a = u8::from_str_radix(&h[6..8], 16).unwrap_or(255) as f32 / 255.0;
            [r, g, b, a]
        }
        _ => [0.5, 0.5, 0.5, 0.3],
    }
}

// ============ Scene Entity Types ============

#[derive(Clone, Debug)]
struct SceneCube {
    frame_id: String,
    px: f32, py: f32, pz: f32,
    ox: f32, oy: f32, oz: f32, ow: f32,
    sx: f32, sy: f32, sz: f32,
    r: f32, g: f32, b: f32, a: f32,
}

#[derive(Clone, Debug)]
struct SceneLine {
    frame_id: String,
    line_type: u32, // 0=LINE_STRIP, 1=LINE_LOOP, 2=LINE_LIST
    px: f32, py: f32, pz: f32,
    ox: f32, oy: f32, oz: f32, ow: f32,
    r: f32, g: f32, b: f32, a: f32,
    points: Vec<f32>, // x,y,z triples
}

/// Generate unit cube wireframe: 12 edges as line pairs
fn generate_unit_cube_wireframe() -> Vec<f32> {
    // 8 vertices of unit cube centered at origin (-0.5 to 0.5)
    let v: [[f32; 3]; 8] = [
        [-0.5, -0.5, -0.5], [ 0.5, -0.5, -0.5],
        [ 0.5,  0.5, -0.5], [-0.5,  0.5, -0.5],
        [-0.5, -0.5,  0.5], [ 0.5, -0.5,  0.5],
        [ 0.5,  0.5,  0.5], [-0.5,  0.5,  0.5],
    ];
    // 12 edges
    let edges: [(usize, usize); 12] = [
        (0,1),(1,2),(2,3),(3,0), // back face
        (4,5),(5,6),(6,7),(7,4), // front face
        (0,4),(1,5),(2,6),(3,7), // connecting edges
    ];
    let mut positions = Vec::with_capacity(12 * 2 * 3);
    for (a, b) in edges.iter() {
        positions.extend_from_slice(&v[*a]);
        positions.extend_from_slice(&v[*b]);
    }
    positions
}

/// Build a model matrix from translation + quaternion rotation + scale
/// Build a 4x4 offset matrix from translation (tx,ty,tz) and Euler angles (roll,pitch,yaw in radians).
fn build_offset_matrix(tx: f32, ty: f32, tz: f32, roll: f32, pitch: f32, yaw: f32) -> Mat4 {
    // Euler ZYX convention
    let cr = roll.cos(); let sr = roll.sin();
    let cp = pitch.cos(); let sp = pitch.sin();
    let cy = yaw.cos(); let sy = yaw.sin();

    let mut m = [0.0f32; 16];
    m[0] = cy * cp;
    m[1] = sy * cp;
    m[2] = -sp;
    m[4] = cy * sp * sr - sy * cr;
    m[5] = sy * sp * sr + cy * cr;
    m[6] = cp * sr;
    m[8] = cy * sp * cr + sy * sr;
    m[9] = sy * sp * cr - cy * sr;
    m[10] = cp * cr;
    m[12] = tx;
    m[13] = ty;
    m[14] = tz;
    m[15] = 1.0;
    Mat4 { data: m }
}

fn build_model_matrix(px: f32, py: f32, pz: f32, qx: f32, qy: f32, qz: f32, qw: f32, sx: f32, sy: f32, sz: f32) -> Mat4 {
    // Quaternion to rotation matrix
    let xx = qx * qx; let yy = qy * qy; let zz = qz * qz;
    let xy = qx * qy; let xz = qx * qz; let yz = qy * qz;
    let wx = qw * qx; let wy = qw * qy; let wz = qw * qz;

    let mut m = [0.0f32; 16];
    // Column 0
    m[0] = (1.0 - 2.0 * (yy + zz)) * sx;
    m[1] = (2.0 * (xy + wz)) * sx;
    m[2] = (2.0 * (xz - wy)) * sx;
    // Column 1
    m[4] = (2.0 * (xy - wz)) * sy;
    m[5] = (1.0 - 2.0 * (xx + zz)) * sy;
    m[6] = (2.0 * (yz + wx)) * sy;
    // Column 2
    m[8] = (2.0 * (xz + wy)) * sz;
    m[9] = (2.0 * (yz - wx)) * sz;
    m[10] = (1.0 - 2.0 * (xx + yy)) * sz;
    // Column 3 (translation)
    m[12] = px;
    m[13] = py;
    m[14] = pz;
    m[15] = 1.0;
    Mat4 { data: m }
}

// ============ Scene State ============

/// A single grid layer with its own GPU buffers.
struct SceneStateGrid {
    vao: WebGlVertexArrayObject,
    pos_buffer: WebGlBuffer,
    col_buffer: WebGlBuffer,
    vertex_count: i32,
    size: f64,
    divisions: u32,
    color: String,
    frame_id: String,
    visible: bool,
}

struct SceneState {
    gl: GL,
    grid_program: WebGlProgram,
    grids: Vec<SceneStateGrid>,
    point_cloud_program: WebGlProgram,
    point_cloud_vao: WebGlVertexArrayObject,
    point_cloud_vertex_count: i32,
    point_cloud_buffer_pos: WebGlBuffer,
    point_cloud_buffer_color: WebGlBuffer,
    line_program: WebGlProgram,
    axes_vao: WebGlVertexArrayObject,
    // Uniform-color shader for scene entities
    uc_program: WebGlProgram,
    cube_vao: WebGlVertexArrayObject,
    cube_vertex_count: i32,
    // Dynamic line buffer for scene lines
    scene_line_vao: WebGlVertexArrayObject,
    scene_line_buffer: WebGlBuffer,
    display_frame: String,
    follow_mode: String,
    camera: OrbitCamera,
    canvas_width: u32,
    canvas_height: u32,
    // Config-driven rendering state
    bg_color: [f32; 3],
    // Transforms visual config
    tf_axis_scale: f32,
    tf_line_width: f32,
    tf_line_color: [f32; 3],
    // Manual TF offsets (frame_name -> [tx,ty,tz, rx,ry,rz])
    tf_offsets: std::collections::HashMap<String, [f64; 6]>,
    // Current player time for TF lookups
    current_time_ns: u64,
    // Render stats
    frame_count: u32,
    last_fps_time: f64,
    fps: f32,
    point_count: i32,
    line_count: i32,
    enable_stats: bool,
}

impl SceneState {
    fn new(gl: GL, width: u32, height: u32) -> Result<Self, String> {
        // Compile grid shaders
        let grid_vs = compile_shader(&gl, GL::VERTEX_SHADER, GRID_VERTEX_SHADER)?;
        let grid_fs = compile_shader(&gl, GL::FRAGMENT_SHADER, GRID_FRAGMENT_SHADER)?;
        let grid_program = link_program(&gl, &grid_vs, &grid_fs)?;

        // Compile point cloud shaders
        let pc_vs = compile_shader(&gl, GL::VERTEX_SHADER, POINT_CLOUD_VERTEX_SHADER)?;
        let pc_fs = compile_shader(&gl, GL::FRAGMENT_SHADER, POINT_CLOUD_FRAGMENT_SHADER)?;
        let point_cloud_program = link_program(&gl, &pc_vs, &pc_fs)?;

        // Compile line shaders (TF axes)
        let line_vs = compile_shader(&gl, GL::VERTEX_SHADER, LINE_VERT_SHADER)?;
        let line_fs = compile_shader(&gl, GL::FRAGMENT_SHADER, LINE_FRAG_SHADER)?;
        let line_program = link_program(&gl, &line_vs, &line_fs)?;

        // Create one default grid
        let default_grid = Self::create_grid_buffers(&gl, 10.0, 10, "#248eff33", "Global")?;

        // Create point cloud VAO (initially empty)
        let point_cloud_vao = gl.create_vertex_array().ok_or("Failed to create VAO")?;
        gl.bind_vertex_array(Some(&point_cloud_vao));

        let point_cloud_buffer_pos = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&point_cloud_buffer_pos));
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        let point_cloud_buffer_color = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&point_cloud_buffer_color));
        gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        gl.bind_vertex_array(None);

        // Create axes VAO for TF visualization (X=Red, Y=Green, Z=Blue, 1m length)
        let axes_data: [f32; 36] = [
            // position (3)      color (3)
            0.0, 0.0, 0.0,      1.0, 0.2, 0.2, // X start
            1.0, 0.0, 0.0,      1.0, 0.2, 0.2, // X end
            0.0, 0.0, 0.0,      0.2, 1.0, 0.2, // Y start
            0.0, 1.0, 0.0,      0.2, 1.0, 0.2, // Y end
            0.0, 0.0, 0.0,      0.2, 0.5, 1.0, // Z start
            0.0, 0.0, 1.0,      0.2, 0.5, 1.0, // Z end
        ];
        let axes_vao = gl.create_vertex_array().ok_or("Failed to create axes VAO")?;
        gl.bind_vertex_array(Some(&axes_vao));
        let axes_vbo = gl.create_buffer().ok_or("Failed to create axes VBO")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&axes_vbo));
        unsafe {
            let array = js_sys::Float32Array::view(&axes_data);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
        }
        let stride = 6 * 4; // 6 floats * 4 bytes
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_with_i32(1, 3, GL::FLOAT, false, stride, 3 * 4);
        gl.bind_vertex_array(None);

        // Compile uniform-color shader (for cubes and scene lines)
        let uc_vs = compile_shader(&gl, GL::VERTEX_SHADER, UNIFORM_COLOR_VERT)?;
        let uc_fs = compile_shader(&gl, GL::FRAGMENT_SHADER, UNIFORM_COLOR_FRAG)?;
        let uc_program = link_program(&gl, &uc_vs, &uc_fs)?;

        // Create cube wireframe VAO (static unit cube)
        let cube_positions = generate_unit_cube_wireframe();
        let cube_vertex_count = (cube_positions.len() / 3) as i32;
        let cube_vao = gl.create_vertex_array().ok_or("Failed to create cube VAO")?;
        gl.bind_vertex_array(Some(&cube_vao));
        let cube_vbo = gl.create_buffer().ok_or("Failed to create cube VBO")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&cube_vbo));
        unsafe {
            let array = js_sys::Float32Array::view(&cube_positions);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
        }
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.bind_vertex_array(None);

        // Create dynamic line VAO for scene lines
        let scene_line_vao = gl.create_vertex_array().ok_or("Failed to create scene line VAO")?;
        gl.bind_vertex_array(Some(&scene_line_vao));
        let scene_line_buffer = gl.create_buffer().ok_or("Failed to create scene line buffer")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&scene_line_buffer));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.bind_vertex_array(None);

        Ok(Self {
            gl,
            grid_program,
            grids: vec![default_grid],
            point_cloud_program,
            point_cloud_vao,
            point_cloud_vertex_count: 0,
            point_cloud_buffer_pos,
            point_cloud_buffer_color,
            line_program,
            axes_vao,
            uc_program,
            cube_vao,
            cube_vertex_count,
            scene_line_vao,
            scene_line_buffer,
            display_frame: String::new(),
            follow_mode: "pose".into(),
            camera: OrbitCamera::new(),
            canvas_width: width,
            canvas_height: height,
            bg_color: [0.12, 0.12, 0.14],
            tf_axis_scale: 1.0,
            tf_line_width: 2.0,
            tf_line_color: [1.0, 1.0, 0.0],
            tf_offsets: std::collections::HashMap::new(),
            current_time_ns: 0,
            frame_count: 0,
            last_fps_time: 0.0,
            fps: 0.0,
            point_count: 0,
            line_count: 0,
            enable_stats: false,
        })
    }

    /// Create GPU buffers for a single grid layer.
    fn create_grid_buffers(gl: &GL, size: f64, divisions: u32, color: &str, frame_id: &str) -> Result<SceneStateGrid, String> {
        let half_size = size as i32;
        let spacing = size as f32 / divisions.max(1) as f32;
        let grid_color = parse_hex_color(color);
        let (grid_pos, grid_col) = generate_grid_and_axes(half_size, spacing, grid_color);
        let vertex_count = (grid_pos.len() / 3) as i32;

        let vao = gl.create_vertex_array().ok_or("Failed to create grid VAO")?;
        gl.bind_vertex_array(Some(&vao));

        let pos_buffer = gl.create_buffer().ok_or("Failed to create grid pos buffer")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&pos_buffer));
        unsafe {
            let array = js_sys::Float32Array::view(&grid_pos);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::DYNAMIC_DRAW);
        }
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        let col_buffer = gl.create_buffer().ok_or("Failed to create grid col buffer")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&col_buffer));
        unsafe {
            let array = js_sys::Float32Array::view(&grid_col);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::DYNAMIC_DRAW);
        }
        gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        gl.bind_vertex_array(None);

        Ok(SceneStateGrid {
            vao,
            pos_buffer,
            col_buffer,
            vertex_count,
            size,
            divisions,
            color: color.to_string(),
            frame_id: frame_id.to_string(),
            visible: true,
        })
    }

    /// Synchronize the grids vector with config. Recreate buffers as needed.
    fn sync_grids(&mut self, config_grids: &[lichtblick_panels::three_dee::GridLayer]) {
        let gl = &self.gl;
        // Resize grids vector to match config
        while self.grids.len() > config_grids.len() {
            let g = self.grids.pop().unwrap();
            gl.delete_vertex_array(Some(&g.vao));
            gl.delete_buffer(Some(&g.pos_buffer));
            gl.delete_buffer(Some(&g.col_buffer));
        }
        for (i, cfg_grid) in config_grids.iter().enumerate() {
            if i < self.grids.len() {
                // Update existing grid
                let g = &mut self.grids[i];
                g.visible = cfg_grid.visible;
                g.frame_id = cfg_grid.frame_id.clone();
                // Rebuild geometry if size/divisions/color changed
                if g.size != cfg_grid.size || g.divisions != cfg_grid.divisions || g.color != cfg_grid.color {
                    g.size = cfg_grid.size;
                    g.divisions = cfg_grid.divisions;
                    g.color = cfg_grid.color.clone();
                    let half_size = cfg_grid.size as i32;
                    let spacing = cfg_grid.size as f32 / cfg_grid.divisions.max(1) as f32;
                    let grid_color = parse_hex_color(&cfg_grid.color);
                    let (grid_pos, grid_col) = generate_grid_and_axes(half_size, spacing, grid_color);
                    g.vertex_count = (grid_pos.len() / 3) as i32;

                    gl.bind_vertex_array(Some(&g.vao));
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&g.pos_buffer));
                    unsafe {
                        let array = js_sys::Float32Array::view(&grid_pos);
                        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::DYNAMIC_DRAW);
                    }
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&g.col_buffer));
                    unsafe {
                        let array = js_sys::Float32Array::view(&grid_col);
                        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::DYNAMIC_DRAW);
                    }
                    gl.bind_vertex_array(None);
                }
            } else {
                // Create new grid
                if let Ok(new_grid) = Self::create_grid_buffers(gl, cfg_grid.size, cfg_grid.divisions, &cfg_grid.color, &cfg_grid.frame_id) {
                    let mut ng = new_grid;
                    ng.visible = cfg_grid.visible;
                    self.grids.push(ng);
                }
            }
        }
    }

    /// Regenerate grid geometry and re-upload to GPU buffers (legacy single grid).
    fn update_grid(&mut self, size: f64, divisions: u32) {
        if let Some(g) = self.grids.first_mut() {
            let half_size = size as i32;
            let spacing = size as f32 / divisions.max(1) as f32;
            let grid_color = parse_hex_color(&g.color);
            let (grid_pos, grid_col) = generate_grid_and_axes(half_size, spacing, grid_color);
            g.vertex_count = (grid_pos.len() / 3) as i32;
            g.size = size;
            g.divisions = divisions;

            let gl = &self.gl;
            gl.bind_vertex_array(Some(&g.vao));

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&g.pos_buffer));
            unsafe {
                let array = js_sys::Float32Array::view(&grid_pos);
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::DYNAMIC_DRAW);
            }

            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&g.col_buffer));
            unsafe {
                let array = js_sys::Float32Array::view(&grid_col);
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::DYNAMIC_DRAW);
            }

            gl.bind_vertex_array(None);
        }
    }

    fn render(&mut self) {
        let gl = &self.gl;

        // FPS tracking
        let now = js_sys::Date::now(); // ms
        self.frame_count += 1;
        if now - self.last_fps_time >= 1000.0 {
            self.fps = self.frame_count as f32 / ((now - self.last_fps_time) as f32 / 1000.0);
            self.frame_count = 0;
            self.last_fps_time = now;
        }

        gl.viewport(0, 0, self.canvas_width as i32, self.canvas_height as i32);
        gl.clear_color(self.bg_color[0], self.bg_color[1], self.bg_color[2], 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.enable(GL::DEPTH_TEST);
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        let aspect = self.canvas_width as f32 / self.canvas_height.max(1) as f32;

        // === Follow Mode: adjust camera based on display frame ===
        let current_time_ns = self.current_time_ns;
        let follow_mode = self.follow_mode.clone();
        let display_frame = self.display_frame.clone();

        if !display_frame.is_empty() && follow_mode != "fixed" {
            TF_STATE.with(|tree| {
                let tf = tree.borrow();
                // Look up display frame relative to root (get its world pose)
                if let Some(root_frames) = tf.frames().first().cloned() {
                    if let Some(transform) = tf.lookup(&root_frames, &display_frame, current_time_ns) {
                        let tx = transform.translation.x as f32;
                        let ty = transform.translation.y as f32;
                        let tz = transform.translation.z as f32;

                        if follow_mode == "pose" {
                            // Follow both position and orientation
                            self.camera.target = Vec3::new(tx, ty, tz);
                            // Extract yaw from quaternion and adjust azimuth
                            let q = &transform.rotation;
                            let yaw = (2.0 * (q.w * q.z + q.x * q.y))
                                .atan2(1.0 - 2.0 * (q.y * q.y + q.z * q.z)) as f32;
                            self.camera.azimuth = std::f32::consts::FRAC_PI_4 - yaw;
                        } else if follow_mode == "position" {
                            // Follow position only, keep user orientation
                            self.camera.target = Vec3::new(tx, ty, tz);
                        }
                    }
                }
            });
        }

        let view = self.camera.view_matrix();
        let proj = self.camera.projection_matrix(aspect);
        let vp = proj.multiply(&view);

        // Draw grids (each in its own frame)
        let mut total_lines = 0i32;
        gl.use_program(Some(&self.grid_program));
        let grid_vp_loc = gl.get_uniform_location(&self.grid_program, "u_viewProjection");
        let grid_model_loc = gl.get_uniform_location(&self.grid_program, "u_modelMatrix");
        gl.uniform_matrix4fv_with_f32_array(grid_vp_loc.as_ref(), false, &vp.data);

        for grid in &self.grids {
            if !grid.visible {
                continue;
            }
            // Compute model matrix: TF lookup for grid's frame_id
            let model_matrix = if !display_frame.is_empty() && !grid.frame_id.is_empty() && grid.frame_id != display_frame {
                TF_STATE.with(|tree| {
                    tree.borrow().lookup(&display_frame, &grid.frame_id, current_time_ns)
                        .map(|t| t.to_mat4_f32())
                }).unwrap_or_else(|| Mat4::identity().data)
            } else {
                Mat4::identity().data
            };
            gl.uniform_matrix4fv_with_f32_array(grid_model_loc.as_ref(), false, &model_matrix);

            gl.bind_vertex_array(Some(&grid.vao));
            gl.draw_arrays(GL::LINES, 0, grid.vertex_count);
            total_lines += grid.vertex_count / 2;
        }

        // Draw point cloud if any
        if self.point_cloud_vertex_count > 0 {
            gl.use_program(Some(&self.point_cloud_program));
            let vp_loc = gl.get_uniform_location(&self.point_cloud_program, "u_viewProjection");
            gl.uniform_matrix4fv_with_f32_array(vp_loc.as_ref(), false, &vp.data);

            let ps_loc = gl.get_uniform_location(&self.point_cloud_program, "u_pointSize");
            gl.uniform1f(ps_loc.as_ref(), 3.0);

            gl.bind_vertex_array(Some(&self.point_cloud_vao));
            gl.draw_arrays(GL::POINTS, 0, self.point_cloud_vertex_count);
        }

        // Draw TF axes for each frame in the tree
        if !self.display_frame.is_empty() {
            gl.use_program(Some(&self.line_program));
            let vp_loc = gl.get_uniform_location(&self.line_program, "u_viewProjection");
            let model_loc = gl.get_uniform_location(&self.line_program, "u_modelMatrix");
            gl.uniform_matrix4fv_with_f32_array(vp_loc.as_ref(), false, &vp.data);
            gl.bind_vertex_array(Some(&self.axes_vao));
            gl.line_width(self.tf_line_width);

            let display_frame = self.display_frame.clone();
            let axis_scale = self.tf_axis_scale;
            let offsets = &self.tf_offsets;
            let time_ns = self.current_time_ns;
            TF_STATE.with(|tree| {
                let tf = tree.borrow();
                for frame in tf.frames() {
                    if let Some(transform) = tf.lookup(&display_frame, &frame, time_ns) {
                        let mut matrix = transform.to_mat4_f32();

                        // Apply manual offset if configured
                        if let Some(off) = offsets.get(&frame) {
                            let offset_mat = build_offset_matrix(
                                off[0] as f32, off[1] as f32, off[2] as f32,
                                off[3] as f32, off[4] as f32, off[5] as f32,
                            );
                            let base = Mat4 { data: matrix };
                            let combined = base.multiply(&offset_mat);
                            matrix = combined.data;
                        }

                        // Apply axis scale
                        if axis_scale != 1.0 {
                            let base = Mat4 { data: matrix };
                            let scale_mat = build_model_matrix(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, axis_scale, axis_scale, axis_scale);
                            let combined = base.multiply(&scale_mat);
                            matrix = combined.data;
                        }

                        gl.uniform_matrix4fv_with_f32_array(model_loc.as_ref(), false, &matrix);
                        gl.draw_arrays(GL::LINES, 0, 6);
                    }
                }
            });
        }

        // Draw scene entities (cubes and lines from SceneUpdate)
        let display_frame = self.display_frame.clone();
        let time_ns = self.current_time_ns;
        SCENE_ENTITIES.with(|entities| {
            let ent = entities.borrow();
            let (cubes, lines) = &*ent;

            if cubes.is_empty() && lines.is_empty() {
                return;
            }

            gl.use_program(Some(&self.uc_program));
            let vp_loc = gl.get_uniform_location(&self.uc_program, "u_viewProjection");
            let model_loc = gl.get_uniform_location(&self.uc_program, "u_modelMatrix");
            let color_loc = gl.get_uniform_location(&self.uc_program, "u_color");
            gl.uniform_matrix4fv_with_f32_array(vp_loc.as_ref(), false, &vp.data);

            // Draw cubes
            if !cubes.is_empty() {
                gl.bind_vertex_array(Some(&self.cube_vao));
                gl.line_width(2.0);

                for cube in cubes.iter() {
                    // Get TF from display_frame to entity's frame_id
                    let frame_tf = if !display_frame.is_empty() && !cube.frame_id.is_empty() {
                        TF_STATE.with(|tree| {
                            tree.borrow().lookup(&display_frame, &cube.frame_id, time_ns)
                                .map(|t| t.to_mat4_f32())
                        })
                    } else {
                        None
                    };
                    let frame_mat = frame_tf.map(|d| Mat4 { data: d }).unwrap_or_else(Mat4::identity);

                    // Build cube model matrix: translate + rotate + scale
                    let cube_mat = build_model_matrix(
                        cube.px, cube.py, cube.pz,
                        cube.ox, cube.oy, cube.oz, cube.ow,
                        cube.sx, cube.sy, cube.sz,
                    );
                    let model = frame_mat.multiply(&cube_mat);
                    gl.uniform_matrix4fv_with_f32_array(model_loc.as_ref(), false, &model.data);
                    gl.uniform4f(color_loc.as_ref(), cube.r, cube.g, cube.b, cube.a);
                    gl.draw_arrays(GL::LINES, 0, self.cube_vertex_count);
                }
            }

            // Draw lines
            if !lines.is_empty() {
                gl.bind_vertex_array(Some(&self.scene_line_vao));
                gl.line_width(2.0);

                for line in lines.iter() {
                    if line.points.is_empty() {
                        continue;
                    }
                    let vertex_count = (line.points.len() / 3) as i32;

                    // Get TF from display_frame to entity's frame_id
                    let frame_tf = if !display_frame.is_empty() && !line.frame_id.is_empty() {
                        TF_STATE.with(|tree| {
                            tree.borrow().lookup(&display_frame, &line.frame_id, time_ns)
                                .map(|t| t.to_mat4_f32())
                        })
                    } else {
                        None
                    };
                    let frame_mat = frame_tf.map(|d| Mat4 { data: d }).unwrap_or_else(Mat4::identity);

                    let line_mat = build_model_matrix(
                        line.px, line.py, line.pz,
                        line.ox, line.oy, line.oz, line.ow,
                        1.0, 1.0, 1.0,
                    );
                    let model = frame_mat.multiply(&line_mat);
                    gl.uniform_matrix4fv_with_f32_array(model_loc.as_ref(), false, &model.data);
                    gl.uniform4f(color_loc.as_ref(), line.r, line.g, line.b, line.a);

                    // Upload points to dynamic buffer
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.scene_line_buffer));
                    unsafe {
                        let array = js_sys::Float32Array::view(&line.points);
                        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::DYNAMIC_DRAW);
                    }

                    let draw_mode = match line.line_type {
                        1 => GL::LINE_LOOP,
                        2 => GL::LINES,
                        _ => GL::LINE_STRIP,
                    };
                    gl.draw_arrays(draw_mode, 0, vertex_count);
                    total_lines += vertex_count / 2;
                }
            }
        });

        self.line_count = total_lines;

        gl.bind_vertex_array(None);
    }

    /// Upload point cloud data (positions: [x,y,z,...], colors: [r,g,b,a,...])
    fn update_point_cloud(&mut self, positions: &[f32], colors: &[f32]) {
        let gl = &self.gl;
        self.point_cloud_vertex_count = (positions.len() / 3) as i32;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.point_cloud_buffer_pos));
        unsafe {
            let array = js_sys::Float32Array::view(positions);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::DYNAMIC_DRAW);
        }

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.point_cloud_buffer_color));
        unsafe {
            let array = js_sys::Float32Array::view(colors);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::DYNAMIC_DRAW);
        }
    }

    fn set_display_frame(&mut self, frame: String) {
        self.display_frame = frame;
    }

    /// Raycast from screen coordinates to the Y=0 ground plane.
    /// Returns (x, z) world coordinates of the intersection point.
    fn raycast_to_ground(&self, screen_x: f32, screen_y: f32) -> Option<(f32, f32, f32)> {
        let aspect = self.canvas_width as f32 / self.canvas_height.max(1) as f32;
        let view = self.camera.view_matrix();
        let proj = self.camera.projection_matrix(aspect);
        let vp = proj.multiply(&view);

        // Invert the VP matrix (simplified - use manual 4x4 inverse)
        let inv_vp = invert_mat4(&vp)?;

        // Convert screen coords to NDC (-1..1)
        let ndc_x = (2.0 * screen_x / self.canvas_width as f32) - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_y / self.canvas_height as f32);

        // Near and far points in NDC
        let near_ndc = [ndc_x, ndc_y, -1.0, 1.0];
        let far_ndc = [ndc_x, ndc_y, 1.0, 1.0];

        // Transform to world space
        let near_world = mat4_mul_vec4(&inv_vp, &near_ndc);
        let far_world = mat4_mul_vec4(&inv_vp, &far_ndc);

        if near_world[3].abs() < 1e-10 || far_world[3].abs() < 1e-10 {
            return None;
        }

        let origin = Vec3::new(
            near_world[0] / near_world[3],
            near_world[1] / near_world[3],
            near_world[2] / near_world[3],
        );
        let far_pt = Vec3::new(
            far_world[0] / far_world[3],
            far_world[1] / far_world[3],
            far_world[2] / far_world[3],
        );

        let dir = far_pt.sub(&origin);

        // Intersect with Y=0 plane
        if dir.y.abs() < 1e-10 {
            return None; // Ray parallel to ground
        }

        let t = -origin.y / dir.y;
        if t < 0.0 {
            return None; // Behind camera
        }

        let x = origin.x + dir.x * t;
        let y = 0.0;
        let z = origin.z + dir.z * t;

        Some((x, y, z))
    }
}

/// Invert a 4x4 matrix. Returns None if singular.
fn invert_mat4(m: &Mat4) -> Option<Mat4> {
    let d = &m.data;
    let mut inv = [0.0f32; 16];

    inv[0] = d[5]*d[10]*d[15] - d[5]*d[11]*d[14] - d[9]*d[6]*d[15] + d[9]*d[7]*d[14] + d[13]*d[6]*d[11] - d[13]*d[7]*d[10];
    inv[4] = -d[4]*d[10]*d[15] + d[4]*d[11]*d[14] + d[8]*d[6]*d[15] - d[8]*d[7]*d[14] - d[12]*d[6]*d[11] + d[12]*d[7]*d[10];
    inv[8] = d[4]*d[9]*d[15] - d[4]*d[11]*d[13] - d[8]*d[5]*d[15] + d[8]*d[7]*d[13] + d[12]*d[5]*d[11] - d[12]*d[7]*d[9];
    inv[12] = -d[4]*d[9]*d[14] + d[4]*d[10]*d[13] + d[8]*d[5]*d[14] - d[8]*d[6]*d[13] - d[12]*d[5]*d[10] + d[12]*d[6]*d[9];
    inv[1] = -d[1]*d[10]*d[15] + d[1]*d[11]*d[14] + d[9]*d[2]*d[15] - d[9]*d[3]*d[14] - d[13]*d[2]*d[11] + d[13]*d[3]*d[10];
    inv[5] = d[0]*d[10]*d[15] - d[0]*d[11]*d[14] - d[8]*d[2]*d[15] + d[8]*d[3]*d[14] + d[12]*d[2]*d[11] - d[12]*d[3]*d[10];
    inv[9] = -d[0]*d[9]*d[15] + d[0]*d[11]*d[13] + d[8]*d[1]*d[15] - d[8]*d[3]*d[13] - d[12]*d[1]*d[11] + d[12]*d[3]*d[9];
    inv[13] = d[0]*d[9]*d[14] - d[0]*d[10]*d[13] - d[8]*d[1]*d[14] + d[8]*d[2]*d[13] + d[12]*d[1]*d[10] - d[12]*d[2]*d[9];
    inv[2] = d[1]*d[6]*d[15] - d[1]*d[7]*d[14] - d[5]*d[2]*d[15] + d[5]*d[3]*d[14] + d[13]*d[2]*d[7] - d[13]*d[3]*d[6];
    inv[6] = -d[0]*d[6]*d[15] + d[0]*d[7]*d[14] + d[4]*d[2]*d[15] - d[4]*d[3]*d[14] - d[12]*d[2]*d[7] + d[12]*d[3]*d[6];
    inv[10] = d[0]*d[5]*d[15] - d[0]*d[7]*d[13] - d[4]*d[1]*d[15] + d[4]*d[3]*d[13] + d[12]*d[1]*d[7] - d[12]*d[3]*d[5];
    inv[14] = -d[0]*d[5]*d[14] + d[0]*d[6]*d[13] + d[4]*d[1]*d[14] - d[4]*d[2]*d[13] - d[12]*d[1]*d[6] + d[12]*d[2]*d[5];
    inv[3] = -d[1]*d[6]*d[11] + d[1]*d[7]*d[10] + d[5]*d[2]*d[11] - d[5]*d[3]*d[10] - d[9]*d[2]*d[7] + d[9]*d[3]*d[6];
    inv[7] = d[0]*d[6]*d[11] - d[0]*d[7]*d[10] - d[4]*d[2]*d[11] + d[4]*d[3]*d[10] + d[8]*d[2]*d[7] - d[8]*d[3]*d[6];
    inv[11] = -d[0]*d[5]*d[11] + d[0]*d[7]*d[9] + d[4]*d[1]*d[11] - d[4]*d[3]*d[9] - d[8]*d[1]*d[7] + d[8]*d[3]*d[5];
    inv[15] = d[0]*d[5]*d[10] - d[0]*d[6]*d[9] - d[4]*d[1]*d[10] + d[4]*d[2]*d[9] + d[8]*d[1]*d[6] - d[8]*d[2]*d[5];

    let det = d[0]*inv[0] + d[1]*inv[4] + d[2]*inv[8] + d[3]*inv[12];
    if det.abs() < 1e-10 {
        return None;
    }
    let inv_det = 1.0 / det;
    for val in inv.iter_mut() {
        *val *= inv_det;
    }
    Some(Mat4 { data: inv })
}

/// Multiply a 4x4 matrix by a 4-element vector.
fn mat4_mul_vec4(m: &Mat4, v: &[f32; 4]) -> [f32; 4] {
    let d = &m.data;
    [
        d[0]*v[0] + d[4]*v[1] + d[8]*v[2] + d[12]*v[3],
        d[1]*v[0] + d[5]*v[1] + d[9]*v[2] + d[13]*v[3],
        d[2]*v[0] + d[6]*v[1] + d[10]*v[2] + d[14]*v[3],
        d[3]*v[0] + d[7]*v[1] + d[11]*v[2] + d[15]*v[3],
    ]
}

// ============ Public TF frame metadata helper ============

/// Format a nanosecond duration into a human-readable short string.
pub fn format_short_duration(duration_ns: u64) -> String {
    if duration_ns < 1_000 {
        format!("{} ns", duration_ns)
    } else if duration_ns < 1_000_000 {
        format!("{:.1} µs", duration_ns as f64 / 1_000.0)
    } else if duration_ns < 1_000_000_000 {
        format!("{:.1} ms", duration_ns as f64 / 1_000_000.0)
    } else {
        format!("{:.1} s", duration_ns as f64 / 1_000_000_000.0)
    }
}

/// Get metadata for a TF frame: (parent_name, history_size, age_string).
pub fn get_tf_frame_metadata(frame_name: &str, current_time_ns: u64) -> Option<(String, usize, String)> {
    TF_STATE.with(|tf| {
        let tree = tf.borrow();
        let parent = tree.get_parent(frame_name)?;
        let history_size = tree.get_history_size(&parent, frame_name);
        let age_str = if let Some(latest_ns) = tree.get_latest_timestamp(&parent, frame_name) {
            if current_time_ns > latest_ns {
                format_short_duration(current_time_ns - latest_ns)
            } else {
                "0 ns".to_string()
            }
        } else {
            "unknown".to_string()
        };
        Some((parent, history_size, age_str))
    })
}

// ============ PointCloud2 Decoder ============

/// Decode a ROS PointCloud2 message (CDR encoded) into positions and colors.
pub fn decode_point_cloud2(data: &[u8], encoding: &str) -> Option<(Vec<f32>, Vec<f32>)> {
    if data.len() < 32 {
        return None;
    }

    // Skip CDR encapsulation header for CDR encoding
    let offset = if encoding == "cdr" || encoding == "CDR" {
        4
    } else {
        0
    };

    if data.len() < offset + 4 {
        return None;
    }

    let d = &data[offset..];

    // PointCloud2 layout (ROS2 CDR):
    // Header (stamp + frame_id string)
    // height: u32
    // width: u32
    // fields: sequence of PointField
    // is_bigendian: bool
    // point_step: u32
    // row_step: u32
    // data: sequence of bytes
    // is_dense: bool

    // For simplicity, use a cursor-based reader
    let mut pos = 0;

    // Skip header: sec(4) + nanosec(4)
    if d.len() < pos + 8 {
        return None;
    }
    pos += 8;

    // frame_id string: length(4) + chars + null + alignment
    if d.len() < pos + 4 {
        return None;
    }
    let frame_len = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
    pos += 4 + frame_len;
    // Align to 4
    pos = (pos + 3) & !3;

    // height, width
    if d.len() < pos + 8 {
        return None;
    }
    let height = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]);
    pos += 4;
    let width = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]);
    pos += 4;

    // Fields sequence: count(4) + fields
    if d.len() < pos + 4 {
        return None;
    }
    let num_fields = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
    pos += 4;

    #[derive(Clone, Debug)]
    struct PointField {
        name: String,
        offset: u32,
        datatype: u8,
        count: u32,
    }

    let mut fields = Vec::new();
    for _ in 0..num_fields {
        // name string
        if d.len() < pos + 4 {
            return None;
        }
        let name_len =
            u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
        pos += 4;
        if d.len() < pos + name_len {
            return None;
        }
        let name = String::from_utf8_lossy(&d[pos..pos + name_len.saturating_sub(1)]).to_string();
        pos += name_len;
        // Align to 4
        pos = (pos + 3) & !3;

        // offset: u32
        if d.len() < pos + 4 {
            return None;
        }
        let field_offset = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]);
        pos += 4;

        // datatype: u8
        if d.len() < pos + 1 {
            return None;
        }
        let datatype = d[pos];
        pos += 1;
        // Align to 4
        pos = (pos + 3) & !3;

        // count: u32
        if d.len() < pos + 4 {
            return None;
        }
        let count = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]);
        pos += 4;

        fields.push(PointField {
            name,
            offset: field_offset,
            datatype,
            count,
        });
    }

    // is_bigendian: u8
    if d.len() < pos + 1 {
        return None;
    }
    pos += 1;
    // Align to 4
    pos = (pos + 3) & !3;

    // point_step: u32
    if d.len() < pos + 4 {
        return None;
    }
    let point_step = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
    pos += 4;

    // row_step: u32
    if d.len() < pos + 4 {
        return None;
    }
    pos += 4; // skip row_step

    // data sequence: length(4) + bytes
    if d.len() < pos + 4 {
        return None;
    }
    let data_len = u32::from_le_bytes([d[pos], d[pos + 1], d[pos + 2], d[pos + 3]]) as usize;
    pos += 4;

    if d.len() < pos + data_len {
        return None;
    }
    let cloud_data = &d[pos..pos + data_len];

    // Find field offsets
    let x_field = fields.iter().find(|f| f.name == "x")?;
    let y_field = fields.iter().find(|f| f.name == "y")?;
    let z_field = fields.iter().find(|f| f.name == "z")?;
    let intensity_field = fields.iter().find(|f| f.name == "intensity");
    let r_field = fields.iter().find(|f| f.name == "red" || f.name == "r");
    let g_field = fields.iter().find(|f| f.name == "green" || f.name == "g");
    let b_field = fields.iter().find(|f| f.name == "blue" || f.name == "b");

    let num_points = (height as usize) * (width as usize);
    let mut positions_out = Vec::with_capacity(num_points * 3);
    let mut colors_out = Vec::with_capacity(num_points * 4);

    for i in 0..num_points {
        let base = i * point_step;
        if base + point_step > cloud_data.len() {
            break;
        }

        // Read XYZ (assuming FLOAT32 = datatype 7)
        let x = read_f32_le(cloud_data, base + x_field.offset as usize);
        let y = read_f32_le(cloud_data, base + y_field.offset as usize);
        let z = read_f32_le(cloud_data, base + z_field.offset as usize);

        // Skip NaN points
        if x.is_nan() || y.is_nan() || z.is_nan() {
            continue;
        }

        positions_out.push(x);
        positions_out.push(z); // Swap Y/Z for ROS→GL convention (Y up in GL, Z up in ROS)
        positions_out.push(-y);

        // Colorize
        if let (Some(rf), Some(gf), Some(bf)) = (r_field, g_field, b_field) {
            let r = read_color_field(cloud_data, base + rf.offset as usize, rf.datatype);
            let g = read_color_field(cloud_data, base + gf.offset as usize, gf.datatype);
            let b = read_color_field(cloud_data, base + bf.offset as usize, bf.datatype);
            colors_out.extend_from_slice(&[r, g, b, 1.0]);
        } else if let Some(int_f) = intensity_field {
            // Color by intensity (map to rainbow)
            let intensity = read_f32_le(cloud_data, base + int_f.offset as usize);
            let (r, g, b) = intensity_to_color(intensity);
            colors_out.extend_from_slice(&[r, g, b, 1.0]);
        } else {
            // Color by elevation (Z in ROS = Y in our GL scene)
            let elev = z;
            let (r, g, b) = elevation_to_color(elev);
            colors_out.extend_from_slice(&[r, g, b, 1.0]);
        }
    }

    if positions_out.is_empty() {
        return None;
    }

    Some((positions_out, colors_out))
}

fn read_f32_le(data: &[u8], offset: usize) -> f32 {
    if offset + 4 > data.len() {
        return f32::NAN;
    }
    f32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

fn read_color_field(data: &[u8], offset: usize, datatype: u8) -> f32 {
    match datatype {
        1 => data.get(offset).copied().unwrap_or(0) as f32 / 255.0, // UINT8
        7 => read_f32_le(data, offset),                               // FLOAT32
        _ => 0.5,
    }
}

/// Map intensity (0-255 typical) to a turbo-like colormap
fn intensity_to_color(intensity: f32) -> (f32, f32, f32) {
    let t = (intensity / 255.0).clamp(0.0, 1.0);
    // Simplified turbo colormap approximation
    let r = (1.0 - (t - 0.75).abs() * 4.0).clamp(0.0, 1.0);
    let g = (1.0 - (t - 0.5).abs() * 4.0).clamp(0.0, 1.0);
    let b = (1.0 - (t - 0.25).abs() * 4.0).clamp(0.0, 1.0);
    (r.max(0.1), g.max(0.1), b.max(0.1))
}

/// Map elevation (-2..+5 typical) to a gradient
fn elevation_to_color(z: f32) -> (f32, f32, f32) {
    let t = ((z + 2.0) / 7.0).clamp(0.0, 1.0);
    let r = t;
    let g = 1.0 - (t - 0.5).abs() * 2.0;
    let b = 1.0 - t;
    (r.max(0.1), g.max(0.1), b.max(0.1))
}

// ============ Check if schema is PointCloud2 ============

pub fn is_point_cloud_schema(schema: &str) -> bool {
    schema.contains("PointCloud2")
        || schema.contains("pointcloud")
        || schema == "sensor_msgs/msg/PointCloud2"
        || schema == "sensor_msgs/PointCloud2"
}

// ============ Custom DynamicMessage → JsValue converter (snake_case, defaults, longs→f64) ============

/// Convert a prost-reflect DynamicMessage to a JsValue object with:
/// - Original proto field names (snake_case)
/// - All fields emitted including defaults (like protobufjs {defaults: true})
/// - 64-bit integers converted to f64 (like protobufjs {longs: Number})
/// - Enums converted to integer values (like protobufjs default)
fn dynamic_message_to_js(msg: &prost_reflect::DynamicMessage) -> JsValue {
    use prost_reflect::ReflectMessage;
    let obj = js_sys::Object::new();
    let desc = msg.descriptor();
    for field_desc in desc.fields() {
        let name = field_desc.name();
        let value = msg.get_field(&field_desc);
        let js_val = prost_value_to_js(&value, &field_desc);
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(name), &js_val);
    }
    obj.into()
}

/// Convert a prost_reflect::Value to JsValue
fn prost_value_to_js(value: &prost_reflect::Value, field_desc: &prost_reflect::FieldDescriptor) -> JsValue {
    use prost_reflect::Value;
    use prost_reflect::Kind;

    match value {
        Value::Bool(b) => JsValue::from_bool(*b),
        Value::I32(n) => JsValue::from_f64(*n as f64),
        Value::I64(n) => JsValue::from_f64(*n as f64),
        Value::U32(n) => JsValue::from_f64(*n as f64),
        Value::U64(n) => JsValue::from_f64(*n as f64),
        Value::F32(n) => JsValue::from_f64(*n as f64),
        Value::F64(n) => JsValue::from_f64(*n),
        Value::String(s) => JsValue::from_str(s),
        Value::Bytes(b) => {
            let arr = js_sys::Uint8Array::new_with_length(b.len() as u32);
            arr.copy_from(b);
            arr.into()
        }
        Value::EnumNumber(n) => {
            // Return integer value (protobufjs default behavior)
            JsValue::from_f64(*n as f64)
        }
        Value::Message(nested_msg) => {
            use prost_reflect::ReflectMessage;
            // Special handling for google.protobuf.Timestamp: convert seconds/nanos → sec/nsec
            if nested_msg.descriptor().full_name() == "google.protobuf.Timestamp" {
                let obj = js_sys::Object::new();
                let mut sec = 0.0_f64;
                let mut nsec = 0.0_f64;
                for fd in nested_msg.descriptor().fields() {
                    let val = nested_msg.get_field(&fd);
                    if fd.name() == "seconds" {
                        if let Value::I64(s) = val.as_ref() { sec = *s as f64; }
                    }
                    if fd.name() == "nanos" {
                        if let Value::I32(n) = val.as_ref() { nsec = *n as f64; }
                    }
                }
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("sec"), &JsValue::from_f64(sec));
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("nsec"), &JsValue::from_f64(nsec));
                return obj.into();
            }
            dynamic_message_to_js(nested_msg)
        }
        Value::List(list) => {
            let arr = js_sys::Array::new_with_length(list.len() as u32);
            for (i, item) in list.iter().enumerate() {
                // For list items, we use the same field descriptor
                let js_item = prost_value_to_js(item, field_desc);
                arr.set(i as u32, js_item);
            }
            arr.into()
        }
        Value::Map(map) => {
            let obj = js_sys::Object::new();
            for (key, val) in map.iter() {
                let key_str = match key {
                    prost_reflect::MapKey::Bool(b) => b.to_string(),
                    prost_reflect::MapKey::I32(n) => n.to_string(),
                    prost_reflect::MapKey::I64(n) => n.to_string(),
                    prost_reflect::MapKey::U32(n) => n.to_string(),
                    prost_reflect::MapKey::U64(n) => n.to_string(),
                    prost_reflect::MapKey::String(s) => s.clone(),
                };
                let js_val = prost_value_to_js(val, field_desc);
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str(&key_str), &js_val);
            }
            obj.into()
        }
    }
}

// ============ Parse JS FrameTransform object into StampedTransform ============

/// Parse a JS object { parent_frame_id, child_frame_id, tx, ty, tz, rx, ry, rz, rw, timestamp_sec, timestamp_nsec }
/// into a StampedTransform for insertion into the TF tree.
fn parse_js_frame_transform(obj: &JsValue) -> Option<StampedTransform> {
    let get_str = |key: &str| -> String {
        js_sys::Reflect::get(obj, &JsValue::from_str(key))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default()
    };
    let get_f64 = |key: &str| -> f64 {
        js_sys::Reflect::get(obj, &JsValue::from_str(key))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    };

    let parent = get_str("parent_frame_id");
    let child = get_str("child_frame_id");

    if child.is_empty() {
        return None;
    }

    let tx = get_f64("tx");
    let ty = get_f64("ty");
    let tz = get_f64("tz");
    let rx = get_f64("rx");
    let ry = get_f64("ry");
    let rz = get_f64("rz");
    let rw = get_f64("rw");
    let sec = get_f64("timestamp_sec") as u64;
    let nsec = get_f64("timestamp_nsec") as u64;

    let timestamp_ns = sec * 1_000_000_000 + nsec;

    Some(StampedTransform {
        parent_frame: parent,
        child_frame: child,
        timestamp_ns,
        translation: Vec3d { x: tx, y: ty, z: tz },
        rotation: Quaternion { x: rx, y: ry, z: rz, w: rw },
    })
}

/// Parse the JS object returned by js_convert_message_to_scene into (Vec<SceneCube>, Vec<SceneLine>)
fn parse_scene_update_result(result: &JsValue) -> (Vec<SceneCube>, Vec<SceneLine>) {
    let mut cubes = Vec::new();
    let mut lines = Vec::new();

    let get_f32 = |obj: &JsValue, key: &str| -> f32 {
        js_sys::Reflect::get(obj, &JsValue::from_str(key))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as f32
    };
    let get_str = |obj: &JsValue, key: &str| -> String {
        js_sys::Reflect::get(obj, &JsValue::from_str(key))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default()
    };

    // Parse cubes array
    if let Ok(cubes_val) = js_sys::Reflect::get(result, &JsValue::from_str("cubes")) {
        if let Ok(arr) = js_sys::Array::try_from(cubes_val) {
            for i in 0..arr.length() {
                let c = arr.get(i);
                cubes.push(SceneCube {
                    frame_id: get_str(&c, "frame_id"),
                    px: get_f32(&c, "px"), py: get_f32(&c, "py"), pz: get_f32(&c, "pz"),
                    ox: get_f32(&c, "ox"), oy: get_f32(&c, "oy"), oz: get_f32(&c, "oz"),
                    ow: {
                        let w = get_f32(&c, "ow");
                        if w == 0.0 { 1.0 } else { w }
                    },
                    sx: get_f32(&c, "sx"), sy: get_f32(&c, "sy"), sz: get_f32(&c, "sz"),
                    r: get_f32(&c, "r"), g: get_f32(&c, "g"), b: get_f32(&c, "b"), a: get_f32(&c, "a"),
                });
            }
        }
    }

    // Parse lines array
    if let Ok(lines_val) = js_sys::Reflect::get(result, &JsValue::from_str("lines")) {
        if let Ok(arr) = js_sys::Array::try_from(lines_val) {
            for i in 0..arr.length() {
                let l = arr.get(i);
                let frame_id = get_str(&l, "frame_id");
                let line_type = get_f32(&l, "line_type") as u32;
                let px = get_f32(&l, "px");
                let py = get_f32(&l, "py");
                let pz = get_f32(&l, "pz");
                let ox = get_f32(&l, "ox");
                let oy = get_f32(&l, "oy");
                let oz = get_f32(&l, "oz");
                let ow = { let w = get_f32(&l, "ow"); if w == 0.0 { 1.0 } else { w } };
                let r = get_f32(&l, "r");
                let g = get_f32(&l, "g");
                let b = get_f32(&l, "b");
                let a = get_f32(&l, "a");

                // Parse points flat array [x,y,z, x,y,z, ...]
                let mut points = Vec::new();
                if let Ok(pts_val) = js_sys::Reflect::get(&l, &JsValue::from_str("points")) {
                    if let Ok(pts_arr) = js_sys::Array::try_from(pts_val) {
                        for j in 0..pts_arr.length() {
                            if let Some(v) = pts_arr.get(j).as_f64() {
                                points.push(v as f32);
                            }
                        }
                    }
                }

                if !points.is_empty() {
                    lines.push(SceneLine {
                        frame_id, line_type, px, py, pz, ox, oy, oz, ow, r, g, b, a, points,
                    });
                }
            }
        }
    }

    (cubes, lines)
}

// ============ Thread-local Scene Storage ============

thread_local! {
    static SCENE: std::cell::RefCell<Option<SceneState>> = std::cell::RefCell::new(None);
    static TF_STATE: std::cell::RefCell<TfTree> = std::cell::RefCell::new(TfTree::new());
    /// Scene entities (cubes, lines) from SceneUpdate converters.
    static SCENE_ENTITIES: std::cell::RefCell<(Vec<SceneCube>, Vec<SceneLine>)> = std::cell::RefCell::new((Vec::new(), Vec::new()));
    /// Cache of compiled prost-reflect DescriptorPools keyed by schema name.
    static PROTO_POOLS: std::cell::RefCell<std::collections::HashMap<String, prost_reflect::DescriptorPool>> = std::cell::RefCell::new(std::collections::HashMap::new());
    /// Tracks schemas that failed to compile (don't retry).
    static FAILED_SCHEMAS: std::cell::RefCell<std::collections::HashSet<String>> = std::cell::RefCell::new(std::collections::HashSet::new());
}

fn with_scene<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&SceneState) -> R,
{
    SCENE.with(|s| s.borrow().as_ref().map(f))
}

fn with_scene_mut<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut SceneState) -> R,
{
    SCENE.with(|s| s.borrow_mut().as_mut().map(f))
}

fn set_scene(scene: SceneState) {
    SCENE.with(|s| *s.borrow_mut() = Some(scene));
}

fn has_scene() -> bool {
    SCENE.with(|s| s.borrow().is_some())
}

// ============ Leptos Component ============

/// 3D Panel component with WebGL2 rendering.
#[component]
pub fn ThreeDeePanel(node_id: NodeId) -> impl IntoView {
    let state = use_app_state();
    let layout = use_layout_state();
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let point_count = RwSignal::new(0i32);
    let fps_signal = RwSignal::new(0.0f32);
    let line_count_signal = RwSignal::new(0i32);

    // Timestamp cache: skip re-processing messages that haven't changed
    let last_processed_times = RwSignal::new(std::collections::HashMap::<String, u64>::new());

    // Initialize WebGL on mount
    Effect::new(move |_| {
        if has_scene() {
            return;
        }
        let Some(canvas_el) = canvas_ref.get() else {
            return;
        };
        let canvas: HtmlCanvasElement = canvas_el.into();

        // Set canvas size to match parent
        let parent = canvas.parent_element().unwrap();
        let w = parent.client_width().max(200) as u32;
        let h = parent.client_height().max(200) as u32;
        canvas.set_width(w);
        canvas.set_height(h);

        let gl: GL = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        match SceneState::new(gl, w, h) {
            Ok(mut s) => {
                s.render();
                set_scene(s);
            }
            Err(e) => {
                web_sys::console::error_1(&format!("3D init error: {}", e).into());
            }
        }
    });

    // Orbit camera controls (using thread_local Rc for non-Send state)
    let is_rotating = std::rc::Rc::new(std::cell::RefCell::new(false));
    let is_panning = std::rc::Rc::new(std::cell::RefCell::new(false));
    let last_mouse = std::rc::Rc::new(std::cell::RefCell::new((0i32, 0i32)));

    let is_rotating_down = is_rotating.clone();
    let is_panning_down = is_panning.clone();
    let last_mouse_down = last_mouse.clone();
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        *last_mouse_down.borrow_mut() = (ev.client_x(), ev.client_y());

        // Click-to-Publish: Shift + Left Click → raycast to ground
        if ev.button() == 0 && ev.shift_key() {
            // Get canvas-relative position
            if let Some(target) = ev.target() {
                if let Ok(canvas) = target.dyn_into::<web_sys::HtmlCanvasElement>() {
                    let rect = canvas.get_bounding_client_rect();
                    let cx = (ev.client_x() as f64 - rect.left()) as f32;
                    let cy = (ev.client_y() as f64 - rect.top()) as f32;

                    let hit = with_scene(|s| s.raycast_to_ground(cx, cy));
                    if let Some(Some((x, _y, z))) = hit {
                        let cfg = layout.get_three_dee_config(node_id);
                        let topic = &cfg.publish.topic;
                        let msg = format!(
                            "[Click-to-Publish] type={}, topic={}, point=({:.3}, 0.0, {:.3})",
                            cfg.publish.publish_type, topic, x, z
                        );
                        web_sys::console::log_1(&msg.into());
                    }
                }
            }
            return;
        }

        if ev.button() == 0 {
            *is_rotating_down.borrow_mut() = true;
        } else if ev.button() == 1 || ev.button() == 2 {
            *is_panning_down.borrow_mut() = true;
        }
    };

    let is_rotating_move = is_rotating.clone();
    let is_panning_move = is_panning.clone();
    let last_mouse_move = last_mouse.clone();
    let on_mousemove = move |ev: leptos::ev::MouseEvent| {
        let (lx, ly) = *last_mouse_move.borrow();
        let dx = ev.client_x() - lx;
        let dy = ev.client_y() - ly;
        *last_mouse_move.borrow_mut() = (ev.client_x(), ev.client_y());

        if *is_rotating_move.borrow() {
            with_scene_mut(|s| {
                s.camera.rotate(dx as f32, dy as f32);
                s.render();
            });
        } else if *is_panning_move.borrow() {
            with_scene_mut(|s| {
                s.camera.pan(dx as f32, dy as f32);
                s.render();
            });
        }
    };

    let is_rotating_up = is_rotating.clone();
    let is_panning_up = is_panning.clone();
    let on_mouseup = move |_: leptos::ev::MouseEvent| {
        *is_rotating_up.borrow_mut() = false;
        *is_panning_up.borrow_mut() = false;
    };

    let on_wheel = move |ev: leptos::ev::WheelEvent| {
        ev.prevent_default();
        with_scene_mut(|s| {
            s.camera.zoom(ev.delta_y() as f32);
            s.render();
        });
    };

    // Sync display_frame from sidebar selection to scene
    Effect::new(move |_| {
        let selected = state.display_frame.get();
        with_scene_mut(|s| {
            s.set_display_frame(selected);
            s.render();
        });
    });

    // Update point cloud and TF tree from player data on frame tick
    Effect::new(move |_| {
        let _tick = state.frame_tick.get();
        // Also re-run when 3D config changes (background color, grid, etc.)
        layout.three_dee_configs.track();

        // Apply config to scene state
        let cfg = layout.get_three_dee_config(node_id);
        with_scene_mut(|s| {
            // Background color (parse hex)
            let hex = cfg.scene.background_color.trim_start_matches('#');
            if hex.len() == 6 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex[0..2], 16),
                    u8::from_str_radix(&hex[2..4], 16),
                    u8::from_str_radix(&hex[4..6], 16),
                ) {
                    s.bg_color = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
                }
            }

            // Sync all grids from config
            s.sync_grids(&cfg.custom_layers.grids);

            // Camera settings
            s.camera.perspective = cfg.view.perspective;
            s.camera.fov_y = (cfg.view.fovy as f32).to_radians();
            s.camera.near = cfg.view.near as f32;
            s.camera.far = cfg.view.far as f32;

            // Follow mode
            s.follow_mode = cfg.follow_mode.clone();

            // Render stats
            s.enable_stats = cfg.scene.enable_stats;

            // Transforms visual config
            s.tf_axis_scale = cfg.transforms.axis_scale as f32;
            s.tf_line_width = cfg.transforms.line_width as f32;
            // Parse transforms line color
            let lhex = cfg.transforms.line_color.trim_start_matches('#');
            if lhex.len() == 6 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&lhex[0..2], 16),
                    u8::from_str_radix(&lhex[2..4], 16),
                    u8::from_str_radix(&lhex[4..6], 16),
                ) {
                    s.tf_line_color = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
                }
            }

            // Manual TF offsets
            s.tf_offsets.clear();
            for (frame, offset) in &cfg.transforms.offsets {
                s.tf_offsets.insert(frame.clone(), [
                    offset.translation[0], offset.translation[1], offset.translation[2],
                    offset.rotation[0], offset.rotation[1], offset.rotation[2],
                ]);
            }
        });

        let Some(player) = get_player() else {
            return;
        };

        // Update current time for TF lookups
        let current_time_ns = player.current_time_ns();
        with_scene_mut(|s| {
            s.current_time_ns = current_time_ns;
        });

        let topics = player.topics();

        // ===== Process TF messages (native CDR) =====
        let mut frames_changed = false;
        for topic_info in topics.iter() {
            if is_tf_schema(&topic_info.schema_name) {
                if let Some(msg) = player.get_current_message(&topic_info.name) {
                    // Skip if same timestamp already processed
                    let mut should_process = false;
                    last_processed_times.update(|map| {
                        let last = map.get(&topic_info.name).copied().unwrap_or(0);
                        if msg.log_time_ns != last {
                            map.insert(topic_info.name.clone(), msg.log_time_ns);
                            should_process = true;
                        }
                    });
                    if !should_process {
                        continue;
                    }

                    let transforms = decode_tf_message_cdr(&msg.data);
                    if !transforms.is_empty() {
                        TF_STATE.with(|tf| {
                            let mut tree = tf.borrow_mut();
                            for t in transforms {
                                tree.insert(t);
                            }
                        });
                        frames_changed = true;
                    }
                }
            }
        }

        // ===== Process extension message converters (e.g. osi3 -> foxglove.FrameTransforms) =====
        for topic_info in topics.iter() {
            // Skip native TF topics (already handled above)
            if is_tf_schema(&topic_info.schema_name) {
                continue;
            }

            // Check if any extension converter is registered for this schema
            if !js_has_converters(&topic_info.schema_name) {
                continue;
            }

            // Don't retry schemas that already failed to compile
            let already_failed = FAILED_SCHEMAS.with(|s| {
                s.borrow().contains(&topic_info.schema_name)
            });
            if already_failed {
                continue;
            }

            // Ensure we have a compiled DescriptorPool for this schema
            let has_pool = PROTO_POOLS.with(|pools| {
                pools.borrow().contains_key(&topic_info.schema_name)
            });

            if !has_pool {
                // Try to compile the schema from MCAP FileDescriptorSet bytes
                if let Some(schema_data) = player.get_schema_data(&topic_info.schema_name) {
                    match prost_reflect::DescriptorPool::decode(schema_data.as_slice()) {
                        Ok(pool) => {
                            log::info!("Compiled proto descriptor pool for: {}", topic_info.schema_name);
                            PROTO_POOLS.with(|pools| {
                                pools.borrow_mut().insert(topic_info.schema_name.clone(), pool);
                            });
                        }
                        Err(e) => {
                            log::warn!("Failed to compile proto schema {}: {}", topic_info.schema_name, e);
                            FAILED_SCHEMAS.with(|s| {
                                s.borrow_mut().insert(topic_info.schema_name.clone());
                            });
                            continue;
                        }
                    }
                } else {
                    continue;
                }
            }

            // Get current message and decode in Rust, then pass to JS converters
            if let Some(msg) = player.get_current_message(&topic_info.name) {
                // Skip if same timestamp already processed
                let mut should_process = false;
                last_processed_times.update(|map| {
                    let last = map.get(&topic_info.name).copied().unwrap_or(0);
                    if msg.log_time_ns != last {
                        map.insert(topic_info.name.clone(), msg.log_time_ns);
                        should_process = true;
                    }
                });
                if !should_process {
                    continue;
                }

                // Decode protobuf in Rust using prost-reflect
                let js_obj = PROTO_POOLS.with(|pools| {
                    let pools_ref = pools.borrow();
                    let pool = pools_ref.get(&topic_info.schema_name)?;
                    let message_desc = pool.get_message_by_name(&topic_info.schema_name)?;
                    match prost_reflect::DynamicMessage::decode(message_desc, msg.data.as_slice()) {
                        Ok(dynamic_msg) => {
                            // Convert DynamicMessage to JsValue with snake_case field names
                            Some(dynamic_message_to_js(&dynamic_msg))
                        }
                        Err(e) => {
                            log::warn!("Failed to decode proto message {}: {}", topic_info.schema_name, e);
                            None
                        }
                    }
                });

                if let Some(message_obj) = js_obj {
                    // Serialize topic-specific config for the converter
                    let topic_cfg = cfg.topics.get(&topic_info.name).cloned().unwrap_or_default();
                    let config_js = serde_wasm_bindgen::to_value(&topic_cfg).unwrap_or(JsValue::NULL);

                    // Call FrameTransforms converter
                    let result = js_convert_message_with_object(&topic_info.schema_name, message_obj.clone(), config_js.clone());

                    if !result.is_null() && !result.is_undefined() {
                        if let Ok(array) = js_sys::Array::try_from(result) {
                            for i in 0..array.length() {
                                let frame_obj = array.get(i);
                                if let Some(tf) = parse_js_frame_transform(&frame_obj) {
                                    TF_STATE.with(|tree| {
                                        tree.borrow_mut().insert(tf);
                                    });
                                    frames_changed = true;
                                }
                            }
                        }
                    }

                    // Call SceneUpdate converter
                    let scene_result = js_convert_message_to_scene(&topic_info.schema_name, message_obj, config_js);
                    if !scene_result.is_null() && !scene_result.is_undefined() {
                        let (cubes, lines) = parse_scene_update_result(&scene_result);
                        if !cubes.is_empty() || !lines.is_empty() {
                            SCENE_ENTITIES.with(|ent| {
                                let mut e = ent.borrow_mut();
                                e.0 = cubes;
                                e.1 = lines;
                            });
                            with_scene_mut(|s| {
                                s.render();
                            });
                        }
                    }
                }
            }
        }

        // Update available frames signal if new frames were discovered
        if frames_changed {
            let frames = TF_STATE.with(|tf| tf.borrow().frames());
            let current_frames = state.tf_frames.get_untracked();
            if frames != current_frames {
                state.tf_frames.set(frames.clone());
                // Auto-select display frame if empty
                if state.display_frame.get_untracked().is_empty() && !frames.is_empty() {
                    // Prefer common root frames
                    let preferred = ["map", "odom", "world", "earth", "base_link", "Global"];
                    let selected = preferred.iter()
                        .find(|p| frames.contains(&p.to_string()))
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| frames[0].clone());
                    state.display_frame.set(selected);
                }
            }
        }

        // ===== Process PointCloud2 =====
        let pc_topic = topics
            .iter()
            .find(|t| is_point_cloud_schema(&t.schema_name));

        if let Some(topic_info) = pc_topic {
            if let Some(msg) = player.get_current_message(&topic_info.name) {
                // Skip if same timestamp already processed
                let mut should_process = false;
                last_processed_times.update(|map| {
                    let last = map.get(&topic_info.name).copied().unwrap_or(0);
                    if msg.log_time_ns != last {
                        map.insert(topic_info.name.clone(), msg.log_time_ns);
                        should_process = true;
                    }
                });

                if should_process {
                    if let Some((positions, colors)) =
                        decode_point_cloud2(&msg.data, &msg.encoding)
                    {
                        let count = (positions.len() / 3) as i32;
                        with_scene_mut(|s| {
                            s.update_point_cloud(&positions, &colors);
                            s.point_count = count;
                            s.render();
                        });
                    }
                } else {
                    // Same message, just re-render scene (camera may have moved)
                    with_scene_mut(|s| s.render());
                }
            }
        } else {
            // No point cloud - just render the grid
            with_scene_mut(|s| s.render());
        }

        // Update stats signals from scene
        with_scene(|s| {
            point_count.set(s.point_count);
            fps_signal.set(s.fps);
            line_count_signal.set(s.line_count);
        });
    });

    // Stats visibility from config
    let show_stats = move || {
        layout.get_three_dee_config(node_id).scene.enable_stats
    };

    let on_mouseup_clone = on_mouseup.clone();

    view! {
        <div class="panel-3d-canvas-container" style="position:relative;">
            <canvas
                node_ref=canvas_ref
                class="panel-3d-canvas"
                on:mousedown=on_mousedown
                on:mousemove=on_mousemove
                on:mouseup=on_mouseup
                on:mouseleave=on_mouseup_clone
                on:wheel=on_wheel
                on:contextmenu=move |ev: leptos::ev::MouseEvent| ev.prevent_default()
            />
            {move || {
                if show_stats() {
                    Some(view! {
                        <div class="render-stats-overlay" style="position:absolute; top:4px; left:4px; background:rgba(0,0,0,0.7); color:#0f0; font-family:monospace; font-size:11px; padding:4px 8px; border-radius:4px; pointer-events:none; z-index:10;">
                            <div>{move || format!("FPS: {:.0}", fps_signal.get())}</div>
                            <div>{move || format!("Points: {}", point_count.get())}</div>
                            <div>{move || format!("Lines: {}", line_count_signal.get())}</div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
