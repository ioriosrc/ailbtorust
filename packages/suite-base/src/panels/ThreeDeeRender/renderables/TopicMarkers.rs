```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;

struct LayerSettingsMarker {
    color: Option<String>,
    show_outlines: Option<bool>,
    selected_id_variable: Option<String>,
    namespaces: HashMap<String, LayerSettingsMarkerNamespace>,
}

impl LayerSettingsMarker {
    fn new(topic: &str) -> Self {
        Self {
            color: None,
            show_outlines: None,
            selected_id_variable: None,
            namespaces: HashMap::new(),
        }
    }
}

struct LayerSettingsMarkerNamespace {
    visible: bool,
}

struct MarkersNamespace {
    topic: String,
    namespace: String,
    marker_pool: MarkerPool, // Assuming MarkerPool is a type defined elsewhere
    namespaces: HashMap<String, MarkersNamespace>,
}

impl MarkersNamespace {
    fn new(topic: &str, namespace: &str) -> Self {
        Self {
            topic: topic.to_string(),
            namespace: namespace.to_string(),
            marker_pool: MarkerPool::new(), // Assuming MarkerPool is a type defined elsewhere
            namespaces: HashMap::new(),
        }
    }

    fn add_marker_message(&mut self, marker: &Marker, receive_time: u64) {
        match marker.action {
            MarkerAction::ADD | MarkerAction::MODIFY => {
                self.#add_or_update_marker(marker, receive_time);
            }
            MarkerAction::DELETE => {
                self.#delete_marker(marker.namespace, marker.id);
            }
            MarkerAction::DELETEALL => {
                self.#delete_all_markers(marker.namespace);
            }
            _ => {
                // Unknown action
                self.renderer.settings.errors.addTo_topic(
                    self.topic.to_string(),
                    INVALID_MARKER_ACTION,
                    format!("Invalid marker action {}", marker.action),
                );
            }
        }
    }

    fn update(&mut self) {
        for ns in self.namespaces.values() {
            for renderable in &ns.markersById {
                renderable.update(renderable.value().marker, renderable.value().receive_time);
            }
        }
    }

    fn start_frame(
        &self,
        current_time: u64,
        render_frame_id: String,
        fixed_frame_id: String,
    ) {
        self.visible = self.settings.visible;
        if !self.visible {
            self.renderer.settings.errors.clear_topic(self.topic.to_string());
            return;
        }

        for ns in self.namespaces.values() {
            for renderable in &ns.markersById {
                renderable.visible = ns.settings.visible;
                if !renderable.visible {
                    continue;
                }

                let marker = renderable.value().marker;
                let receive_time = renderable.value().receive_time;
                let expiresIn = renderable.value().expires_in;

                // Check if this marker has expired
                if expires_in != None && current_time > receive_time + expiresIn as u64 {
                    self.#delete_marker(ns.namespace, marker.id);
                    continue;
                }

                let frame_id = self.renderer.normalize_frame_id(marker.header.frame_id.to_string());
                let src_time = marker.frame_locked.unwrap_or(current_time);
                let updated = update_pose(
                    renderable,
                    self.transform_tree.clone(),
                    render_frame_id.to_string(),
                    fixed_frame_id.to_string(),
                    frame_id.to_string(),
                    current_time,
                    src_time,
                );
                renderable.visible = updated;
                let topic = renderable.value().topic.to_string();
                if !updated {
                    let message = missing_transform_message(
                        &render_frame_id.to_string(),
                        &fixed_frame_id.to_string(),
                        &frame_id.to_string(),
                    );
                    self.renderer.settings.errors.addTo_topic(topic, MISSING_TRANSFORM, message);
                } else {
                    self.renderer.settings.errors.removeFrom_topic(topic, MISSING_TRANSFORM);
                }
            }
        }
    }

    fn #add_or_update_marker(&mut self, marker: &Marker, receive_time: u64) {
        let ns = self.namespaces.get_mut(marker.namespace.as_str());
        if ns.is_none() {
            ns = Some(self.#create_markers_namespace(marker.namespace.as_str()));
        }

        let renderable = ns.as_mut().unwrap().markers_by_id.entry(marker.id).or_insert_with(|| {
            let pool = self.marker_pool.clone();
            match marker.type_ as i32 {
                0 => Box::new(MarkerTypeArrow::new(marker, receive_time)) as Box<dyn Renderable>,
                // Add other marker types here
                _ => unreachable!("Invalid marker type"),
            }
        });

        renderable.update(marker, receive_time);
    }

    fn #delete_marker(&mut self, namespace: &str, id: i32) {
        let ns = self.namespaces.get_mut(namespace).unwrap();
        if let Some(renderable) = ns.markers_by_id.remove(&id) {
            self.marker_pool.release(renderable);
        }
    }

    fn #delete_all_markers(&mut self, namespace: &str) {
        let mut ns = self.namespaces.get_mut(namespace).unwrap_or_else(|| panic!("Namespace not found"));
        for id in ns.markers_by_id.keys() {
            self.#delete_marker(namespace, *id);
        }
    }

    fn #create_markers_namespace(namespace: &str) -> MarkersNamespace {
        Self::new(namespace, namespace)
    }
}
```