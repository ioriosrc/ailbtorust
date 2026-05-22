```rust
use chrono::{Duration, Utc};
use geometry_msgs::Point32;
use std::time::Instant;

struct LayerSettingsPolygon {
    visible: bool,
    lineWidth: f64,
    color: String,
}

const DEFAULT_COLOR = "rgba(124, 107, 1, 1)";
const DEFAULT_LINE_WIDTH = 0.1;

fn makeRgba(color: &str) -> [u8; 4] {
    let parts = color.split(',').map(|s| s.trim().parse::<f64>().unwrap()).collect::<Vec<f64>>();
    if parts.len() != 4 || parts[3] < 0.0 || parts[3] > 1.0 {
        panic!("Invalid RGBA color");
    }
    [parts[0] as u8, parts[1] as u8, parts[2] as u8, (parts[3] * 255.0) as u8]
}

fn rgbaToCssString(color: &[u8; 4]) -> String {
    format!("rgba({}, {}, {}, {})", color[0], color[1], color[2], color[3])
}

struct PolygonRenderable {
    topic: String,
    receiveTime: Instant,
    messageTime: u64,
    frameId: String,
    pose: geometry_msgs::Pose,
    settings: LayerSettingsPolygon,
    lines: Option<RenderableLineStrip>,
}

impl PolygonRenderable {
    fn new(
        topic: String,
        renderer: &Renderer,
        receive_time: Instant,
        message_time: u64,
        frame_id: String,
        pose: geometry_msgs::Pose,
        settings_path: Vec<&str>,
        settings: LayerSettingsPolygon,
        topic_stamped: PolygonStamped,
        lines: Option<RenderableLineStrip>,
    ) -> Self {
        Self {
            topic,
            receive_time,
            message_time,
            frame_id,
            pose,
            settings,
            lines,
        }
    }

    fn dispose(&mut self) {
        if let Some(ref mut lines) = self.lines {
            lines.dispose();
        }
    }
}

struct Polygons(SceneExtension<PolygonRenderable>) {
    renderer: &Renderer;

    pub fn new(renderer: &Renderer, name: String) -> Self {
        Self { renderer }
    }

    fn get_subscriptions(&self) -> Vec<AnyRendererSubscription> {
        vec![AnyRendererSubscription::new(
            "schema",
            POLYGON_STAMPED_DATATYPES,
            self.handle_polygon.clone(),
            Box::pin(Self::only_last_by_topic_message),
        )]
    }

    fn settings_nodes(&self) -> Vec<SettingsTreeEntry> {
        let config_topics = &self.renderer.config.topics;
        let handler = self.handle_settings_action.clone();
        let mut entries: Vec<SettingsTreeEntry> = vec![];
        for topic in self.renderer.topics.iter().flatten() {
            if !topic_is_convertible_to_schema(topic, POLYGON_STAMPED_DATATYPES) {
                continue;
            }
            let config = (config_topics.get(topic.name()).unwrap_or_default()) as Partial<LayerSettingsPolygon>;
            let fields: SettingsTreeFields = vec![
                ("Line Width", |v| v.to_string()),
                ("Color", |v| rgba_to_css_string(&makeRgba(v.as_str()))),
            ];

            entries.push(SettingsTreeEntry {
                path: ["topics".to_string(), topic.name().to_string()],
                node: SettingsTreeNode::new(
                    topic.name().to_string(),
                    "Star".to_string(),
                    fields,
                    config.visible.unwrap_or(DEFAULT_SETTINGS.visible),
                    handler.clone(),
                ),
            });
        }
        entries
    }

    fn handle_settings_action(&self, action: SettingsTreeAction) -> Result<(), SettingsTreeError> {
        if action.action != "update" || action.payload.path.len() != 3 {
            return Err(SettingsTreeError::InvalidPath);
        }

        self.save_setting(action.payload.path.clone(), action.payload.value.as_str());

        // Update the renderable
        let topic_name = action.payload.path[1];
        let renderable = self.renderables.get(topic_name).unwrap();
        let settings = self.renderer.config.topics.get(topic_name).unwrap_or_default() as Partial<LayerSettingsPolygon>;

        renderable.userData.settings = { ...DEFAULT_SETTINGS, ...settings };
        self.#update_polygon_renderable(renderable, renderable.userData.polygon_stamped.clone(), renderable.receive_time);
        Ok(())
    }

    fn #handle_polygon(&self, message_event: PartialMessageEvent<PolygonStamped>) -> Result<(), SettingsTreeError> {
        let topic = message_event.topic;
        let polygon stamped = normalize_polygon stamped(message_event.message);
        let receive_time = Utc::now().timestamp_nanos() as u64;

        let mut renderable = self.renderables.get(topic).unwrap_or_else(|| {
            // Set the initial settings from default values merged with any user settings
            let user_settings = &self.renderer.config.topics[topic];
            let settings = { ...DEFAULT_SETTINGS, ..user_settings };

            PolygonRenderable::new(
                topic.to_string(),
                self.renderer,
                Instant.now(),
                receive_time,
                String::from(""),
                geometry_msgs::Pose::default(),
                settings,
                Vec::from([["topics".to_string(), topic.to_string()].as_ref()]),
                polygon stamped.clone(),
                None,
            )
        });

        self.add(&renderable);
        self.renderables.insert(topic, renderable);

        self.#update_polygon_renderable(renderable, polygon stamped.clone(), receive_time);
        Ok(())
    }

    fn #update_polygon_renderable(
        &mut self,
        renderable: &PolygonRenderable,
        polygon_stamped: PolygonStamped,
        receive_time: u64,
    ) {
        let settings = renderable.userData.settings;

        renderable.receive_time = receive_time;
        renderable.message_time = message_stamped.header.stamp.nanos() as u64;
        renderable.frame_id = self.renderer.normalize_frame_id(polygon_stamped.header.frame_id);
        renderable.pose = geometry_msgs::Pose::default();
        renderable.lines = Some(renderable.lines.take().unwrap_or_else(|| {
            let points = normalize_vector3s(&polygon_stamped.polygon.points).unwrap_or(vec![]);
            RenderableLineStrip::new(
                topic.clone(),
                create_line_strip_marker(polygon_stamped, settings),
                receive_time,
                self.renderer,
            )
        }));

        if let Some(ref mut lines) = renderable.lines {
            lines.update(create_line_strip_marker(polygon_stamped, settings), receive_time);
        }
    }

    fn create_line_strip_marker(
        polygon_stamped: PolygonStamped,
        settings: LayerSettingsPolygon,
    ) -> geometry_msgs::Marker {
        // Close the polygon
        let points = normalize_vector3s(&polygon_stamped.polygon.points).unwrap_or(vec![]);
        if points.len() > 0 {
            points.push(points[0]);
        }

        geometry_msgs::Marker {
            header: polygon_stamped.header,
            ns: "",
            id: 0,
            type: geometry_msgs::msg::MarkerType::LINE_STRIP,
            action: geometry_msgs::msg::MarkerAction::ADD,
            pose: geometry_msgs::Pose::default(),
            scale: geometry_msgs::msg::Vector3 { x: settings.lineWidth, y: 1.0, z: 1.0 },
            color: rgba_to_css_string(&makeRgba(settings.color)),
            lifetime: Duration::from_nanos(0),
            frame_locked: true,
            points,
            colors: Vec::new(),
            text: "",
            mesh_resource: "",
            mesh_use_embedded_materials: false,
        }
    }

    fn normalize_polygon(polygon: Option<&PartialMessage<Polygon>>) -> Option<Vec<Point32>> {
        polygon
            .map(|p| p.polygon.points)
            .unwrap_or(vec![])
            .iter()
            .cloned()
            .collect::<Vec<Point32>>()
    }

    fn normalize_polygon_stamped(polygon_stamped: &PartialMessage<PolygonStamped>) -> PolygonStamped {
        PolygonStamped {
            header: polygon_stamped.header,
            polygon: self.normalize_polygon(Some(&polygon_stamped.polygon)).unwrap_or(vec![]),
        }
    }
}
```