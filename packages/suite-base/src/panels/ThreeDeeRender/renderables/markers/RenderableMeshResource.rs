```rust
use crate::{Renderer, Marker, Material, ThreeDeeRender};
use std::rc::Rc;

pub struct RenderableMeshResource {
    mesh: Option<Rc<ThreeDeeRender::Group>>,
    material: Rc<Material>,
    reference_url: Option<String>,
}

impl RenderableMeshResource {
    pub fn new(
        topic: &str,
        marker: &Marker,
        receive_time: u64,
        renderer: &Renderer,
        options: Option<Options>,
    ) -> Self {
        let material = make_standard_material(marker.color);
        let reference_url = options.map(|opts| opts.reference_url);

        Self {
            mesh: None,
            material,
            reference_url,
        }
    }

    pub fn dispose(&mut self) {
        if let Some(mesh) = &self.mesh {
            mesh.dispose();
        }
        self.material.dispose();
    }

    pub fn update(
        &mut self,
        new_marker: &Marker,
        receive_time: u64,
        force_load: bool,
    ) {
        let prev_marker = self.marker.clone();
        super::super::update(self, new_marker, receive_time);
        let marker = self.marker;

        let transparent = marker.color.a < 1;
        if transparent != self.material.transparent {
            self.material.transparent = transparent;
            self.material.depth_write = !transparent;
            self.material.needs_update();
        }

        rgb_to_three_color(&mut self.material.color, marker.color);
        self.material.opacity = marker.color.a;

        if force_load || marker.mesh_resource != prev_marker.mesh_resource {
            let cur_update_id = *self.update_id.as_ref().unwrap_or(&0) + 1;
            let opts = Options { use_embedded_materials: marker.mesh_use_embedded_materials };

            self.renderer.settings.errors.remove(
                &marker.settings_path,
                "MESH_FETCH_FAILED",
                &format!("Unhandled error loading mesh from \"{}\": {}", marker.mesh_resource, "error message".to_string()),
            );

            let errors = &self.renderer.settings.errors;
            if let Some(mesh) = self.model_cache.load(&marker.mesh_resource, &options, |err| {
                errors.add(
                    &marker.settings_path,
                    "MESH_FETCH_FAILED",
                    &format!("Error loading mesh from \"{}\": {}", marker.mesh_resource, err.to_string()),
                );
            }) {
                if *self.update_id.as_ref().unwrap_or(&0) != cur_update_id {
                    return;
                }

                self.mesh = Some(mesh.clone());
                self.add_mesh(mesh);
                self.update_outline_visibility();

                // Render a new frame now that the model is loaded
                self.renderer.queue_animation_frame();
            }
        }

        self.scale.set(marker.scale.x, marker.scale.y, marker.scale.z);
    }

    fn update_outline_visibility(&self) {
        let show_outlines = super::super::get_settings().unwrap_or_default().show_outlines.unwrap_or(true);
        self.traverse(|line_segments| {
            if line_segments.name == "EDGE_LINE_SEGMENTS_NAME" {
                line_segments.visible = show_outlines;
            }
        });
    }

    async fn load_model(
        &self,
        url: &str,
        opts: Options,
    ) -> Option<Rc<ThreeDeeRender::Group>> {
        let cached_model = self.model_cache.load(&url, &opts, |err| {
            self.renderer.settings.errors.add(
                &self.marker.settings_path,
                "MESH_FETCH_FAILED",
                &format!("Error loading mesh from \"{}\": {}", url, err.to_string()),
            );
        });

        if cached_model.is_none() {
            if !self.renderer.settings.errors.has_error(&self.marker.settings_path, "MESH_FETCH_FAILED") {
                self.renderer.settings.errors.add(
                    &self.marker.settings_path,
                    "MESH_FETCH_FAILED",
                    &format!("Failed to load mesh from \"{}\"", url),
                );
            }
            return None;
        }

        let mesh = cached_model.clone();
        remove_light(&mesh);
        if !opts.use_embedded_materials {
            replace_materials(&mut mesh, &self.material);
        }

        Some(mesh)
    }
}
```