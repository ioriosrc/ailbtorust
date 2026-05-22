```rust
use super::{LayerSettingsEntity, SceneEntities};
use crate::renderer::{AnyRendererSubscription, IRenderer};
use bevy::{
    prelude::*,
    render::primitives::{ArrowPrimitive, CubePrimitive, CylinderPrimitive, LinePrimitive, LineType, ModelPrimitive, SpherePrimitive, TextPrimitive, TriangleListPrimitive},
};

const SCENE_ENTITIES_DEFAULT_SETTINGS: LayerSettingsEntity = {
    show_outlines: true,
    visible: false,
    color: None,
    selected_id_variable: None,
};

pub struct FoxgloveSceneEntities;

impl SceneExtension<TopicEntities> for FoxgloveSceneEntities {
    fn new(renderer: IRenderer) -> Self {
        Self
    }

    fn subscriptions(&self) -> Vec<AnyRendererSubscription> {
        vec![AnyRendererSubscription::new(
            "schema",
            SCENE_UPDATE_DATATYPES.to_vec(),
            move |message_event| self.handle_scene_update(message_event),
        )]
    }

    fn settings_nodes(&self) -> Vec<SettingsTreeEntry> {
        let config_topics = renderer.config.topics;
        let entries: Vec<SettingsTreeEntry> = config_topics
            .iter()
            .filter_map(|(topic, _)| if topic_is_convertible_to_schema(topic, SCENE_UPDATE_DATATYPES) { Some(topic.clone()) } else { None })
            .map(|topic| {
                let user_settings = renderer.config.topics.get(topic).unwrap_or_default();
                SettingsTreeNodeWithActionHandler {
                    label: topic,
                    icon: "Shapes",
                    order: topic.to_lowercase(),
                    fields: vec![
                        (
                            String::from("color"),
                            SettingsTreeEntryFieldInput {
                                input: "rgba",
                                value: user_settings.color,
                            },
                        ),
                        (
                            String::from("show_outlines"),
                            SettingsTreeEntryFieldInput {
                                input: "boolean",
                                value: user_settings.show_outlines,
                            },
                        ),
                        (
                            String::from("selected_id_variable"),
                            SettingsTreeEntryFieldInput {
                                input: "string",
                                help: "When selecting a SceneEntity, this global variable will be set to the entity ID",
                                value: user_settings.selected_id_variable.clone(),
                                placeholder: SELECTED_ID_VARIABLE,
                            },
                        ),
                    ],
                    visible: user_settings.visible,
                    handler: self.handle_settings_action,
                }
            })
            .collect();
        entries
    }

    fn start_frame(
        &mut self,
        _current_time: f64,
        _render_frame_id: String,
        _fixed_frame_id: String,
    ) {
        // Don't use SceneExtension#startFrame() because our renderables represent one topic each with
        // many entities. Instead, call start_frame on each renderable
        for (topic, renderable) in self.renderables.iter_mut() {
            renderable.start_frame(_current_time, _render_frame_id, _fixed_frame_id);
        }
    }

    fn set_color_scheme(
        &mut self,
        color_scheme: &str,
        _background_color: Option<Color>,
    ) {
        for (topic, renderable) in self.renderables.iter_mut() {
            renderable.set_color_scheme(color_scheme);
        }
    }

    fn handle_settings_action(&mut self, action: SettingsTreeAction) {
        if action.action == "update" && action.payload.path.len() == 3 {
            let topic_name = action.payload.path[1].clone();
            let user_settings = match renderer.config.topics.get(&topic_name) {
                Some(settings) => settings,
                None => &DEFAULT_SETTINGS,
            };
            self.save_setting(action.payload.path.clone(), action.payload.value);
            // Update the TopicEntities settings
            let topic_entities = self.renderables.entry(topic_name).or_default();
            topic_entities.user_settings = { ..DEFAULT_SETTINGS, ..user_settings };
            topic_entities.update_settings();
        }
    }

    fn handle_scene_update(&mut self, message_event: PartialMessageEvent<SceneUpdate>) {
        let topic = message_event.topic;
        let scene_updates = message_event.message;

        for deletion_msg in scene_updates.deletions.as_ref().unwrap_or_default() {
            if let Some(deletion) = normalize_scene_entity_deletion(deletion_msg) {
                self.get_topic_entities(topic).delete_entities(deletion);
            }
        }

        for entity_msg in scene_updates.entities.as_ref().unwrap_or_default() {
            if let Some(entity) = normalize_scene_entity(entity_msg) {
                self.get_topic_entities(topic).add_or_update_entity(
                    entity,
                    message_event.receive_time,
                );
            }
        }
    }

    fn get_topic_entities(&mut self, topic: String) -> &mut TopicEntities {
        let renderable = self.renderables.entry(topic).or_default();
        if renderable.is_none() {
            let user_settings = renderer.config.topics.get(&topic).unwrap_or_default();
            let settings_path = vec!["topics", &topic];
            let receive_time = -1.0;
            let message_time = -1.0;
            let frame_id = "";
            let pose = make_pose();
            let settings = { ..DEFAULT_SETTINGS, ..user_settings };
            renderable.insert(
                topic.clone(),
                TopicEntities::new(renderer.clone(), &settings_path),
            );
        }
        renderable.as_mut().unwrap()
    }

    fn dispose(&mut self) {
        for (_, renderable) in self.renderables.iter_mut() {
            renderable.dispose();
        }
        self.primitive_pool.clear();
    }
}

fn normalize_scene_entity(entity: PartialMessage<SceneEntity>) -> SceneEntity {
    let pose = entity.pose.unwrap_or_default();
    let size = entity.size.unwrap_or_default();
    let color = entity.color.unwrap_or_default();

    SceneEntity {
        timestamp: entity.timestamp,
        frame_id: entity.frame_id,
        id: entity.id,
        lifetime: entity.lifetime,
        frame_locked: entity.frame_locked,
        metadata: entity.metadata.map(|kv| ({ key: kv.key, value: kv.value })).collect(),
        arrows: entity.arrows.unwrap_or_default().into_iter().map(normalize_arrow_primitive).collect(),
        cubes: entity.cubes.unwrap_or_default().into_iter().map(normalize_cube_primitive).collect(),
        spheres: entity.spheres.unwrap_or_default().into_iter().map(normalize_sphere_primitive).collect(),
        cylinders: entity.cylinders.unwrap_or_default().into_iter().map(normalize_cylinder_primitive).collect(),
        lines: entity.lines.unwrap_or_default().into_iter().map(normalize_line_primitive).collect(),
        triangles: entity.triangles.unwrap_or_default().into_iter().map(normalize_triangle_list_primitive).collect(),
        texts: entity.texts.unwrap_or_default().into_iter().map(normalize_text_primitive).collect(),
        models: entity.models.unwrap_or_default().into_iter().map(normalize_model_primitive).collect(),
    }
}

fn normalize_scene_entity_deletion(entity: PartialMessage<SceneEntityDeletion>) -> SceneEntityDeletion {
    let pose = entity.pose.unwrap_or_default();
    let id = entity.id.unwrap_or_default();

    SceneEntityDeletion {
        timestamp: entity.timestamp,
        type: entity.type.unwrap_or_default(),
        id,
    }
}

fn normalize_arrow_primitive(
    arrow: PartialMessage<ArrowPrimitive> | Option<ArrowPrimitive>,
) -> ArrowPrimitive {
    match arrow {
        Some(arrow) => ArrowPrimitive {
            pose: arrow.pose.unwrap_or_default(),
            shaft_length: arrow.shaft_length.unwrap_or_default(),
            shaft_diameter: arrow.shaft_diameter.unwrap_or_default(),
            head_length: arrow.head_length.unwrap_or_default(),
            head_diameter: arrow.head_diameter.unwrap_or_default(),
            color: arrow.color.unwrap_or_default(),
        },
        None => ArrowPrimitive {
            pose: Default::default(),
            shaft_length: 0.1,
            shaft_diameter: 0.05,
            head_length: 0.2,
            head_diameter: 0.1,
            color: Color::rgb(1.0, 0.0, 0.0),
        },
    }
}

fn normalize_cube_primitive(
    cube: PartialMessage<CubePrimitive> | Option<CubePrimitive>,
) -> CubePrimitive {
    match cube {
        Some(cube) => CubePrimitive {
            pose: cube.pose.unwrap_or_default(),
            scale: cube.scale.unwrap_or_default(),
            color: cube.color.unwrap_or_default(),
        },
        None => CubePrimitive {
            pose: Default::default(),
            scale: Vec3::splat(1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        },
    }
}

fn normalize_sphere_primitive(
    sphere: PartialMessage<SpherePrimitive> | Option<SpherePrimitive>,
) -> SpherePrimitive {
    match sphere {
        Some(sphere) => SpherePrimitive {
            pose: sphere.pose.unwrap_or_default(),
            scale: sphere.scale.unwrap_or_default(),
            color: sphere.color.unwrap_or_default(),
        },
        None => SpherePrimitive {
            pose: Default::default(),
            scale: Vec3::splat(1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
        },
    }
}

fn normalize_cylinder_primitive(
    cylinder: PartialMessage<CylinderPrimitive> | Option<CylinderPrimitive>,
) -> CylinderPrimitive {
    match cylinder {
        Some(cylinder) => CylinderPrimitive {
            pose: cylinder.pose.unwrap_or_default(),
            scale: cylinder.scale.unwrap_or_default(),
            color: cylinder.color.unwrap_or_default(),
            bottom_radius: cylinder.bottom_radius.unwrap_or_default(),
            top_radius: cylinder.top_radius.unwrap_or_default(),
        },
        None => CylinderPrimitive {
            pose: Default::default(),
            scale: Vec3::splat(1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            bottom_radius: 0.5,
            top_radius: 0.5,
        },
    }
}

fn normalize_line_primitive(
    line: PartialMessage<LinePrimitive> | Option<LinePrimitive>,
) -> LinePrimitive {
    match line {
        Some(line) => LinePrimitive {
            type_: line.type_.unwrap_or_default(),
            pose: line.pose.unwrap_or_default(),
            thickness: line.thickness.unwrap_or_default(),
            scale_invariant: line.scale_invariant.unwrap_or_default(),
            points: line.points.unwrap_or_default().into_iter().map(normalize_vector3).collect(),
            color: line.color.unwrap_or_default(),
            colors: line.colors.unwrap_or_default().into_iter().map(normalize_color_rgba).collect(),
            indices: line.indices.unwrap_or_default().into_iter().map(|idx| idx as usize).collect(),
        },
        None => LinePrimitive {
            type_: LineType::LINE_STRIP,
            pose: Default::default(),
            thickness: 0.1,
            scale_invariant: false,
            points: vec![Vec3::splat(0.0), Vec3::splat(1.0)],
            color: Color::rgb(1.0, 0.0, 0.0),
        },
    }
}

fn normalize_triangle_list_primitive(
    triangles: PartialMessage<TriangleListPrimitive> | Option<TriangleListPrimitive>,
) -> TriangleListPrimitive {
    match triangles {
        Some(triangles) => TriangleListPrimitive {
            pose: triangles.pose.unwrap_or_default(),
            points: triangles.points.unwrap_or_default().into_iter().map(normalize_vector3).collect(),
            color: triangles.color.unwrap_or_default(),
            colors: triangles.colors.unwrap_or_default().into_iter().map(normalize_color_rgba).collect(),
            indices: triangles.indices.unwrap_or_default().into_iter().map(|idx| idx as usize).collect(),
        },
        None => TriangleListPrimitive {
            pose: Default::default(),
            points: vec![Vec3::splat(0.0), Vec3::splat(1.0), Vec3::splat(2.0)],
            color: Color::rgb(1.0, 1.0, 1.0),
            colors: vec![Color::rgb(1.0, 0.0, 0.0), Color::rgb(0.0, 1.0, 0.0)],
            indices: vec![0, 2, 1],
        },
    }
}

fn normalize_text_primitive(
    text: PartialMessage<TextPrimitive> | Option<TextPrimitive>,
) -> TextPrimitive {
    match text {
        Some(text) => TextPrimitive {
            pose: text.pose.unwrap_or_default(),
            billboard: text.billboard.unwrap_or_default(),
            font_size: text.font_size.unwrap_or_default().unwrap_or(16.0),
            scale_invariant: text.scale_invariant.unwrap_or(false),
            color: text.color.unwrap_or_default(),
            text: text.text.unwrap_or(""),
        },
        None => TextPrimitive {
            pose: Default::default(),
            billboard: true,
            font_size: 8.0,
            scale_invariant: false,
            color: Color::rgb(1.0, 0.0, 0.0),
            text: "Hello, World!",
        },
    }
}

fn normalize_model_primitive(
    model: PartialMessage<ModelPrimitive> | Option<ModelPrimitive>,
) -> ModelPrimitive {
    match model {
        Some(model) => ModelPrimitive {
            pose: model.pose.unwrap_or_default(),
            scale: model.scale.unwrap_or_default(),
            color: model.color.unwrap_or_default(),
            override_color: model.override_color.unwrap_or(false),
            url: model.url.unwrap_or(""),
            media_type: model.media_type.unwrap_or(""),
            data: model.data.unwrap_or_default(),
        },
        None => ModelPrimitive {
            pose: Default::default(),
            scale: Vec3::splat(1.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            override_color: false,
            url: "",
            media_type: "text/plain",
            data: vec![],
        },
    }
}

fn normalize_vector3(vec3: PartialMessage<Vec3> | Option<Vec3>) -> Vec3 {
    match vec3 {
        Some(vec3) => vec3,
        None => Vec3::splat(0.0),
    }
}

fn normalize_color_rgba(color: PartialMessage<Color> | Option<Color>) -> Color {
    match color {
        Some(color) => color,
        None => Color::rgb(1.0, 0.0, 0.0),
    }
}
```