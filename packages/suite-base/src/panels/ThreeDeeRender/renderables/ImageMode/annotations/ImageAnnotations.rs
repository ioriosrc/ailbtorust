```rust
use super::{AnyRendererSubscription, ImageModeConfig};
use crate::foxglove::{
    ImageAnnotations as FoxgloveImageAnnotations,
    RosImageMarker as RosImageMarker,
    RosImageMarkerArray as RosImageMarkerArray,
};
use crate::renderer::{IModel, ICameraModel, Topic, MessageEvent, SettingsTreeAction, Annotation};
use crate::settings_manager::LabelPool;
use crate::types::AnyRendererSubscription as AnyRendererSubscriptionRust;
use crate::topics::topic_prefix_matching::sort_prefix_matchesToFront;
use crate::suite_base::{
    panels::{ThreeDeeRender, LayerErrors},
    MessageRenderState,
};
use crate::suite_base::{messages::ImageMarkerArray as RosImageMarkerArray};
use crate::foxglove::annotations_data_types::{IMAGE_ANNOTATIONS_DATATYPES, IMAGE_MARKER_ARRAY_DATATYPES, IMAGE_MARKER_DATATYPES};
use crate::topic_is_convertible_to_schema;
use std::collections::HashSet;
use std::rc::Rc;

const MISSING_SYNCHRONIZED_ANNOTATION: &str = "MISSING_SYNCHRONIZED_ANNOTATION";

type TopicName = String;

struct ImageAnnotationsContext {
    initial_scale: f32;
    initial_canvas_width: f32;
    initial_canvas_height: f32;
    initial_pixel_ratio: f32;
    topics(): Box<dyn Fn() -> Vec<Topic>>;
    config(): Rc<dyn Fn() -> ImageModeConfig>>;
    update_config(update_handler: fn(&mut ImageModeConfig));
    update_settings_tree(): fn();
    label_pool: LabelPool;
    message_handler: Rc<dyn IMessageHandler>;
}

pub struct ImageAnnotations {
    context: ImageAnnotationsContext,
    renderables_by_topic: std::collections::HashMap<TopicName, Box<dyn IModel>>;
    camera_model: Option<ICameraModel>;

    scale: f32;
    canvas_width: f32;
    canvas_height: f32;
    pixel_ratio: f32;

    supported_annotation_schemas: HashSet<&'static str>;
}

impl ImageAnnotations {
    pub fn new(context: ImageAnnotationsContext) -> Self {
        Self {
            context,
            renderables_by_topic: Default::default(),
            camera_model: None,
            scale: context.initial_scale,
            canvas_width: context.initial_canvas_width,
            canvas_height: context.initial_canvas_height,
            pixel_ratio: context.initial_pixel_ratio,
            supported_annotation_schemas: ALL_SUPPORTED_ANNOTATION_SCHEMAS.clone(),
        }
    }

    pub fn get_subscriptions() -> Vec<AnyRendererSubscriptionRust> {
        let schema_names = ALL_SUPPORTED_ANNOTATION_SCHEMAS.into_iter().collect();
        vec![AnyRendererSubscriptionRust {
            type_: "schema".to_string(),
            schema_names: schema_names,
            subscription: Rc::new(move |message_event: MessageEvent<FoxgloveImageAnnotations>| {
                self.#update_from_message_state(message_event);
            }),
        }]
    }

    pub fn handle_topics_changed(&mut self, topics: Vec<Topic>) {
        if topics.is_empty() {
            return;
        }
        let available_annotation_topics = topics.into_iter().filter(|topic| topic_is_convertible_to_schema(topic, ALL_SUPPORTED_ANNOTATION_SCHEMAS)).collect();

        self.context.message_handler.set_available_annotation_topics(available_annotation_topics);
    }

    fn filter_message_queue<T: 'static>(&self, msgs: Vec<MessageEvent<T>>) -> Vec<MessageEvent<T>> {
        // if sync annotations not active, only take the last message for each topic
        if !self.context.config().synchronize() {
            return msgs.into_iter().filter_map(|msg| msg.original_message()).collect();
        }
        msgs
    }

    pub fn dispose(&mut self) {
        for renderable in self.renderables_by_topic.values_mut() {
            renderable.dispose();
            self.remove(renderable);
        }
        self.renderables_by_topic.clear();
    }

    /** Called when seeking or a new data source is loaded.  */
    pub fn removeAllRenderables(&mut self) {
        for renderable in self.renderables_by_topic.values_mut() {
            renderable.dispose();
            self.remove(renderable);
        }
        self.renderables_by_topic.clear();
    }

    pub fn update_scale(
        &mut self,
        scale: f32,
        canvas_width: f32,
        canvas_height: f32,
        pixel_ratio: f32,
    ) {
        self.scale = scale;
        self.canvas_width = canvas_width;
        self.canvas_height = canvas_height;
        self.pixel_ratio = pixel_ratio;

        for renderable in self.renderables_by_topic.values_mut() {
            renderable.set_scale(scale, canvas_width, canvas_height, pixel_ratio);
            renderable.update();
        }
    }

    pub fn update_camera_model(&mut self, camera_model: ICameraModel) {
        self.camera_model = Some(camera_model);
        for renderable in self.renderables_by_topic.values_mut() {
            renderable.set_camera_model(camera_model);
            renderable.update();
        }
    }

    #update_from_message_state(&self, new_state: MessageRenderState) {
        if let Some((original_message, annotations)) = new_state.annotations_by_topic.get(&self.context.config().image_topic).map(|v| (v.original_message(), v.annotations())) {
            self.#handle_message(original_message, annotations);

            // Hide any remaining errors for annotations we are able to render
            self.context.remove_settings_error(["imageAnnotations", original_message.topic.as_str()], MISSING_SYNCHRONIZED_ANNOTATION);
        }
        for topic in new_state.present_annotation_topics.iter() {
            // Even if a full synchronized set is not found, hide errors for annotations that were present
            self.context.remove_settings_error(["imageAnnotations", topic.as_str()], MISSING_SYNCHRONIZED_ANNOTATION);
        }
        for topic in new_state.missing_annotation_topics.iter() {
            self.context.add_settings_error(
                ["imageAnnotations", topic.as_str()],
                MISSING_SYNCHRONIZED_ANNOTATION,
                "Waiting for annotation message with timestamp matching image. Turn off “Sync annotations” to display annotations regardless of timestamp.",
            );
        }
        if new_state.missing_annotation_topics.is_empty() {
            self.removeAllRenderables();
        }
    }

    #handle_message(
        &self,
        message_event: MessageEvent<FoxgloveImageAnnotations | RosImageMarker | RosImageMarkerArray>,
        annotations: Vec<Annotation>,
    ) {
        let renderable = match self.renderables_by_topic.get(&message_event.topic) {
            Some(renderable) => renderable,
            None => {
                let renderable = Box::new(ThreeDeeRender::new(message_event.topic.as_str(), &self.context.label_pool));
                self.renderables_by_topic.insert(message_event.topic.clone(), renderable);
                renderable
            }
        };

        renderable.set_original_message(message_event.message);
        renderable.set_annotations(annotations);
        renderable.update();
    }

    #handle_settings_action(&mut self, action: SettingsTreeAction) {
        if action.action != "update" || action.payload.path.len() < 2 {
            return;
        }
        let { value, path } = action.payload;
        let category = path[0];
        if category != "imageAnnotations" {
            return;
        }
        if path[2] == "visible" && typeof value == "boolean" {
            let topic = path[1].clone() as TopicName;
            self.#handle_topic_visibility_change(topic, value);
        }
        self.context.update_settings_tree();
    }

    #handle_topic_visibility_change(
        &self,
        topic: TopicName,
        visible: bool, // eslint-disable-line @lichtblick/no-boolean-parameters
    ) {
        self.context.update_config(|draft| {
            draft.annotations.entry(topic.clone()).or_insert_with(|| Default::default());
            draft.annotations.get_mut(&topic).unwrap().visible = visible;
        });
        self.context.message_handler.set_config(self.context.config());
        let renderable = match self.renderables_by_topic.get(&topic) {
            Some(renderable) => renderable,
            None => return,
        };

        renderable.visible = visible;
    }

    pub fn settings_nodes(&self) -> Vec<SettingsTreeEntry> {
        let mut entries: Vec<SettingsTreeEntry> = Vec::new();

        entries.push({
            path: ["imageAnnotations"].to_vec(),
            node: {
                label: "Image Annotations".to_string(),
                enable_visibility_filter: true,
                default_expansion_state: "expanded",
            },
        });
        let config = self.context.config().clone();
        let annotation_topics = self
            .context()
            .topics()
            .filter(|topic| topic_is_convertible_to_schema(topic, ALL_SUPPORTED_ANNOTATION_SCHEMAS))
            .collect();

        // Sort annotation topics with prefixes matching the image topic to the top.
        if config.image_topic.is_some() {
            sort_prefix_matchesToFront(
                &annotation_topics,
                config.image_topic.as_ref().unwrap(),
                |topic| topic.name.clone(),
            );
        }

        for topic in annotation_topics.iter() {
            let settings = match config.annotations.get(&topic.name) {
                Some(settings) => settings,
                None => &Default::default(),
            };
            entries.push({
                path: ["imageAnnotations", topic.name].to_vec(),
                node: {
                    label: topic.name.to_string(),
                    visible: settings.visible,
                    handler: self.#handle_settings_action.bind(self),
                },
            });
        }
        entries
    }
}
```