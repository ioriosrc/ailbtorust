```rust
use super::ros::{RosValue};
use crate::{color, layer_settings::LayerSettingsMarker, renderer::IRenderer, Renderable};

pub struct MarkerUserData {
    topic: String,
    marker: crate::ros::Marker,
    original_marker: crate::ros::Marker,
    expires_in: Option<chrono::Duration>,
}

impl Renderable<MarkerUserData> for RenderableMarker {
    fn new(
        topic: String,
        marker: crate::ros::Marker,
        receive_time: chrono::Duration,
        renderer: &IRenderer,
    ) -> Self {
        let name = Self::get_marker_id(&topic, Some(&marker.ns), marker.id);
        let has_lifetime = marker.lifetime.sec != 0 || marker.lifetime.nsec != 0;

        Self {
            name,
            renderer,
            receive_time: receive_time.as_nanos(),
            message_time: marker.header.stamp.to_nanoseconds_since_unix_epoch(),
            frame_id: renderer.normalize_frame_id(marker.header.frame_id),
            pose: marker.pose,
            settings_path: vec![String::from("topics"), topic],
            settings: {
                visible: true,
                frame_locked: marker.frame_locked,
            },
            topic,
            marker,
            original_marker: marker,
            expires_in: has_lifetime.then_some(chrono::Duration::from_nanos(marker.lifetime.sec as i64 * 1_000 + marker.lifetime.nsec as i32)),
        }
    }

    fn id_from_message(&self) -> Option<i64> {
        self.userData.marker.id
    }

    fn selected_id_variable(&self) -> Option<&str> {
        self.get_settings().map(|settings| settings.selected_id_variable.as_str())
    }

    fn get_settings(&self) -> &LayerSettingsMarker {
        self.renderer.config.topics.get(&self.topic).unwrap_or_default()
    }

    fn details(&self) -> RosValue {
        self.userData.original_marker
    }

    fn update(&mut self, marker: crate::ros::Marker, receive_time: chrono::Duration) {
        let has_lifetime = marker.lifetime.sec != 0 || marker.lifetime.nsec != 0;

        if receive_time.is_some() {
            self.userData.receive_time = receive_time.as_nanos();
        }
        self.userData.message_time = marker.header.stamp.to_nanoseconds_since_unix_epoch();
        self.userData.frame_id = self.renderer.normalize_frame_id(marker.header.frame_id);
        self.userData.pose = marker.pose;
        self.userData.marker = self.#render_marker(&marker);
        self.userData.original_marker = marker;
        self.userData.expires_in = has_lifetime.then_some(chrono::Duration::from_nanos(marker.lifetime.sec as i64 * 1_000 + marker.lifetime.nsec as i32));
    }

    fn _marker_colors_to_linear(
        &mut self,
        marker: &crate::ros::Marker,
        points_length: usize,
        callback: &mut dyn FnMut(&[f32], usize),
    ) {
        let color = color::srgb_to_linear(marker.color);

        for i in 0..points_length {
            let srgb = marker.colors.get(i);
            if let Some(srgb) = srgb {
                // Per-point color
                let color2 = color::srgb_to_linear(*srgb);
                callback(&[color2.r, color2.g, color2.b, srgb.a], i);
            } else {
                // Base marker color
                callback(&[color.r, color.g, color.b, marker.color.a], i);
            }
        }
    }

    fn #render_marker(&self, marker: &crate::ros::Marker) -> crate::ros::Marker {
        let topic_name = self.userData.topic;
        let settings = self.renderer.config.topics.get(&topic_name).unwrap_or_default();
        let color_str = settings.color;

        if color_str.is_empty() {
            return marker.clone();
        }

        // Create a clone of the marker with the color overridden
        let color = color::srgb_to_linear(color::string_to_rgba(color_str));
        let new_marker = crate::ros::Marker {
            ..marker.clone()
        };
        new_marker.color = color;
        new_marker.colors = vec![];
        return new_marker;
    }
}
```