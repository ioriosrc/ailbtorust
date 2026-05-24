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

use crate::state::app_state::{get_player, use_app_state};

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
}

impl OrbitCamera {
    pub fn new() -> Self {
        Self {
            target: Vec3::new(0.0, 0.0, 0.0),
            distance: 15.0,
            azimuth: std::f32::consts::FRAC_PI_4,    // 45°
            elevation: std::f32::consts::FRAC_PI_6,  // 30°
            fov_y: std::f32::consts::FRAC_PI_4,      // 45° fov
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
        Mat4::perspective(self.fov_y, aspect, 0.1, 1000.0)
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
out vec4 v_color;
void main() {
    gl_Position = u_viewProjection * vec4(a_position, 1.0);
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
fn generate_grid_and_axes(size: i32, spacing: f32) -> (Vec<f32>, Vec<f32>) {
    let mut positions: Vec<f32> = Vec::new();
    let mut colors: Vec<f32> = Vec::new();

    let half = size as f32 * spacing;

    // Grid lines (gray)
    for i in -size..=size {
        let pos = i as f32 * spacing;
        let alpha = if i == 0 { 0.0 } else { 0.3 }; // Skip center lines (axes go there)

        // Line along X
        positions.extend_from_slice(&[-half, 0.0, pos, half, 0.0, pos]);
        colors.extend_from_slice(&[0.5, 0.5, 0.5, alpha, 0.5, 0.5, 0.5, alpha]);

        // Line along Z
        positions.extend_from_slice(&[pos, 0.0, -half, pos, 0.0, half]);
        colors.extend_from_slice(&[0.5, 0.5, 0.5, alpha, 0.5, 0.5, 0.5, alpha]);
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

// ============ Scene State ============

struct SceneState {
    gl: GL,
    grid_program: WebGlProgram,
    grid_vao: WebGlVertexArrayObject,
    grid_vertex_count: i32,
    point_cloud_program: WebGlProgram,
    point_cloud_vao: WebGlVertexArrayObject,
    point_cloud_vertex_count: i32,
    point_cloud_buffer_pos: WebGlBuffer,
    point_cloud_buffer_color: WebGlBuffer,
    camera: OrbitCamera,
    canvas_width: u32,
    canvas_height: u32,
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

        // Generate grid geometry
        let (grid_pos, grid_col) = generate_grid_and_axes(20, 1.0);
        let grid_vertex_count = (grid_pos.len() / 3) as i32;

        // Create grid VAO
        let grid_vao = gl.create_vertex_array().ok_or("Failed to create VAO")?;
        gl.bind_vertex_array(Some(&grid_vao));

        // Position buffer
        let pos_buf = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&pos_buf));
        unsafe {
            let array = js_sys::Float32Array::view(&grid_pos);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
        }
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        // Color buffer
        let col_buf = gl.create_buffer().ok_or("Failed to create buffer")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&col_buf));
        unsafe {
            let array = js_sys::Float32Array::view(&grid_col);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &array, GL::STATIC_DRAW);
        }
        gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        gl.bind_vertex_array(None);

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

        Ok(Self {
            gl,
            grid_program,
            grid_vao,
            grid_vertex_count,
            point_cloud_program,
            point_cloud_vao,
            point_cloud_vertex_count: 0,
            point_cloud_buffer_pos,
            point_cloud_buffer_color,
            camera: OrbitCamera::new(),
            canvas_width: width,
            canvas_height: height,
        })
    }

    fn render(&self) {
        let gl = &self.gl;

        gl.viewport(0, 0, self.canvas_width as i32, self.canvas_height as i32);
        gl.clear_color(0.12, 0.12, 0.14, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.enable(GL::DEPTH_TEST);
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        let aspect = self.canvas_width as f32 / self.canvas_height.max(1) as f32;
        let view = self.camera.view_matrix();
        let proj = self.camera.projection_matrix(aspect);
        let vp = proj.multiply(&view);

        // Draw grid
        gl.use_program(Some(&self.grid_program));
        let vp_loc = gl.get_uniform_location(&self.grid_program, "u_viewProjection");
        gl.uniform_matrix4fv_with_f32_array(vp_loc.as_ref(), false, &vp.data);

        gl.bind_vertex_array(Some(&self.grid_vao));
        gl.draw_arrays(GL::LINES, 0, self.grid_vertex_count);

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

// ============ Thread-local Scene Storage ============

thread_local! {
    static SCENE: std::cell::RefCell<Option<SceneState>> = std::cell::RefCell::new(None);
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
pub fn ThreeDeePanel() -> impl IntoView {
    let state = use_app_state();
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let point_count = RwSignal::new(0i32);

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
            Ok(s) => {
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

    // Update point cloud from player data on frame tick
    Effect::new(move |_| {
        let _tick = state.frame_tick.get();

        let Some(player) = get_player() else {
            return;
        };

        // Find PointCloud2 topics
        let topics = player.topics();
        let pc_topic = topics
            .iter()
            .find(|t| is_point_cloud_schema(&t.schema_name));

        if let Some(topic_info) = pc_topic {
            if let Some(msg) = player.get_current_message(&topic_info.name) {
                if let Some((positions, colors)) =
                    decode_point_cloud2(&msg.data, &msg.encoding)
                {
                    let count = (positions.len() / 3) as i32;
                    with_scene_mut(|s| {
                        s.update_point_cloud(&positions, &colors);
                        s.render();
                    });
                    point_count.set(count);
                }
            }
        } else {
            // No point cloud - just render the grid
            with_scene(|s| s.render());
        }
    });

    let on_mouseup_clone = on_mouseup.clone();

    view! {
        <div class="panel-3d-canvas-container">
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
        </div>
    }
}
