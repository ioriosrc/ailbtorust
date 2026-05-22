```rust
use std::vec::Vec;

#[derive(Debug, Clone)]
struct LayerSettingsGrid {
    visible: bool,
    frame_locked: bool,
    label: String,
    instance_id: String,
    layer_id: String,
    frame_id: Option<String>,
    size: f64,
    divisions: u32,
    lineWidth: f64,
    color: String,
    position: [f64; 3],
    rotation: [f64; 3],
}

#[derive(Debug)]
struct GridRenderable {
    user_data: GridUserData,
}

#[derive(Debug)]
struct GridUserData {
    settings: LayerSettingsGrid,
    line_list: RenderableLineList,
}

struct Grids {
    renderer: IRenderer,
}

impl Grids {
    fn new(renderer: IRenderer) -> Self {
        Self { renderer }
    }

    fn dispose(&mut self) {
        // no-op
    }

    fn remove_all_renderables(&mut self) {
        // no-op
    }

    fn settings_nodes(&self) -> Vec<SettingsTreeEntry> {
        let handler = |action: SettingsTreeAction| match action.action {
            "reorder-node" => (),
            "perform-node-action" => {
                if action.payload.path.len() == 2 && action.payload.id == "delete" {
                    self.remove_grid(action.payload.path[1].unwrap());
                }
            },
            _ => (),
        };

        let mut entries: Vec<SettingsTreeEntry> = Vec::new();

        for (instance_id, layer_config) in self.renderer.config.layers() {
            if layer_config.layer_id != LAYER_ID {
                continue;
            }

            let config = layer_config as Partial<LayerSettingsGrid>;
            let frame_id_options = [
                { label: "<Display frame>", value: None },
                ...self.renderer.coordinate_frame_list(),
            ];

            let fields: SettingsTreeFields = vec![
                ("frameId", "select", options, Some(config.frame_id)),
                ("size", "number", 0.0, 1.0, PRECISION_DISTANCE, Some(format!("{}", config.size))),
                ("divisions", "number", 1, MAX_DIVISIONS as f64, PRECISION_DEGREES, Some(format!("{}", config.divisions))),
                ("lineWidth", "number", 0.0, 1.0, PRECISION_DEGREES, Some(format!("{}", config.lineWidth))),
                ("color", "rgba", None),
                ("position", "vec3", vec!["X", "Y", "Z"], PRECISION_DISTANCE, Some(vec![config.position[0], config.position[1], config.position[2]])),
                ("rotation", "vec3", vec!["R", "P", "Y"], PRECISION_DEGREES, Some(vec![config.rotation[0], config.rotation[1], config.rotation[2]])),
            ];

            entries.push(SettingsTreeEntry {
                path: ["layers", instance_id],
                node: SettingsTreeNode {
                    label: config.label.unwrap_or_else(|| t!("threeDee:grid")),
                    icon: "Grid",
                    fields,
                    visible: config.visible.unwrap_or(DEFAULT_SETTINGS.visible),
                    actions: vec![SettingsNodeAction {
                        type: "action",
                        id: "delete",
                        label: t!("threeDee:delete"),
                    }],
                    order: layer_config.order.unwrap_or(1),
                    handler,
                },
            });

            if !self.renderables.contains_key(instance_id) {
                self.create_grid(instance_id, config);
            }
        }

        entries
    }

    fn start_frame(
        &mut self,
        currentTime: std::time::Duration,
        render_frame_id: String,
        fixed_frame_id: String,
    ) {
        for (renderable in self.renderables.values_mut()) {
            renderable.user_data.frame_id = renderable.user_data.settings.frame_id.as_ref().unwrap_or(&render_frame_id);
        }
        super::start_frame(currentTime, render_frame_id, fixed_frame_id);
    }

    fn handle_settings_action(&mut self, action: SettingsTreeAction) {
        if action.action == "reorder-node" {
            return;
        }
        let path = action.payload.path;

        if (action.action == "perform-node-action") {
            if (path.len() === 2 && action.payload.id == "delete") {
                let instance_id = path[1].unwrap();
                self.remove_grid(instance_id);
                self.update_settings_tree();
                self.renderer.update_custom_layers_count();
            }
            return;
        }

        if path.len() != 3 {
            return; // Doesn't match the pattern of ["layers", instance_id, field]
        }

        self.save_setting(path, action.payload.value);

        let instance_id = path[1].unwrap();
        let settings = self.renderer.config.layers(instance_id).as_ref().unwrap_or(&DEFAULT_SETTINGS);
        self.update_grid(instance_id, settings);
    }

    fn add_grid(&mut self, instance_id: String) {
        log::info!("Creating {LAYER_ID} layer {instance_id}");

        let config: LayerSettingsGrid = { ..DEFAULT_SETTINGS };
        config.instance_id = instance_id;

        // Add this instance to the config
        self.renderer.update_config(|draft| draft.layers.insert(instance_id, config));

        // Add a renderable
        self.create_grid(instance_id, config);

        // Update the settings tree
        self.update_settings_tree();
    }

    fn update_grid(&mut self, instance_id: String, settings: &LayerSettingsGrid) {
        let mut renderable = self.renderables.get_mut(instance_id).unwrap_or_else(|| {
            let marker = create_marker(settings);
            let line_list_id = format!("{}:LINE_LIST", instance_id);
            let line_list = RenderableLineList::new(line_list_id, marker, None, self.renderer, LineOptions {});
            let renderable = GridRenderable { user_data: GridUserData { settings: settings.clone(), line_list } };
            self.add(renderable);
            self.renderables.insert(instance_id, renderable);
            renderable
        });

        let prev_settings = &renderable.user_data.settings;
        let markers_equal =
            settings.size == prev_settings.size &&
            settings.divisions == prev_settings.divisions &&
            settings.frame_id.as_ref() == prev_settings.frame_id.as_ref() &&
            settings.lineWidth == prev_settings.lineWidth &&
            settings.color == prev_settings.color;

        renderable.user_data.settings = settings.clone();

        if !markers_equal {
            let marker = create_marker(settings);
            renderable.user_data.line_list.update(marker, None);
        }

        if (
            !vec3_tuple_approx_equals(&settings.position, &prev_settings.position)
                || !vec3_tuple_approx_equals(&settings.rotation, &prev_settings.rotation)
        ) {
            let pose = xyzrpy_to_pose(&settings.position, &settings.rotation);
            renderable.user_data.pose = pose;
        }
    }

    fn create_grid(&mut self, instance_id: String, settings: LayerSettingsGrid) -> GridRenderable {
        let marker = create_marker(settings);
        let line_list_id = format!("{}:LINE_LIST", instance_id);
        let line_list = RenderableLineList::new(line_list_id, marker, None, self.renderer, LineOptions {});
        let renderable = GridRenderable { user_data: GridUserData { settings, line_list } };
        self.add(renderable);
        self.renderables.insert(instance_id, renderable);
        renderable
    }
}

fn create_marker(settings: &LayerSettingsGrid) -> Marker {
    let size = settings.size;
    let divisions = settings.divisions as usize;
    let step = size / divisions as f64;
    let half_size = size / 2.0;

    let mut points: Vec<Vector3> = Vec::new();
    // Create a grid of line segments centered around <0, 0>
    for i in 0..divisions + 1 {
        let x = -half_size + i as f64 * step;
        points.push(Vector3 { x, y: -half_size, z: 0.0 });
        points.push(Vector3 { x, y: half_size, z: 0.0 });
        points.push(Vector3 { x: -half_size, y: x, z: 0.0 });
        points.push(Vector3 { x: half_size, y: x, z: 0.0 });
    }

    let color = Color::from_hex(settings.color.as_str()).unwrap();
    Marker {
        header: Header {
            frame_id: String::new(),
            stamp: Time::ZERO,
        },
        ns: String::new(),
        id: 0,
        type_: MarkerType::LINE_LIST,
        action: MarkerAction::ADD,
        pose: Matrix4x4::IDENTITY,
        scale: Vector3 { x: settings.lineWidth, y: 1.0, z: 1.0 },
        color,
        lifetime: Time::ZERO,
        frame_locked: true,
        points,
        colors: Vec::new(),
        text: String::new(),
        mesh_resource: String::new(),
        mesh_use_embedded_materials: false,
    }
}
```