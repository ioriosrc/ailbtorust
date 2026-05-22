```rust
use std::ops::{AddAssign, Sub};

const THREE_TO_LINEAR_SRGB: f32 = 1.0 / 2.4;

struct PickingMaterial {
    uniforms: HashMap<String, any>,
}

impl PickingMaterial {
    fn new() -> Self {
        let uniforms = HashMap::new();
        uniforms.insert(String::from("objectId"), vec![f64::NAN; 4]);
        PickingMaterial { uniforms }
    }
}

#[derive(Debug)]
struct RenderablePointsAnnotation {
    geometry: DynamicBufferGeometry,
    points: THREE::Points,
    picking_material: PickingMaterial,
    scale: f32,
    pixel_ratio: f32,
    scale_needs_update: bool,
    original_message: Option<RosObject>,
    annotation: Option<NormalizedPointsAnnotation & { style: "points" }>,
    annotation_needs_update: bool,
    camera_model: Option<ICameraModel>,
    camera_model_needs_update: bool,
}

impl RenderablePointsAnnotation {
    fn new(topic_name: String) -> Self {
        let geometry = DynamicBufferGeometry::new();
        geometry.create_attribute("position", f32::array![3; 10], 3);
        geometry.create_attribute("color", u8::array![4; 10], 4, true);

        let points_material = THREE::PointsMaterial {
            size: 0,
            size_attenuation: false,
            vertex_colors: true,
            ..annotation_render_order_material_props()
        };

        let picking_material = PickingMaterial::new();

        let points = THREE::Points(geometry.clone(), points_material);
        points.render_order = ANNOTATION_RENDER_ORDER::POINTS;
        points.userData.picking_material = &picking_material;

        RenderablePointsAnnotation {
            geometry,
            points,
            picking_material,
            scale: 0.0,
            pixel_ratio: 0.0,
            scale_needs_update: false,
            original_message: None,
            annotation: None,
            annotation_needs_update: false,
            camera_model: None,
            camera_model_needs_update: false,
        }
    }

    fn dispose(&mut self) {
        self.geometry.dispose();
        self.points_material.dispose();
        self.picking_material.dispose();
        // You might need to handle other resources here
    }

    fn details(&self) -> Option<HashMap<String, RosValue>> {
        if let (Some(original_message), Some(annotation)) = (&self.original_message, &self.annotation) {
            return Some({
                "annotation": get_annotationAtPath(original_message, annotation.message_path),
                "originalMessage": original_message,
            });
        }
        None
    }

    fn set_scale(
        &mut self,
        scale: f32,
        canvas_width: f32,
        canvas_height: f32,
        pixel_ratio: f32,
    ) {
        self.scale_needs_update = scale != self.scale || pixel_ratio != self.pixel_ratio;
        self.scale = scale;
        self.pixel_ratio = pixel_ratio;
    }

    fn set_camera_model(&mut self, camera_model: Option<ICameraModel>) {
        self.camera_model_needs_update = self.camera_model != camera_model;
        self.camera_model = camera_model;
    }

    fn set_annotation(
        &mut self,
        annotation: NormalizedPointsAnnotation & { style: "points" },
        original_message: Option<RosObject>,
    ) {
        self.annotation_needs_update = self.annotation != Some(annotation);
        self.original_message = original_message;
        self.annotation = Some(annotation);
    }

    fn update(&mut self) {
        if !self.annotation.is_some() || !self.camera_model.is_some() {
            self.visible = false;
            return;
        }
        self.visible = true;

        if self.annotation_needs_update || self.scale_needs_update {
            self.annotation_needs_update = false;
            let { thickness } = self.annotation.unwrap();
            // thickness specifies radius, PointsMaterial.size specifies diameter
            self.picking_material.uniforms["size"].as_mut().unwrap()[0] = thickness * 2 * self.scale;
            self.points_material.needs_update = true;

            self.picking_material.uniforms["size"].as_mut().unwrap()[1] =
                thickness * 2 * self.scale * self.pixel_ratio;
            self.picking_material.needs_update = true;
        }

        if self.annotation_needs_update || self.camera_model_needs_update {
            self.annotation_needs_update = false;
            self.camera_model_needs_update = false;

            let { points, outline_colors, outline_color, fillColor } = self.annotation.unwrap();

            self.geometry.resize(points.len() as usize);
            let position_attribute = self.geometry.getAttribute("position") as &mut THREE::BufferAttribute<f32>;
            let color_attribute = self.geometry.getAttribute("color") as &mut THREE::BufferAttribute<u8>;

            for i in 0..points.len() {
                let color = outline_colors.get(i).unwrap_or(&outline_color);
                let point = points[i].clone().unwrap();

                let camera_model = self.camera_model.as_ref();
                if let Some(camera_model) = camera_model {
                    camera_model.project_pixel_to_3d_plane(temp_vec3, point);
                }

                position_attribute.set_component(i * 3 + 0, temp_vec3.x as f32);
                position_attribute.set_component(i * 3 + 1, temp_vec3.y as f32);
                position_attribute.set_component(i * 3 + 2, temp_vec3.z as f32);

                color_attribute.set_component(
                    i * 4 + 0,
                    SRGBToLinear(color.r) * 255.0f32,
                );
                color_attribute.set_component(
                    i * 4 + 1,
                    SRGBToLinear(color.g) * 255.0f32,
                );
                color_attribute.set_component(
                    i * 4 + 2,
                    SRGBToLinear(color.b) * 255.0f32,
                );
                color_attribute.set_component(
                    i * 4 + 3,
                    (color.a as f32) * 255.0f32,
                );
            }

            position_attribute.needs_update = true;
            color_attribute.needs_update = true;
        }
    }
}

fn SRGBToLinear(color: u8) -> f32 {
    let r = (color >> 16) as f32 / 255.0f32;
    let g = (color >> 8) as f32 / 255.0f32;
    let b = color as f32 / 255.0f32;

    return f32::pow(r, 1.0 / 2.4) * 255.0f32
        + f32::pow(g, 1.0 / 2.4) * 255.0f32
        + f32::pow(b, 1.0 / 2.4) * 255.0f32;
}
```