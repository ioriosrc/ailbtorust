```rust
use crate::color::*;
use crate::{DynamicBufferGeometry, Mesh, MeshStandardMaterial};
use glam::Vec4;
use std::{
    f64,
    ops::{AddAssign, DivAssign},
};

type TriangleMesh = Mesh<DynamicBufferGeometry, MeshStandardMaterial>;

pub struct RenderableTriangles {
    triangle_meshes: Vec<TriangleMesh>,
    expires_at: Option<u64>,
}

impl RenderableTriangles {
    pub fn new() -> Self {
        Self {
            triangle_meshes: vec![make_triangle_mesh()],
            expires_at: None,
        }
    }

    fn ensure_capacity(&mut self, tri_count: usize) {
        while tri_count > self.triangle_meshes.len() {
            self.triangle_meshes.push(make_triangle_mesh());
        }
    }

    fn update_triangle_meshes(&mut self, tris: &Vec<Point3>) {
        self.ensure_capacity(tris.len());

        let mut tri_mesh_idx = 0;
        for primitive in tris {
            if !self.is_point_valid(primitive) {
                self.add_error(
                    format!("Entity: {}, triangles[{}](1st index) - Point definition at index {} is not finite", self.entity.id(), tri_mesh_idx, 0),
                );
                continue;
            }

            let mut vert_changed = false;
            let color_changed = false;

            // note this sets the drawrange to the count
            // we set the drawrange again for indexed geometries below
            let vertices = &mut self.triangle_meshes[tri_mesh_idx].geometry.attributes.position.unwrap();
            let normal = &mut self.triangle_meshes[tri_mesh_idx].geometry.attributes.normal.unwrap();

            const single_color = self.settings.color.is_some()
                && string_to_rgba(temp_rgba, self.settings.color.as_ref().unwrap()).is_ok();

            if !single_color {
                let color = primitive.colors.get(0).unwrap_or(&missing_color);
                // only trigger on last point index
                if primitive.points.len() == 1 && color.is_equal_to(&missing_color) {
                    // will only show 1st triMeshIdx of issue -- addError prevents the adding of errors with duplicate errorIds
                    self.add_error(
                        format!("Entity: {}, triangles[{}](1st index) - Colors array should be same size as points array, showing #00ff00 instead", self.entity.id(), tri_mesh_idx),
                    );
                }

                let rgb_linear = SRGBToLinearRGBLUT(temp_rgb, color.r, color.g, color.b);

                let color_stride = self.triangle_meshes[tri_mesh_idx].geometry.attributes.color.unwrap().item_size;
                let color_offset = i * color_stride;

                const EPS = 2 / 255;
                let diff =
                    f64::abs(colors.array()[color_offset]! as f32 / 255 - rgb_linear.r) > EPS ||
                    f64::abs(colors.array()[color_offset + 1]! as f32 / 255 - rgb_linear.g) > EPS ||
                    f64::abs(colors.array()[color_offset + 2]! as f32 / 255 - rgb_linear.b) > EPS ||
                    f64::abs(colors.array()[color_offset + 3]! as f32 - color.a)! > EPS;

                if diff {
                    colors.setXYZW(i, rgb_linear.r, rgb_linear.g, rgb_linear.b, color.a);
                    color_changed = true;
                }
            }

            // covers the case where a geometry went from being defined by a single
            // color to vertex colors but there was no difference in the vertex
            // colors that already existed and the new ones we can tell this by
            // checking the current vertexColors of the material, if false ->
            // previously singleColor
            let vertex_color_changed = !self.triangle_meshes[tri_mesh_idx].material.vertex_colors &&
                !single_color && primitive.colors.len() > 0;
            if (vertex_color_changed) {
                self.triangle_meshes[tri_mesh_idx].material.vertex_colors = true;
                // need to set overall material color back or else it will blend them with the vertex colors
                self.triangle_meshes[tri_mesh_idx].material.color.set([1.0, 1.0, 1.0]);
                self.triangle_meshes[tri_mesh_idx].material.opacity = 1.0;
                // can assume that color exists since colorchanged is true
                vertices.needs_update();
            } else if (single_color) {
                let new_color = rgb_to_three_color(temp_rgb, single_color).unwrap_or(Color::WHITE);
                let material_needs_update =
                    self.triangle_meshes[tri_mesh_idx].material.vertex_colors ||
                    !self.triangle_meshes[tri_mesh_idx].material.color.equals(new_color) ||
                    self.triangle_meshes[tri_mesh_idx].material.opacity != single_color.a;
                if (material_needs_update) {
                    self.triangle_meshes[tri_mesh_idx].material.vertex_colors = false;
                    self.triangle_meshes[tri_mesh_idx].material.color.set(new_color);
                    self.triangle_meshes[tri_mesh_idx].material.opacity = single_color.a;
                    vertices.needs_update();
                }
            }

            if (self.triangle_meshes[tri_mesh_idx].material.transparent != transparent) {
                self.triangle_meshes[tri_mesh_idx].material.transparent = transparent;
                self.triangle_meshes[tri_mesh_idx].depth_write = !transparent;
                vertices.needs_update();
            }

            let indices = primitive.indices.unwrap_or(&Vec4::ZERO);
            if (indices.len() > 0) {
                if (!self.triangle_meshes[tri_mesh_idx].geometry.index.is_some()) {
                    let array = Vec2d::from_iter(indices.iter().copied()).collect::<Vec2d>().to_vec();
                    self.triangle_meshes[tri_mesh_idx]
                        .geometry
                        .index
                        .replace(Vec3d::from_iter(array).into());
                } else {
                    let array = self.triangle_meshes[tri_mesh_idx].geometry.index.as_ref().unwrap();
                    for (i, &idx) in indices.iter().enumerate() {
                        if array[i] != idx as u32 {
                            array.set(i, idx as u32);
                            vertices.needs_update();
                            break;
                        }
                    }
                }

                // this is set in `geometry.resize` to itemCount
                // which works for non-indexed geometries but not for indexed geoms
                self.triangle_meshes[tri_mesh_idx].set_draw_range(0, indices.len());
            } else {
                self.triangle_meshes[tri_mesh_idx].geometry.index = None;
            }

            self.triangle_meshes[tri_mesh_idx]
                .position
                .set(primitive.pose.position);
            self.triangle_meshes[tri_mesh_idx]
                .quaternion
                .set(primitive.pose.orientation);

            tri_mesh_idx += 1;
        }
    }

    pub fn dispose(&mut self) {
        for mesh in &mut self.triangle_meshes {
            mesh.geometry.dispose();
            mesh.material.dispose();
        }
        self.clear_errors();
    }

    pub fn update(
        &mut self,
        topic: &str,
        entity: Option<&crate::SceneEntity>,
        settings: crate::LayerSettingsEntity,
        receive_time: u64,
    ) {
        super::update(topic, entity, settings, receive_time);
        if let Some(entity) = entity {
            let lifetime_ns = super::to_nanosec(entity.lifetime);
            self.expires_at = lifetime_ns == 0u64 ? None : receive_time + lifetime_ns;
            self.update_triangle_meshes(&entity.triangles);
        }
    }

    pub fn update_settings(&mut self, settings: crate::LayerSettingsEntity) {
        self.update(settings.topic(), self.entity.as_ref(), settings, self.receive_time());
    }
}

fn make_triangle_mesh() -> Mesh<DynamicBufferGeometry, MeshStandardMaterial> {
    let mut geometry = DynamicBufferGeometry::new();
    geometry.set_attribute("position", vec![0.0; 3 * 4]);
    geometry.set_attribute("normal", vec![0.0; 3 * 4]);
    geometry.set_attribute("color", vec![0.0; 4 * 4]);

    let material = MeshStandardMaterial::new();
    material.metalness = 0.0;
    material.roughness = 1.0;
    material.flat_shading = true;
    material.side = glam::Side::DoubleSide;

    Mesh::new(geometry, material)
}

fn is_point_valid(pt: crate::Point3) -> bool {
    f64::is_finite(pt.x) && f64::is_finite(pt.y) && f64::is_finite(pt.z)
}
```