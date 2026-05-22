```rust
use web_sys::{Blob, Request, Response};
use wasm_bindgen::JsCast;

struct ModelCacheOptions {
    edge_material: Box<dyn THREE.Material>,
    ignore_collada_up_axis: bool,
    mesh_up_axis: MeshUpAxis,
    fetch_asset: BuiltinPanelExtensionContext<'a>::unstable_fetch_asset,
}

enum MeshUpAxis {
    Y_UP,
    Z_UP,
}

type LoadedModel = Three::Group;

pub struct ModelCache {
    edge_material: Box<dyn THREE.Material>,
    fetch_asset: BuiltinPanelExtensionContext<'a>::unstable_fetch_asset,
    draco_loader: Option<DRACOLoader>,
    collada_texture_object_urls: std::collections::HashMap<String, String>,
}

impl ModelCache {
    pub fn new(options: ModelCacheOptions) -> Self {
        Self {
            edge_material: options.edge_material,
            fetch_asset: options.fetch_asset,
            draco_loader: None,
            collada_texture_object_urls: std::collections::HashMap::new(),
        }
    }

    pub async fn dispose(&mut self) {
        if let Some(draco_loader) = &mut self.draco_loader {
            draco_loader.dispose();
        }
        for (_, object_url) in &self.collada_texture_object_urls {
            url.revoke_object_url(object_url);
        }
    }

    pub fn load_model(&mut self, url: String) -> Result<LoadedModel, JsValue> {
        let request = Request::new(&url).unwrap();
        self.fetch_asset(request, false).then(|response| {
            match response.status() {
                200 => Ok(response.blob().map_err(|e| e.into())?),
                _ => Err(JsValue::from_str("Failed to load model")),
            }
        })
        .and_then(|blob| {
            let reader = js_sys::BlobReader::new();
            reader.read_text(&blob).then(|text| async move {
                Ok(text.map_err(|e| e.into())?)
            })
        })
        .await?
        .parse::<THREE::Group>()
        .map_err(|e| JsValue::from_str(e.to_string().as_ref()))
    }

    pub fn add_edges(&mut self, model: &Three::Group) -> LoadedModel {
        let mut edges_to_add = Vec::new();

        model.traverse(|child| {
            if child.is_instance_of::<THREE::Mesh>() {
                // Enable shadows for all meshes
                child.cast_shadow(true);
                child.receive_shadow(true);

                // Draw edges for all meshes
                let geometry = THREE::EdgesGeometry::new(child.geometry(), 40);
                let line = Three::LineSegments::new(geometry, &self.edge_material);
                line.name = "edges".to_string();
                edges_to_add.push((line, child));
            }
        });

        for (line, parent) in edges_to_add {
            parent.add(&line);
        }

        model
    }

    pub fn fix_dae_materials(&mut self, model: &Three::Group) -> LoadedModel {
        model.traverse(|child| {
            if child.is_instance_of::<THREE::Mesh>() {
                if let Some(material) = child.material() {
                    if material instanceof THREE::MeshLambertMaterial {
                        let material = to_standard(&material);
                        child.material = material;
                    } else if material instanceof THREE::MeshStandardMaterial {
                        material.dithering = true;
                    }
                }
            }
        });

        model
    }

    pub fn fix_obj_materials(&mut self, model: &Three::Group) -> LoadedModel {
        model.traverse(|child| {
            if child.is_instance_of::<THREE::Mesh>() {
                if let Some(material) = child.material() {
                    if material instanceof THREE::MeshPhongMaterial {
                        let material = to_standard(&material);
                        child.material = material;
                    } else if material instanceof THREE::MeshStandardMaterial {
                        material.metalness = 0.0;
                        material.roughness = 1.0;
                        material.dithering = true;
                    }
                }
            }
        });

        model
    }

    fn to_standard(material: &THREE::MeshPhongMaterial) -> THREE::MeshStandardMaterial {
        let standard = THREE::MeshStandardMaterial::new("standard".to_string());
        let shininess = (material as Box<dyn THREE.Material>).shininess.unwrap_or(0.0);

        // MeshStandardMaterial.copy() assumes the normalScale property exists, which
        // is true for other MeshStandardMaterials or MeshPhongMaterial but not
        // MeshLambertMaterial. Default initialize this property if needed so the
        // `standard.copy(material)` below succeeds
        let maybe_phong = material as Box<dyn THREE::MeshPhongMaterial>;
        maybe_phong.normal_scale.unwrap_or_default();

        standard.copy(material);
        standard.metalness = 0.0;
        standard.roughness = 1.0 - shininess / 100.0;
        standard.dithering = true;
        standard
    }

    fn rewrite_url(url: String) -> String {
        if url.starts_with("package://") && url.ends_with(".tiff?") {
            return format!("x-foxglove-converted-tiff://{url}");
        }
        url
    }

    fn unrewrite_url(url: String) -> String {
        if url.starts_with("x-foxglove-converted-tiff://") {
            return "package://".to_string();
        }
        url
    }

    fn base_url(url: &str) -> &str {
        let mut parts = url.split('/');
        parts.next(); // Skip the protocol part
        parts.next().unwrap_or("")
    }
}
```