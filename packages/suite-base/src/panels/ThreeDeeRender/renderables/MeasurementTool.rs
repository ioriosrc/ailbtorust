```rust
use bevy::{prelude::*, render::camera as camera_plugin, utils::Vec3};

const LATE_RENDER_ORDER: i32 = 9999999;

struct FixedSizeMeshMaterial {
    color: Vec3,
}

impl Material for FixedSizeMeshMaterial {
    fn fragment_shader(&self) -> &'static str {
        r#"
            #include <common>
            uniform vec2 canvas_size;
            void main() {
                vec4 mv_position = model_view_matrix * vec4(0., 0., 0., 1.);

                // Adapted from THREE.ShaderLib.sprite
                vec2 scale;
                scale.x = length(vec3(modelMatrix[0].xyz));
                scale.y = length(vec3(modelMatrix[1].xyz));

                gl_Position = projection_matrix * mv_position;

                // Add position after projection to maintain constant pixel size
                gl_Position.xy += position.xy / canvas_size * scale * gl_Position.w;
            }
        "#}
    }

    fn vertex_shader(&self) -> &'static str {
        r#"
            #include <common>
            uniform vec2 canvas_size;
            void main() {
                vec4 mv_position = model_view_matrix * vec4(0., 0., 0., 1.);

                // Adapted from THREE.ShaderLib.sprite
                vec2 scale;
                scale.x = length(vec3(modelMatrix[0].xyz));
                scale.y = length(vec3(modelMatrix[1].xyz));

                gl_Position = projection_matrix * mv_position;

                // Add position after projection to maintain constant pixel size
                gl_Position.xy += position.xy / canvas_size * scale * gl_Position.w;
            }
        "#}
    }

    fn uniforms(&self) -> Vec<(String, uniform::UniformRef)> {
        vec![(
            "canvas_size".to_string(),
            uniform::Vec2Uniform::new(self.canvas_size),
        )]
    }
}

pub struct MeasurementTool {
    circle_geometry: mesh::MeshGeometry,
    circle_material: Material,
    circle1: mesh::Mesh,
    circle2: mesh::Mesh,
    line_position_attribute: BufferAttribute<f32>,
    line: mesh::Mesh,
    line_occluded: mesh::Mesh,
    label: Label,

    point1_needs_update: bool,
    point2_needs_update: bool,

    point1: Option<Vec3>,
    point2: Option<Vec3>,

    state: MeasurementState,
}

enum MeasurementState {
    Idle,
    PlaceFirstPoint,
    PlaceSecondPoint,
}

impl MeasurementTool {
    pub fn new(renderer: &Renderer, name: String) -> Self {
        let circle_geometry = mesh::MeshGeometry::new_sphere(5.0, 16);
        let circle_material = Material::from(FixedSizeMeshMaterial {
            color: renderer.color_picker.pick_color("MeasurementTool Circle").into(),
        });
        let circle1 = mesh::Mesh::new(&circle_geometry, &circle_material);
        let circle2 = mesh::Mesh::new(&circle_geometry, &circle_material);

        let line_position_attribute = BufferAttribute::new(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 3);
        let line = mesh::Mesh::new(
            &mesh::MeshGeometry::from_points(vec![Vec3::ZERO; 2]),
            &line_material,
        );
        let line_occluded = mesh::Mesh::new(
            &mesh::MeshGeometry::from_points(vec![Vec3::ZERO; 2]),
            &LineDashedMaterial {
                color: renderer.color_picker.pick_color("MeasurementTool Line").into(),
                dash_size: 1.0,
                gap_size: 1.0,
                depth_test: false,
                depth_func: bevy::render::camera::DepthFunc::GreaterDepth, // opposite of default THREE.LessEqualDepth
            },
        );

        let label = renderer.label_pool.acquire();
        label.visible = false;
        label.set_billboard(true);
        label.set_size_attenuation(false);
        label.set_line_height(12);
        label.set_color(renderer.color_picker.pick_color("MeasurementTool Label").into());

        // Make the label appear on top of other objects in the scene so it doesn't get clipped/occluded
        label.render_order = LATE_RENDER_ORDER;
        label.material.depth_test = false;
        label.material.depth_write = false;
        label.material.transparent = true;

        line.frustum_culled = false;
        line_occluded.frustum_culled = false;
        line.geometry.set_attribute("position", &line_position_attribute);
        lineOccluded.compute_line_distances();
        line_position_attribute.needs_update = true;

        Self {
            circle_geometry,
            circle_material,
            circle1,
            circle2,
            line_position_attribute,
            line,
            line_occluded,
            label,
            point1_needs_update: false,
            point2_needs_update: false,
            point1: None,
            point2: None,
            state: MeasurementState::Idle,
        }
    }

    pub fn dispose(&mut self) {
        self.renderer.label_pool.release(self.label);
        self.circle_geometry.dispose();
        self.circle_material.dispose();
        self.line.geometry.dispose();
        self.line.material.dispose();
        self.line_occluded.geometry.dispose();
        self.line_occluded.material.dispose();
    }

    pub fn start_measuring(&mut self) {
        self.state = MeasurementState::PlaceFirstPoint;
        self.point1 = None;
    }

    pub fn stop_measuring(&mut self) {
        self.point1 = None;
        self.point2 = None;
        self.state = MeasurementState::Idle;
    }

    pub fn start_frame(
        &mut self,
        _current_time: u64,
        render_frame_id: String,
        fixed_frame_id: String,
    ) {
        let canvas_size = self.renderer.input.canvas_size();
        self.circle_material.uniforms()["canvas_size"] = uniform::Vec2Uniform::new(canvas_size);

        if self.point1.is_some() || self.point2.is_some() {
            self.render();
        }
    }

    fn set_state(&mut self, state: MeasurementState) {
        self.state = state;
        match state {
            MeasurementState::Idle => {
                self.renderer.input.remove_listener("click", &self.handle_click);
                self.renderer.input.remove_listener("mousemove", &self.handle_mousemove);
                self.dispatchEvent(Event { type: "foxglove.measure-end" });
            }
            MeasurementState::PlaceFirstPoint => {
                self.point1 = None;
                self.renderer.input.add_listener("click", &self.handle_click);
                self.renderer.input.add_listener("mousemove", &self.handle_mousemove);
                self.dispatchEvent(Event { type: "foxglove.measure-start" });
            }
            MeasurementState::PlaceSecondPoint => {}
        }
    }

    fn handle_mousemove(
        &mut self,
        _cursor_coords: Vec2,
        world_space_cursor_coords: Option<Vec3>,
        _event: Event<MouseEvent>,
    ) {
        if let Some(world_space_cursor_coords) = world_space_cursor_coords {
            match self.state {
                MeasurementState::Idle => {}
                MeasurementState::PlaceFirstPoint => {
                    (self.point1.take() ??= Vec3::ZERO).copy_from(&world_space_cursor_coords);
                    self.point1_needs_update = true;
                }
                MeasurementState::PlaceSecondPoint => {
                    (self.point2.take() ??= Vec3::ZERO).copy_from(&world_space_cursor_coords);
                    self.point2_needs_update = true;
                    self.update_distance();
                }
            }
            self.render();
        }
    }

    fn update_distance(&mut self) {
        if let Some((point1, point2)) = (self.point1.as_ref(), self.point2.as_ref()) {
            if point1.is_some() && point2.is_some() {
                self.label.set_text(format!("{:.2}", point1.unwrap().distance_to(point2.unwrap())));
            }
        }
    }

    fn render(&mut self) {
        let is_circle_visible = |point| match point {
            Some(_) => true,
            None => false,
        };

        if is_circle_visible(self.point1.as_ref()) {
            self.circle1.visible = true;
            self.circle1.position.copy_from(&self.point1.unwrap());
            self.line_position_attribute.set_xyz(0, self.point1.unwrap().x, self.point1.unwrap().y, self.point1.unwrap().z);
            self.line_position_attribute.needs_update = true;
        } else {
            self.circle1.visible = false;
        }

        if is_circle_visible(self.point2.as_ref()) {
            self.circle2.visible = true;
            self.circle2.position.copy_from(&self.point2.unwrap());
            self.line_position_attribute.set_xyz(1, self.point2.unwrap().x, self.point2.unwrap().y, self.point2.unwrap().z);
            self.line_position_attribute.needs_update = true;
        } else {
            self.circle2.visible = false;
        }

        if let (Some(point1), Some(point2)) = (self.point1.as_ref(), self.point2.as_ref()) {
            self.line.visible = true;
            self.lineOccluded.visible = true;
            self.label.visible = true;
            self.label.position.lerp(&point1.unwrap(), 0.5).copy_to(&self.label.position);
        } else {
            self.line.visible = false;
            self.lineOccluded.visible = false;
            self.label.visible = false;
        }

        self.renderer.queue_frame();
    }
}
```