```rust
use std::rc::Rc;

use crate::{
    color::{rgb_to_three_color, RGBColor},
    geometry::{create_geometry, create_edges_geometry},
    marker::Marker,
};

#[derive(Debug)]
pub struct RenderableCube {
    mesh: Rc<ThreeMesh>,
    outline: Rc<ThreeLineSegments>,
}

impl RenderableCube {
    pub fn new(
        topic: &str,
        marker: &Marker,
        receive_time: &Option<u64>,
        renderer: &Renderer,
    ) -> Self {
        // Cube mesh
        let cube_geometry = renderer.shared_geometry.get_geometry(topic, || create_geometry());
        let cube_edges_geometry = renderer.shared_geometry.get_geometry(
            topic,
            || create_edges_geometry(cube_geometry.clone()),
        );
        let mesh = Rc::new(ThreeMesh::new(
            &cube_geometry,
            &make_standard_material(marker.color),
        ));
        let outline = Rc::new(ThreeLineSegments::new(
            &cube_edges_geometry,
            renderer.outline_material.clone(),
        ));

        mesh.set_cast_shadow(true);
        mesh.set_receive_shadow(true);
        mesh.add(&outline);

        Self {
            mesh,
            outline,
        }
    }

    pub fn dispose(&self) {
        self.mesh.material.dispose();
    }

    pub fn update(&mut self, new_marker: &Marker, receive_time: &Option<u64>) {
        super::RenderableMarker::update(self, new_marker, receive_time);
        let marker = self.data.marker;

        let transparent = marker.color.a < 1.0;
        if transparent != self.mesh.material.transparent {
            self.mesh.material.transparent = transparent;
            self.mesh.material.depth_write = !transparent;
            self.mesh.material.needs_update = true;
        }

        self.outline.visible = self.get_settings()?.show_outlines.unwrap_or(true);

        rgb_to_three_color(&mut self.mesh.material.color, marker.color);
        self.mesh.material.opacity = marker.color.a;

        self.scale.set(marker.scale.x, marker.scale.y, marker.scale.z);
    }
}

fn create_geometry() -> Rc<ThreeGeometry> {
    let cube_geometry = Rc::new(ThreeBoxGeometry::new());
    cube_geometry.compute_bounding_sphere();
    cube_geometry
}

fn create_edges_geometry(cube_geometry: Rc<ThreeBoxGeometry>) -> Rc<ThreeEdgesGeometry> {
    let cube_edges_geometry = Rc::new(ThreeEdgesGeometry::new(
        &cube_geometry,
        40.0,
    ));
    cube_edges_geometry.compute_bounding_sphere();
    cube_edges_geometry
}
```