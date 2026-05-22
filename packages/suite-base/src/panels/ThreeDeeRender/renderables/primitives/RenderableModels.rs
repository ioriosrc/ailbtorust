```rust
use std::collections::{HashMap};
use three::{
    BufferGeometry, Geometry, Material, Mesh, MeshStandardMaterial, Object3D, Raycaster, Vector3,
};

struct RenderableModel {
    material: Option<MeshStandardMaterial>,
    model: Object3D,
    cached_model: LoadedModel,
    primitive: ModelPrimitive,
}

impl RenderableModel {
    fn new(cached_model: LoadedModel) -> Self {
        let material = Material::new().with_metalness(0.0).with_roughness(1.0);
        let model = Object3D::from(cached_model.clone(true));
        remove_light(&model);

        RenderableModel {
            material,
            model,
            cached_model,
            primitive,
        }
    }

    fn update(&mut self, primitive: ModelPrimitive) {
        if let Some(override_color) = &self.primitive.override_color {
            if !self.material.is_none() {
                let color = rgb_to_three_color(*override_color);
                self.material.as_ref().color = color;
                self.material.as_ref().opacity = *override_color.a;
                self.material.as_ref().transparent = *override_color.a < 1.0;
                self.material.as_ref().depth_write = !*override_color.a < 1.0;
            } else {
                let material = MeshStandardMaterial::new();
                rgb_to_three_color(override_color, &mut material.color);
                material.opacity = override_color.a;
                material.transparent = *override_color.a < 1.0;
                material.depth_write = !*override_color.a < 1.0;

                self.material = Some(material);
                self.model = Object3D::from(self.cached_model.clone(true));
            }
        } else if let Some(ref material) = &self.material {
            if let Some(cached_model) = &self.cached_model {
                self.model = clone_and_prepare_model(&cached_model).unwrap();
                material.dispose();
                self.cached_model.dispose();
            }
        }

        self.model.scale.set(primitive.scale.x, primitive.scale.y, primitive.scale.z);
        self.model.position.set(
            primitive.pose.position.x,
            primitive.pose.position.y,
            primitive.pose.position.z,
        );
        self.model.quaternion.set(
            primitive.pose.orientation.x,
            primitive.pose.orientation.y,
            primitive.pose.orientation.z,
            primitive.pose.orientation.w,
        );
    }

    fn dispose(&mut self) {
        self.material.as_mut().map(|m| m.dispose());
        self.model.clear();
        self.cached_model.dispose();
    }
}

fn clone_and_prepare_model(cached_model: &LoadedModel) -> Option<Mesh> {
    let model = cached_model.clone(true);
    remove_light(&model);

    Some(model)
}

fn data_primitives_match(model1: &ModelPrimitive, model2: &ModelPrimitive) -> bool {
    model1.media_type == model2.media_type && byte_arrays_equal(&model1.data, &model2.data)
}

fn url_primitives_match(model1: &ModelPrimitive, model2: &ModelPrimitive) -> bool {
    model1.url == model2.url && model1.media_type == model2.media_type
}
```