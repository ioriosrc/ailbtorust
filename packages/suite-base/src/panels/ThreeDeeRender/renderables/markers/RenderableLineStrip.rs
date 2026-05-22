```rust
use three::{Camera, CanvasRenderer, LineGeometry, LineMaterial, Material, Object3D, PerspectiveCamera, Scene};
use web_sys::CanvasRenderingContext2D;

struct RenderableLineStrip {
    geometry: LineGeometry,
    line_prepass: Line2,
    line: Line2,
    position_buffer: Vec<f32>,
    color_buffer: Vec<u8>,
}

impl RenderableLineStrip {
    fn new(topic: &str, marker: &Marker, receive_time: Option<bigint>) -> Self {
        let mut geometry = LineGeometry::new();
        let options = CameraOptions::default().resolution(receive_time.map(|t| t.as_f32())); // Assuming receive_time is in milliseconds

        // Depth pass 1
        let mat_line_prepass = make_line_prepass_material(marker, &options);
        self.line_prepass = Line2::new(&geometry, &mat_line_prepass);
        self.line_prepass.render_order = 1;
        self.line_prepass.is_visible = true; // Set to false initially

        // Color pass 2
        let mat_line = make_line_material(marker, &options);
        self.line = Line2::new(&geometry, &mat_line);
        self.line.render_order = 2;
        let picking_width = marker.scale.x * 1.2;
        self.line.set_picking_material(make_line_picking_material(picking_width, &options));

        self.update(marker, receive_time);

        Self {
            geometry,
            line_prepass,
            line,
            position_buffer: Vec::new(),
            color_buffer: Vec::new(),
        }
    }

    fn dispose(&mut self) {
        self.line_prepass.material.dispose();
        self.line.material.dispose();

        if let Some(picking_material) = self.line.userData.picking_material {
            picking_material.dispose();
            self.line.userData.picking_material = None;
        }

        self.geometry.dispose();
    }

    fn update(&mut self, new_marker: &Marker, receive_time: Option<bigint>) {
        let prev_marker = self.marker.clone();
        self.update_marker(new_marker, receive_time);
        let marker = self.marker;

        if marker.points.is_empty() {
            // THREE.LineGeometry.setPositions crashes when given an empty array:
            // https://github.com/foxglove/studio/issues/3954
            self.line_prepass.is_visible = false;
            self.line.is_visible = false;
            return;
        } else {
            self.line_prepass.is_visible = true;
            self.line.is_visible = true;
        }

        if marker.has_transparency() != prev_marker.has_transparency() {
            self.line_prepass.material.set_transparent(marker.has_transparency());
            self.line_prepass.material.depth_write = !marker.has_transparency();
            self.line_prepass.geometry.update_instance_count(marker.points.len() - 1);
            self.line.material.set_transparent(marker.has_transparency());
            self.line.material.depth_write = !marker.has_transparency();
            self.line.geometry.update_instance_count(marker.points.len() - 1);
        }

        let mat_line_prepass = &self.line_prepass.material as &LineMaterialWithAlphaVertex;
        mat_line_prepass.line_width = marker.scale.x;
        let mat_line = &self.line.material as &LineMaterialWithAlphaVertex;
        mat_line.line_width = marker.scale.x;

        if marker.points.len() > self.position_buffer.len() / 3 {
            self.position_buffer.resize(marker.points.len() * 3);
        }
        let positions = &mut self.position_buffer;
        for i in 0..marker.points.len() {
            let point = &marker.points[i];
            let offset = i * 3;
            positions[offset] = point.x as f32;
            positions[offset + 1] = point.y as f32;
            positions[offset + 2] = point.z as f32;
        }

        self.geometry.set_positions(positions);
        self.geometry.update_instance_count(marker.points.len() - 1);

        if marker.has_transparency() {
            let instance_color_buffer = THREE.InstancedInterleavedBuffer::new(&self.color_buffer, 8, 1);
            self.geometry.setAttribute(
                "instanceColorStart",
                &THREE.InterleavedBufferAttribute::new(instance_color_buffer, 4),
            );
            self.geometry.setAttribute(
                "instanceColorEnd",
                &THREE.InterleavedBufferAttribute::new(instance_color_buffer, 4),
            );
        }

        let color_buffer = &mut self.color_buffer;
        let color1: [f32; 4] = [0.0; 4];
        copy_tuple4(color1, color1);
        self._marker_colors_to_linear(marker, marker.points.len(), |color2, ii| {
            if ii == 0 {
                copy_tuple4(&color2, &color1);
                return;
            }
            let i = ii - 1;
            let offset = i * 8;

            color_buffer[offset] = (color1[0] as f32 * 255.0).round() as u8;
            color_buffer[offset + 1] = (color1[1] as f32 * 255.0).round() as u8;
            color_buffer[offset + 2] = (color1[2] as f32 * 255.0).round() as u8;
            color_buffer[offset + 3] = (color1[3] as f32 * 255.0).round() as u8;

            color_buffer[offset + 4] = (color2[0] as f32 * 255.0).round() as u8;
            color_buffer[offset + 5] = (color2[1] as f32 * 255.0).round() as u8;
            color_buffer[offset + 6] = (color2[2] as f32 * 255.0).round() as u8;
            color_buffer[offset + 7] = (color2[3] as f32 * 255.0).round() as u8;

            copy_tuple4(&color2, &color1);
        });
    }

    fn _marker_colors_to_linear(&self, marker: &Marker, points_length: usize, callback: impl FnMut(&[f32], usize)) {
        // Converts color-per-point to pairs format in a flattened typed array
        if self.color_buffer.len() < 8 * points_length {
            self.color_buffer.resize(8 * points_length);
        }
        let color_buffer = &mut self.color_buffer;
        let color1: [f32; 4] = [0.0; 4];
        callback(&color1, 0);

        for i in 1..points_length {
            let point = &marker.points[i];
            let offset = i * 8;

            color_buffer[offset] = (color1[0] as f32 * 255.0).round() as u8;
            color_buffer[offset + 1] = (color1[1] as f32 * 255.0).round() as u8;
            color_buffer[offset + 2] = (color1[2] as f32 * 255.0).round() as u8;
            color_buffer[offset + 3] = (color1[3] as f32 * 255.0).round() as u8;

            color_buffer[offset + 4] = (point.color.r as f32 * 255.0).round() as u8;
            color_buffer[offset + 5] = (point.color.g as f32 * 255.0).round() as u8;
            color_buffer[offset + 6] = (point.color.b as f32 * 255.0).round() as u8;
            color_buffer[offset + 7] = (point.color.a as f32 * 255.0).round() as u8;

            callback(&point.color, i);
        }
    }
}

fn copy_tuple4(from: [f32; 4], to: [f32; 4]) {
    to[0] = from[0];
    to[1] = from[1];
    to[2] = from[2];
    to[3] = from[3];
}
```