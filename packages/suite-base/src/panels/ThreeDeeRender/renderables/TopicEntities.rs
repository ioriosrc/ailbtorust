```rust
use std::collections::{HashMap, HashSet};

// Define the necessary structures and types from the provided codebase

#[derive(Debug)]
struct EntityTopicUserData {
    topic: String,
    settings: LayerSettingsEntity,
}

#[derive(Debug)]
enum EntityRenderables {
    Cubes(CubeRenderable),
    Models(ModelRenderable),
    Lines(LineRenderable),
    Cylinders(CylinderRenderable),
    Arrows(ArrowRenderable),
    Spheres(SphereRenderable),
    Texts(TextRenderable),
    Triangles(TriangleRenderable),
}

const ALL_PRIMITIVE_TYPES: &[PrimitiveType] = &[
    PrimitiveType::CUBES,
    PrimitiveType::MODELS,
    PrimitiveType::LINES,
    PrimitiveType::CYLINDERS,
    PrimitiveType::ARROWS,
    PrimitiveType::SPHERES,
    PrimitiveType::TEXTS,
    PrimitiveType::TRIANGLES,
];

#[derive(Debug)]
struct TopicEntities {
    name: String,
    primitive_pool: PrimitivePool,
    renderer: IRenderer,
    userData: EntityTopicUserData,
}

impl TopicEntities {
    fn new(
        name: &str,
        primitive_pool: PrimitivePool,
        renderer: IRenderer,
        userData: EntityTopicUserData,
    ) -> Self {
        TopicEntities {
            name,
            primitive_pool,
            renderer,
            userData,
        }
    }

    fn dispose(&mut self) {
        for renderables in self.#renderablesById.values_mut() {
            renderables.clear();
        }
        self.#renderablesById.clear();
    }

    fn update_settings(&mut self) {
        // Updates each individual primitive renderable using the current topic settings
        for (renderables, _) in &self.#renderables_by_id {
            for renderable in renderables.values_mut() {
                renderable.update_settings(self.userData.settings);
            }
        }
    }

    fn set_color_scheme(&mut self, color_scheme: "dark" | "light") {
        for (_, renderables) in &self.#renderables_by_id {
            for renderable in renderables.values_mut() {
                renderable.set_color_scheme(color_scheme);
            }
        }
    }

    fn start_frame(
        &mut self,
        current_time: i128,
        render_frame_id: String,
        fixed_frame_id: String,
    ) {
        self.visible = self.userData.settings.visible;
        if !self.visible {
            self.renderer.settings.errors.clear_topic(&self.topic);
            return;
        }

        for (_, renderables) in &self.#renderables_by_id {
            for renderable in renderables.values_mut() {
                let entity = renderable.get_entity();
                if entity.is_none() {
                    continue;
                }

                // Check if this entity has expired
                let expires_at = renderable.get_expires_at();
                if expires_at != None && current_time > expires_at {
                    self.delete_entity(entity.id());
                    break;
                }

                let frame_id = self.renderer.normalize_frame_id(entity.frame_id());
                let src_time = entity.frame_locked() ? current_time : i128::from(to_nanoseconds(entity.timestamp()));
                let updated = update_pose(
                    renderable,
                    self.renderer.transform_tree(),
                    render_frame_id,
                    fixed_frame_id,
                    frame_id,
                    current_time,
                    src_time,
                );
                renderable.set_visible(updated);
                let topic = &self.userData.topic;
                if !updated {
                    let message = missing_transform_message(&render_frame_id, &fixed_frame_id, &frame_id);
                    self.renderer.settings.errors.add_to_topic(topic, MISSING_TRANSFORM, message);
                } else {
                    self.renderer.settings.errors.remove_from_topic(topic, MISSING_TRANSFORM);
                }
            }
        }
    }

    fn add_or_update_entity(&mut self, entity: SceneEntity, receive_time: i128) {
        let id = &entity.id();
        let renderables = self.#renderables_by_id.entry(id.to_string()).or_default();

        for primitive_type in ALL_PRIMITIVE_TYPES {
            if !entity[PRIMITIVE_KEYS[*primitive_type as usize]].is_empty() {
                if !renderables.contains_key(primitive_type) {
                    let renderable = self.primitive_pool.acquire(*primitive_type);
                    renderable.name(&format!("{}:{}", id, primitive_type));
                    renderable.set_user_data_entity_id(id.to_string());
                    renderable.set_user_data_settings_path(&self.userData.settings_path);
                    renderable.set_color_scheme(&self.renderer.color_scheme);
                    // @ts-expect-error TS doesn't know that renderable matches primitive_type
                    renderables.insert(primitive_type.clone(), renderable);
                }
                renderable.update(&self.userData.topic, &entity, &self.userData.settings, receive_time);
            } else if let Some(renderable) = renderables.get_mut(*primitive_type) {
                self.renderer.remove(renderable);
                self.primitive_pool.release(*primitive_type, renderable);
                renderables.remove(primitive_type);
            }
        }
    }

    fn delete_entities(&mut self, deletion: SceneEntityDeletion) {
        match deletion.type_() {
            SceneEntityDeletionType::MATCHING_ID => self.delete_entity(deletion.id()),
            SceneEntityDeletionType::ALL => self.delete_all_entities(),
            _ => {
                self.renderer.settings.errors.add_to_topic(
                    &self.topic,
                    INVALID_DELETION_TYPE,
                    format!("Invalid deletion type {:?}", deletion.type_()),
                );
            }
        }
    }

    fn delete_entity(&mut self, id: String) {
        if let Some(renderables) = self.#renderables_by_id.get_mut(id.as_str()) {
            for renderable in renderables.values_mut() {
                self.renderer.remove(renderable);
                self.primitive_pool.release(*renderable.primitive_type(), renderable);
                renderables.remove(&renderable.primitive_type());
            }
        }
    }

    fn delete_all_entities(&mut self) {
        for id in &self.#renderables_by_id.keys().cloned() {
            self.delete_entity(id.to_string());
        }
        self.#renderables_by_id.clear();
    }
}
```