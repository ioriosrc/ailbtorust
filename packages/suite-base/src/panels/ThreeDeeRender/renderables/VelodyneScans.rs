```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use foxglove_schemas::{
    NumericType, PointCloud as FoxglovePointCloud, RosObject, VelodyneScan,
};
use lichtblick_rostime::{to_nanosec, to_sec};

use suite::base::panels::three_dee_render::renderables::{PointCloudHistoryRenderable, PointCloudMaterial};
use suite_base::players::{RawPacket, Transformer};
use suite_base::types::{Calibration, Model, PointCloud, PointFieldDataType, RawPacket};
use suite_base::utils::make_pose;

type LayerSettingsVelodyneScans = LayerSettingsPointExtension & {
    stixels_enabled: bool;
};
const DEFAULT_SETTINGS = {
    ...DEFAULT_POINT_SETTINGS,
    stixels_enabled: false,
    color_field_computed: None,
};

fn point_field_dtype_to_numeric_type(type: PointFieldDataType) -> NumericType {
    match type {
        PointFieldDataType::UINT8 => NumericType::UINT8,
        PointFieldDataType::INT8 => NumericType::INT8,
        PointFieldDataType::UINT16 => NumericType::UINT16,
        PointFieldDataType::INT16 => NumericType::INT16,
        PointFieldDataType::UINT32 => NumericType::UINT32,
        PointFieldDataType::INT32 => NumericType::INT32,
        PointFieldDataType::FLOAT32 => NumericType::FLOAT32,
        PointFieldDataType::FLOAT64 => NumericType::FLOAT64,
        _ => NumericType::UNKNOWN,
    }
}

struct VelodyneCloudConverter {
    transformers: HashMap<Model, Transformer>,
}

impl VelodyneCloudConverter {
    fn decode(&self, scan: VelodyneScan) -> Option<FoxglovePointCloud> {
        if scan.packets.is_empty() {
            return None;
        }

        let first_packet_data = scan.packets[0];
        let model = RawPacket::infer_model(first_packet_data.data);
        if model.is_none() {
            return None;
        }

        let stamp = to_sec(scan.header.stamp);
        let max_points = RawPacket::MAX_POINTS_PER_PACKET * scan.packets.len();
        let cloud = PointCloud { stamp, max_points };
        let transformer = self.get_transformer(model.unwrap());

        for packet in scan.packets {
            transformer.unpack(&RawPacket(packet.data), stamp, to_sec(packet.stamp), &mut cloud);
        }

        cloud.trim();

        if cloud.width == 0 || cloud.height == 0 {
            return None;
        }

        Some({
            timestamp: scan.header.stamp,
            frame_id: scan.header.frame_id,
            pose: make_pose(),
            point_stride: cloud.point_step,
            fields: cloud.fields
                .iter()
                .map(|field| ({
                    name: field.name.clone(),
                    offset: field.offset,
                    type: point_field_dtype_to_numeric_type(field.datatype),
                }))
                .collect(),
            data: cloud.data,
        })
    }

    fn get_transformer(&self, model: Model) -> &Transformer {
        self.transformers
            .get(&model)
            .expect("Missing transformer for model")
    }
}

struct VelodyneScans {
    renderables: HashMap<String, PointCloudHistoryRenderable>,
    point_cloud_fields_by_topic: HashMap<String, Vec<&str>>,
    velodyne_cloud_converter: VelodyneCloudConverter,
}

impl VelodyneScans {
    fn new(renderer: &suite_base::IRenderer) -> Self {
        let mut renderables = HashMap::new();
        let point_cloud_fields_by_topic = HashMap::new();
        let velodyne_cloud_converter = VelodyneCloudConverter {};

        Self {
            renderables,
            point_cloud_fields_by_topic,
            velodyne_cloud_converter,
        }
    }

    fn get_subscriptions(&self) -> Vec<(String, suite_base::IRendererSubscription)> {
        vec![(
            "foxglove.VelodyneScans".to_string(),
            suite_base::IRendererSubscription {
                type_: "schema",
                schema_names: vec!["velodyne_msgs/VelodyneScan".to_string()],
                subscription: {
                    handler: Self::handle_velodyne_scan,
                    filter_queue: self.#process_message_queue.bind(self),
                },
            },
        )]
    }

    fn #process_message_queue<T>(&self, msgs: Vec< suite_base::MessageEvent<T>>) -> Vec<suite_base::MessageEvent<T>> {
        if msgs.is_empty() {
            return msgs;
        }
        let mut msgs_by_topic = HashMap::new();
        for msg in &msgs {
            let { topic } = msg;
            let receive_time = to_nanosec(msg.receive_time);
            let point_cloud = self.velodyne_cloud_converter.decode(msg.message);
            if !point_cloud.is_some() {
                return msgs;
            }

            // Update the mapping of topic to point cloud field names if necessary
            let fields = self.point_cloud_fields_by_topic.get(topic).unwrap_or(&vec![]);
            let fields_updated = false;
            if fields.len() != point_cloud.fields.len() {
                fields = point_cloud.fields.iter().map(|field| field.name.clone()).collect();
                self.point_cloud_fields_by_topic.insert(topic.to_string(), fields);
                fields_updated = true;
                self.update_settings_tree();
            }

            let renderable = self.renderables.get(topic).unwrap_or(&PointCloudHistoryRenderable {
                topic: topic.to_string(),
                renderer: self,
                receive_time,
                message_time: to_sec(msg.message.receive_time),
                frame_id: self.renderer.normalize_frame_id(msg.message.header.frame_id),
                pose: make_pose(),
                settings_path: vec![topic.to_string()],
                settings: DEFAULT_SETTINGS.clone(),
                topic,
                latest_cloud: point_cloud.unwrap(),
                latest_original_message: msg.message as RosObject,
                material: PointCloudMaterial::new(DEFAULT_SETTINGS.clone()),
                picking_material: create_picking_material(DEFAULT_SETTINGS.clone()),
                instance_picking_material: create_instance_picking_material(DEFAULT_SETTINGS.clone()),
                stixel_material: create_stixel_material(DEFAULT_SETTINGS.clone()),
            });

            self.add(renderable);
            self.renderables.insert(topic.to_string(), renderable);
        }
        msgs
    }

    fn update_settings_tree(&self) {
        // Implement the logic to update the settings tree based on the current topic and its settings
        // This method should be implemented to handle the change in settings for each topic
    }

    fn handle_settings_action(&mut self, action: suite_base::SettingsTreeAction) {
        let path = action.payload.path;
        if action.action != "update" || path.len() != 3 {
            return;
        }

        self.save_setting(path, action.payload.value);

        // Update the renderable
        let topic_name = path[1].to_string();
        let renderable = self.renderables.get_mut(&topic_name).unwrap_or_else(|| {
            unimplemented!("Renderable not found for topic: {}", topic_name);
        });

        let prev_settings = self.renderer.config.topics.get(&topic_name).unwrap_or_default();
        let settings = { ...DEFAULT_SETTINGS, ..prev_settings };

        if settings.color_field.is_none() && renderable.settings_changed_fields.contains_key("colorField")) {
            auto_select_color_settings(settings.clone(), &renderable.fields, suite_base::ColorModeConfig {
                supports_packed_rgb_modes: false,
            });

            self.renderer.update_config(|draft| {
                draft.topics.get_mut(&topic_name).unwrap().clone().color_field = settings.color_field;
                draft.topics.get_mut(&topic_name).unwrap().color_mode = settings.color_mode;
                draft.topics.get_mut(&topic_name).unwrap().color_map = settings.color_map;
            });
        }

        renderable.update_settings(settings);
    }
}
```