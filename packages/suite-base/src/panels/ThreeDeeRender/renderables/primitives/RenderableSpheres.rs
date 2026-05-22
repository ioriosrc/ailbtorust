```rust
use three::{BufferAttribute, Geometry, Material, Mesh, Matrix4, Quaternion, Vector3, Color};
use std::cell::RefCell;

struct Sphere {
    position: Vector3,
    orientation: Quaternion,
    size: Vector3,
}

pub struct RenderableSpheres {
    geometry: RefCell<Geometry>,
    material: Material,
    instance_matrix: BufferAttribute<f64, 16>,
    instance_color: Option<BufferAttribute<Color, 1>>,
    expires_at: Option<chrono::Duration>,
}

impl RenderableSpheres {
    pub fn new(renderer: &Renderer) -> Self {
        let geometry = RefCell::new(
            Geometry::from_geometry_type(GeometryType::Sphere)
                .with_uv(true)
                .with_index_buffer(PrimitiveType::Triangles)
                .unwrap(),
        );

        let material = Material::default();

        let instance_matrix = BufferAttribute::new(vec![0.0; 16 * 16], 4);
        let instance_color = None;

        Self {
            geometry,
            material,
            instance_matrix,
            instance_color,
            expires_at: None,
        }
    }

    pub fn update(&mut self, topic: Option<&str>, entity: Option<&SceneEntity>, settings: &LayerSettingsEntity, receive_time: chrono::Duration) {
        if let Some(entity) = entity {
            let lifetime_ns = entity.lifetime.ns();
            self.expires_at = Some(receive_time + lifetime_ns);
            self.update_mesh(entity.spheres());
        }
    }

    pub fn update_settings(&mut self, settings: &LayerSettingsEntity) {
        self.update(None, None, settings, chrono::Duration::zero());
    }

    fn update_mesh(&mut self, spheres: &[Sphere]) {
        let is_transparent = !spheres.iter().any(|sphere| sphere.color.a < 1.0);

        self.ensure_capacity(spheres.len());

        let override_color = if let Some(color) = settings.color {
            color.to_rgba()
        } else {
            None
        };

        for (i, sphere) in spheres.iter_mut().enumerate() {
            let color = override_color.unwrap_or(sphere.color);
            self.geometry.borrow_mut().set_attribute("color", &[Color::rgba_to_u32(color)]);
            self.instance_matrix.set_x(i, color.a);
            self.instance_matrix.set_y(i, sphere.position.x);
            self.instance_matrix.set_z(i, sphere.position.y);
            self.instance_matrix.set_w(i, sphere.position.z);
        }

        if self.material.transparent != is_transparent {
            self.material.transparent = is_transparent;
            self.material.needs_update();
        }

        if self.geometry.borrow().get_attribute("color").is_none() && spheres.len() > 0 {
            // needed to make colors work: https://discourse.threejs.org/t/instancedmesh-color-doesnt-work-when-initial-count-is-0/41355
            self.material.needs_update();
        }

        self.geometry.borrow_mut().set_attribute("instance_matrix", &[self.instance_matrix[i] for i in 0..spheres.len()]);
    }

    fn ensure_capacity(&mut self, num_instances: usize) {
        if num_instances > self.instance_matrix.capacity() / 16 {
            let new_capacity = (num_instances as f64 * 1.5).ceil() as usize;
            self.instance_matrix.resize(new_capacity * 16, 0.0);
        }
    }

    fn dispose(&mut self) {
        self.geometry.borrow_mut().clear();
        self.material.dispose();
    }
}

fn create_geometry() -> Geometry {
    let geometry = Geometry::from_geometry_type(GeometryType::Sphere)
        .with_uv(true)
        .with_index_buffer(PrimitiveType::Triangles)
        .unwrap();

    geometry.compute_bounding_sphere();
    geometry
}
```