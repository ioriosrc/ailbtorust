```rust
use three::geometry::SphereGeometry;
use three::material::StandardMaterial;
use three::DynamicInstancedMesh;
use crate::{Renderer, Marker};
use std::time::SystemTime;

struct RenderableSphereList {
    mesh: DynamicInstancedMesh<SphereGeometry, StandardMaterial>,
}

impl RenderableSphereList {
    pub fn new(
        topic: &str,
        marker: &Marker,
        receive_time: SystemTime | Option<SystemTime>,
        renderer: &Renderer,
    ) -> Self {
        let geometry = renderer.shared_geometry.get_geometry(
            "RenderableSphere",
            || Box::new(SphereGeometry::new(renderer.max_lod)),
        );
        let material = StandardMaterial::new(marker);

        let mut instanced_mesh = DynamicInstancedMesh::new(geometry, material, marker.points.len());
        instanced_mesh.cast_shadow = true;
        instanced_mesh.receive_shadow = true;

        Self {
            mesh: instanced_mesh,
        }
    }

    pub fn dispose(&mut self) {
        self.mesh.material.dispose();
    }

    pub fn update(
        &mut self,
        new_marker: &Marker,
        receive_time: SystemTime | Option<SystemTime>,
    ) {
        let prev_marker = self.mesh.material.user_data.get::<Option<&Marker>>(true);
        super::update(&mut self.mesh, new_marker, receive_time);

        if *prev_marker != Some(new_marker) {
            self.mesh.material.transparent = new_marker.has_transparency();
            self.mesh.material.depth_write = !new_marker.has_transparency();
            self.mesh.material.needs_update = true;
        }

        let marker = &self.mesh.material.user_data.get::<&Marker>(true).unwrap();

        self.mesh.set(
            marker.points,
            marker.scale,
            marker.colors,
            marker.color,
        );
    }
}
```