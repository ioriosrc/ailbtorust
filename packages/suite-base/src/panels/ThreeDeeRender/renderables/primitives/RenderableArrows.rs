```rust
use crate::{
    color::{make_rgba, rgb_to_three_color},
    geometry::ShaftGeometry,
    material::MeshStandardMaterialWithInstanceOpacity,
};
use three::{LineSegments, MeshStandardMaterial, Object3D, Shape};

struct RenderableArrows {
    shaft_geometry: Box<ShaftGeometry>,
    head_geometry: Box<ShaftGeometry>,
    shaft_outline_geometry: Object3D,
    head_outline_geometry: Object3D,

    material: MeshStandardMaterialWithInstanceOpacity,
    instance_opacity: Vec<f32>,
}

impl RenderableArrows {
    pub fn new(renderer: &Renderer) -> Self {
        let shaft_material = Box::new(MeshStandardMaterialWithInstanceOpacity {
            metalness: 0.0,
            roughness: 1.0,
            dithering: true,
        });

        let shaft_geometry = renderer.shared_geometry.get_or_create(
            &["RenderableArrows", "shaft"],
            create_shaft_geometry,
        );
        shaft_geometry.setAttribute("instance_opacity", &self.instance_opacity);

        let head_geometry = renderer.shared_geometry.get_or_create(
            &["RenderableArrows", "head"],
            create_head_geometry,
        );
        head_geometry.setAttribute("instance_opacity", &self.instance_opacity);

        self.shaft_outline_geometry = Object3D::new();
        self.shaft_outline_geometry.add(create_shaft_edges_geometry(&shaft_geometry));
        self.shaft_outline_geometry.name = "shaftedges".to_string();

        self.head_outline_geometry = Object3D::new();
        self.head_outline_geometry.add(create_head_edges_geometry(&head_geometry));
        self.head_outline_geometry.name = "headedges".to_string();

        Self {
            shaft_geometry,
            head_geometry,
            shaft_outline_geometry,
            head_outline_geometry,
            material: shaft_material,
            instance_opacity: vec![1.0; 16],
        }
    }

    fn ensure_capacity(&mut self, num_arrows: usize) {
        if num_arrows > self.instance_opacity.len() {
            let new_capacity = (num_arrows as f32 * 1.5).ceil() as usize + 16;
            self.instance_opacity.resize(new_capacity);
        }
    }

    fn update_mesh(&mut self, arrows: &[ArrowPrimitive]) {
        let override_color = if let Some(color) = &self.userData.settings.color {
            string_to_rgba(color)
        } else {
            None
        };

        let mut is_transparent = false;
        for arrow in arrows {
            if override_color.is_some() && override_color.unwrap().a < 1.0 {
                is_transparent = true;
            }
            self.shaft_mesh.set_color_at(arrow.index, rgb_to_three_color(color));
            self.head_mesh.set_color_at(arrow.index, rgb_to_three_color(color));
            self.instance_opacity[arrow.index] = color.a;

            let mut temp_quat = Quat::identity();
            let position = Vec3::from_array(&arrow.pose.position);
            let rotation = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), arrow.pose.orientation.z);

            let mut temp_vec3 = Vec3::from_array(&arrow.shaft_length);
            let mut temp_vec3_2 = Vec3::from_array(&arrow.shaft_diameter).scale(1.0 + arrow.shaft_diameter / 2.0);
            self.shaft_mesh.set_matrix_at(
                arrow.index,
                Matrix4::from_translation(position) * rotation * Matrix4::from_scale(temp_vec3, temp_vec3_2),
            );

            // offset head position by shaft length in direction of arrow pose
            let mut temp_vec3 = Vec3::from_array(&arrow.shaft_length);
            let mut temp_quat = Quat::identity();
            self.head_mesh.set_matrix_at(
                arrow.index,
                Matrix4::from_translation(position) * rotation * Matrix4::from_scale(temp_vec3, temp_vec3_2),
            );
        }

        if self.material.transparent != is_transparent {
            self.material.transparent = is_transparent;
            self.material.depth_write = !is_transparent;
            self.material.needs_update();
        }

        if self.shaft_mesh.count == 0 && arrows.len() > 0 {
            // needed to make colors work: https://discourse.threejs.org/t/instancedmesh-color-doesnt-work-when-initial-count-is-0/41355
            self.material.needs_update();
        }
        self.shaft_mesh.count = arrows.len();
        self.head_mesh.count = arrows.len();
        self.shaft_outline_geometry.instance_count = arrows.len();
        self.head_outline_geometry.instance_count = arrows.len();

        if let Some(instances) = &self.shaft_mesh.instance_matrix {
            instances.needs_update();
        }
        if let Some(instances) = &self.head_mesh.instance_matrix {
            instances.needs_update();
        }
        if let Some(instances) = &self.instance_opacity {
            instances.needs_update();
        }

        if self.shaft_outline_geometry.is_visible() != self.userData.settings.show_outlines.unwrap_or(true) {
            self.shaft_outline_geometry.set_visible(self.userData.settings.show_outlines.unwrap_or(true));
        }
        if self.head_outline_geometry.is_visible() != self.userData.settings.show_outlines.unwrap_or(true) {
            self.head_outline_geometry.set_visible(self.userData.settings.show_outlines.unwrap_or(true));
        }
    }

    pub fn dispose(&mut self) {
        self.shaft_material.dispose();
        self.shaft_mesh.dispose();
        self.head_mesh.dispose();
        self.shaft_geometry.dispose();
        self.head_geometry.dispose();
        self.shaft_outline_geometry.remove_all_children();
        self.head_outline_geometry.remove_all_children();
    }

    pub fn update(
        &mut self,
        topic: Option<&str>,
        entity: Option<&SceneEntity>,
        settings: LayerSettingsEntity,
        receive_time: u64,
    ) {
        super::RenderablePrimitive::update(self, topic, entity, settings, receive_time);
        if let Some(lifetime_ns) = to_nanoseconds(entity.as_ref().map(|e| e.lifetime)) {
            self.userData.expires_at = lifetime_ns;
        } else {
            self.userData.expires_at = None;
        }
        self.update_mesh(&entity.unwrap_or_default().arrows);

        self.head_outline.visible =
            settings.show_outlines.unwrap_or(true) && entity.as_ref().map(|e| e.settings).is_some();
        self.shaft_outline.visible =
            settings.show_outlines.unwrap_or(true) && entity.as_ref().map(|e| e.settings).is_some();
    }

    pub fn update_settings(&mut self, settings: LayerSettingsEntity) {
        super::RenderablePrimitive::update_settings(self, settings);
    }
}

fn create_shaft_geometry() -> Box<ShaftGeometry> {
    let geometry = ShaftGeometry::new(0.5, 0.5, 1, 16);
    // Adjust cylinder so ends are centered on (0,0,0) and (1,0,0)
    geometry.rotate_z(-std::f32::consts::PI / 2).translate(0.5, 0, 0);
    geometry.compute_bounding_sphere();
    Box::new(geometry)
}

fn create_shaft_edges_geometry(geometry: &ShaftGeometry) -> Shape {
    let edges = EdgesGeometry::new(geometry.clone(), 40);
    edges.compute_bounding_sphere();
    EdgeShape::from_shape(edges)
}

fn create_head_geometry() -> Box<ShaftGeometry> {
    let geometry = ShaftGeometry::new(0.5, 1, 16);
    // Adjust cone so base is centered on (0,0,0) and tip is at (1,0,0)
    geometry.rotate_z(-std::f32::consts::PI / 2).translate(0.5, 0, 0);
    geometry.compute_bounding_sphere();
    Box::new(geometry)
}

fn create_head_edges_geometry(geometry: &ShaftGeometry) -> Shape {
    let edges = EdgesGeometry::new(geometry.clone(), 40);
    edges.compute_bounding_sphere();
    EdgeShape::from_shape(edges)
}
```

Note that the above code assumes that the `Renderer` and related types are defined in a separate crate, and that the `ArrowPrimitive`, `ShaftGeometry`, `MeshStandardMaterialWithInstanceOpacity`, and other necessary structs and functions are defined in that crate as well.