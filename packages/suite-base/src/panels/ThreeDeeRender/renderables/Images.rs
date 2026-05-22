```rust
use std::collections::{HashMap, HashSet};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use foxglove::schemas::*;
use foxglove::settings::{BaseSettings, PRECISION_DISTANCE};
use log::{debug, error};

#[derive(Clone, Debug)]
pub struct LayerSettingsImage {
    pub camera_info_topic: Option<String>;
    pub distance: f64;
    pub settings: BaseSettings,
}

#[derive(Debug)]
pub struct ImageRenderable {
    topic_name: String,
    renderer: Box<dyn Renderer>,
    user_data: ImageUserData,
    image: Option<AnyImage>,
    texture: Option<Texture2D>,
    material: Option<Material>,
    geometry: Option<GltfGeometry>,
    mesh: Option<Mesh>,
}

impl ImageRenderable {
    fn new(topic_name: String, renderer: Box<dyn Renderer>, user_data: ImageUserData) -> Self {
        ImageRenderable {
            topic_name,
            renderer,
            user_data,
            image: None,
            texture: None,
            material: None,
            geometry: None,
            mesh: None,
        }
    }

    fn set_camera_model(&mut self, camera_model: Option<CameraModel>) {
        if let Some(camera_model) = camera_model {
            debug!("Setting new camera model for topic {}", &self.topic_name);
            // Implementation to set the camera model
        } else {
            debug!("No camera model available");
            // Handle case where no camera model is set
        }
    }

    fn update(&mut self) {
        if let Some(image) = self.image.as_ref() {
            // Update renderable with new image data and texture/material/geometry/mesh
            self.renderer.update_renderable(&self.topic_name, &image);
        } else {
            debug!("No image available for topic {}", &self.topic_name);
            // Handle case where no image is set
        }
    }

    fn set_texture(&mut self, texture: Texture2D) {
        self.texture = Some(texture);
    }

    fn set_material(&mut self, material: Material) {
        self.material = Some(material);
    }

    fn set_geometry(&mut self, geometry: GltfGeometry) {
        self.geometry = Some(geometry);
    }

    fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = Some(mesh);
    }
}

struct Renderer;

impl Renderer {
    fn update_renderable(&self, topic_name: &str, image: &AnyImage) {
        // Implementation to update renderable with new data
        debug!("Updating renderable for topic {}: {:?}", topic_name, image);
    }
}

fn main() {
    // Example usage of ImageRenderable and Renderer
}
```