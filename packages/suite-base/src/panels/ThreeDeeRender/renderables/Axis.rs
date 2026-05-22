```rust
use three::{Geometry, Material, Mesh, Matrix4, Vector3};
use std::vec::Vec;

pub struct Axis {
    renderer: Box<dyn IRenderer>,
    shaft_mesh: Option<InstancedMesh<Geometry, StandardMaterial>>,
    head_mesh: Option<InstancedMesh<Geometry, StandardMaterial>>,
}

impl Axis {
    pub fn new(name: &str, renderer: impl IRenderer + 'static) -> Self {
        let shaft_geometry = Self::renderer.shared_geometry.get_geometry(
            format!("{}-shaft-{}", name, renderer.max_lod),
            || create_shaft_geometry(renderer.max_lod),
        );
        let head_geometry = Self::renderer.shared_geometry.get_geometry(
            format!("{}-head-{}", name, renderer.max_lod),
            || create_head_geometry(renderer.max_lod),
        );

        let shaft_mesh = InstancedMesh {
            geometry: shaft_geometry,
            material: StandardMaterial::new(ColorRGBA::WHITE),
            count: 3,
        };
        shaft_mesh.frustum_culled = false;
        shaft_mesh.cast_shadow = true;
        shaft_mesh.receive_shadow = true;

        let head_mesh = InstancedMesh {
            geometry: head_geometry,
            material: StandardMaterial::new(ColorRGBA::WHITE),
            count: 3,
        };
        head_mesh.frustum_culled = false;
        head_mesh.cast_shadow = true;
        head_mesh.receive_shadow = true;

        Self {
            name,
            renderer: Box::new(renderer),
            shaft_mesh: Some(shaft_mesh),
            head_mesh: Some(head_mesh),
        }
    }

    pub fn dispose(&mut self) {
        if let Some(ref mut shaft_mesh) = &mut self.shaft_mesh {
            shaft_mesh.material.dispose();
            shaft_mesh.dispose();
        }
        if let Some(ref mut head_mesh) = &mut self.head_mesh {
            head_mesh.material.dispose();
            head_mesh.dispose();
        }
    }

    static fn update_instances(
        shaft: InstancedMesh<Geometry, StandardMaterial>,
        head: InstancedMesh<Geometry, StandardMaterial>,
        axis_index: usize,
    ) -> () {
        let index_x = axis_index * 3 + 0;
        let index_y = axis_index * 3 + 1;
        let index_z = axis_index * 3 + 2;

        // Set x, y, and z axis arrow shaft directions
        let mut temp_vec = Vector3::new(SHAFT_LENGTH, SHAFT_DIAMETER, SHAFT_DIAMETER);
        let mut mat4 = Matrix4::identity();
        mat4.scale(temp_vec).translate(0.5, 0, 0);

        for i in 0..=2 {
            shaft.set_matrix_at(index_x + i, &mat4);
            head.set_matrix_at(index_x + i, &mat4);
        }

        // Set x, y, and z axis arrow head directions
        temp_vec = Vector3::new(HEAD_LENGTH, HEAD_DIAMETER, HEAD_DIAMETER);
        mat4.identity().scale(temp_vec).translate(SHAFT_LENGTH, 0, 0);

        for i in 0..=2 {
            head.set_matrix_at(index_x + i, &mat4);
        }
    }
}

fn create_shaft_geometry(lod: usize) -> Geometry {
    let subdivs = arrow_shaft_subdivisions(lod);
    let shaft_geometry = Geometry::new();
    shaft_geometry.add_buffer(0.5, 0.5, 1, subdivs, 1, false);
    shaft_geometry.rotate_z(-PI_2);
    shaft_geometry.translate(0.5, 0, 0);
    shaft_geometry.compute_bounding_sphere();
    shaft_geometry
}

fn create_head_geometry(lod: usize) -> Geometry {
    let subdivs = arrow_head_subdivisions(lod);
    let head_geometry = Geometry::new();
    head_geometry.add_buffer(0.5, 1, subdivs, 1, false);
    head_geometry.rotate_z(-PI_2);
    head_geometry.translate(0.5, 0, 0);
    head_geometry.compute_bounding_sphere();
    head_geometry
}

fn standard_material(color: ColorRGBA) -> Material {
    Material::new({
        color: three::Color::from_srgb(color.r, color.g, color.b),
        metalness: 0.,
        roughness: 1.,
        dithering: true,
        opacity: color.a,
        transparent: color.a < 1.,
        depth_write: color.a == 1.,
    })
}
```