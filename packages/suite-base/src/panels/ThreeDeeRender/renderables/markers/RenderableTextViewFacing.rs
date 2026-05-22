```rust
use super::Renderer;
use super::{Marker, RenderableMarker};
use bevy_ecs::{self as ecs, prelude::*};
use bevy_pbr::prelude::*;

pub struct RenderableTextViewFacing {
    label: Label,
}

impl RenderableTextViewFacing {
    pub fn new(
        topic: String,
        marker: Marker,
        receive_time: Option<bevy_time::Instant>,
        renderer: &mut Renderer,
    ) -> Self {
        let mut label = renderer.label_pool.acquire();
        label.set_billboard(true);

        self.add(&label);
        self.update(marker, receive_time);

        RenderableTextViewFacing { label }
    }

    pub fn dispose(&mut self) {
        self.renderer.label_pool.release(&self.label);
    }

    pub fn update(&mut self, new_marker: Marker, receive_time: Option<bevy_time::Instant>) {
        super::RenderableMarker::update(self, new_marker, receive_time);

        let marker = &self.userData.marker;

        label.set_text(&marker.text);
        let alpha = marker.color.a;
        label.set_color(
            SRGBToLinear(marker.color.r),
            SRGBToLinear(marker.color.g),
            SRGBToLinear(marker.color.b),
            alpha,
        );

        let foreground_is_dark = get_luminance(&marker.color.r, &marker.color.g, &marker.color.b) < 0.5;
        if foreground_is_dark {
            label.set_background_color(1., 1., 1., alpha);
        } else {
            label.set_background_color(0., 0., 0., alpha);
        }
        label.set_line_height(marker.scale.z);
    }
}
```