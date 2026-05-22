```rust
use std::vec2;

use three::{geometry::CylinderGeometry, material::StandardMaterial, LineSegments, Box3};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct RenderableMarker {
    topic: String,
    marker: Marker,
    receive_time: Option<bigint>,
    renderer: IRenderer,
}

impl RenderableMarker {
    pub fn new(topic: String, marker: Marker, receive_time: Option<bigint>, renderer: IRenderer) -> Self {
        let material = make_standard_material(&marker.color);
        let cylinder_geometry = renderer.shared_geometry.get_geometry(
            format!("RenderableCylinder-cylinder-{}", renderer.max_lod),
            || create_geometry(renderer.max_lod),
        );
        self.#mesh = ThreeMesh::new(cylinder_geometry, material);
        self.#mesh.cast_shadow = true;
        self.#mesh.receive_shadow = true;
        self.add(&self.#mesh);

        let edges_geometry = renderer.shared_geometry.get_geometry(
            format!("RenderableCylinder-edges-{}", renderer.max_lod),
            || create_edges_geometry(&cylinder_geometry),
        );
        self.#outline = ThreeLineSegments::new(edges_geometry, renderer.outline_material);
        self.#outline.userData.picking = false;
        self.add(&self.#outline);

        self.update(&marker, receive_time);
    }

    pub fn dispose(&mut self) {
        self.#mesh.material.dispose();
    }

    pub fn update(&mut self, new_marker: &Marker, receive_time: Option<bigint>) {
        super::update(self, new_marker, receive_time);
        let marker = self.marker;

        let transparent = marker.color.a < 1;
        if transparent != self.#mesh.material.transparent {
            self.#mesh.material.transparent = transparent;
            self.#mesh.material.depth_write = !transparent;
            self.#mesh.material.needs_update();
        }

        self.#outline.visible = self.get_settings()?.show_outlines.unwrap_or(true);

        rgb_to_three_color(&mut self.#mesh.material.color, &marker.color);
        self.#mesh.material.opacity = marker.color.a;

        self.scale.set(marker.scale.x, marker.scale.y, marker.scale.z);
    }
}

fn create_geometry(lod: DetailLevel) -> CylinderGeometry {
    let subdivisions = cylinder_subdivisions(lod);
    let cylinder_geometry = CylinderGeometry::new(0.5, 0.5, 1, subdivisions);
    cylinder_geometry.rotate_x(std::f32::consts::PI / 2.0); // Make the cylinder geometry stand upright
    cylinder_geometry.compute_bounding_sphere();
    cylinder_geometry
}

fn create_edges_geometry(geometry: &CylinderGeometry) -> LineSegments {
    let cylinder_edges_geometry = LineSegments::new(geometry, renderer.outline_material);
    cylinder_edges_geometry.compute_bounding_sphere();
    cylinder_edges_geometry
}
```