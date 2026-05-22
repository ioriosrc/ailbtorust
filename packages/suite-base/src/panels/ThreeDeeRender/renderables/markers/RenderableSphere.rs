```rust
use three::{Box3, Geometry, Material, Mesh, MeshBasicMaterial, SphereGeometry};

struct RenderableSphere {
    mesh: Mesh<SphereGeometry, Box<dyn Material>>,
}

impl RenderableSphere {
    fn new(
        topic: String,
        marker: Marker,
        receive_time: Option<bigint>,
        renderer: IRenderer,
    ) -> Self {
        let geometry = Renderer::shared_geometry()
            .get_geometry(&format!("{}-{}", Self::name(), renderer.max_lod), || create_geometry(renderer.max_lod));
        let material = make_standard_material(&marker.color);
        let mesh = Mesh::new(geometry, material);

        mesh.cast_shadow = true;
        mesh.receive_shadow = true;
        self.mesh = mesh;

        self.update(&marker, receive_time)
    }

    fn dispose(&mut self) {
        self.mesh.material.dispose();
    }

    fn update(&mut self, new_marker: &Marker, receive_time: Option<bigint>) -> bool {
        super::update(self, new_marker, receive_time);
        let marker = self.data().marker;

        let transparent = marker.color.a < 1.0;
        if transparent != self.mesh.material.is_transparent() {
            self.mesh.material.set_transparent(transparent);
            self.mesh.material.depth_write = !transparent;
            self.mesh.material.needs_update();
        }

        rgb_to_three_color(&mut self.mesh.material.color, &marker.color);
        self.mesh.material.opacity = marker.color.a;

        self.scale.set(marker.scale.x, marker.scale.y, marker.scale.z);

        true
    }
}

fn create_geometry(lod: DetailLevel) -> SphereGeometry {
    let subdivisions = sphere_subdivisions(lod);
    SphereGeometry::new(0.5, subdivisions, subdivisions)
}
```