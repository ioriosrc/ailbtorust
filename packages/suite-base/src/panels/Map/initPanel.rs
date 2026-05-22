```rust
use std::rc::Rc;
use web_sys::{JsCast, UnwrapThrow};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use lue::Marker;
use lue::MarkerIcon;
use lue::MarkerImageOptions;
use leaflet::{css, Options as LeafletOptions};
use mapbox_gl::Map;

pub fn init_panel(
    crash: Rc<dyn Fn(Box<dyn std::error::Error>)>,
    context: Rc<PanelExtensionContext>,
) -> Box<dyn Fn()> {
    let marker_icon = MarkerIcon::new_from_url("leaflet/dist/images/marker-icon.png");
    marker_icon.set_options(MarkerImageOptions::new()
        .icon_size([25, 41])
        .icon_anchor([12, 41])
        .popup_anchor([1, -34])
        .tooltip_anchor([16, -28])
        .shadow_size([41, 41]));

    Box::new(move || {
        let map = Map::new_with_options(
            "map",
            LeafletOptions::default()
                .center([-51.2777, 39.0992]) // Sydney, Australia
                .zoom(12),
        );

        map.add_marker(
            Marker::new([[-51.2777, 39.0992], [-51.2778, 39.0994]])
                .set_icon(marker_icon.clone()),
        );
    })
}
```