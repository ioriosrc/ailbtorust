```rust
use leptos::*;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

fn main() -> Html {
    let (width, set_width) = create_signal(500);
    let (height, set_height) = create_signal(700);
    let (is_bounds_reset, set_is_bounds_reset) = create_signal(false);
    let (data, set_data) = create_signal(vec![data_point]);
    let (options, set_options) = create_signal(options);

    let chart_component_props = props! {
        width,
        height,
        is_bounds_reset,
        data,
        options,
        type: "scatter",
    };

    html! {
        <div style={style! {
            width: *width,
            height: *height,
            background: "#000",
        }}>
            <ChartComponent {...chart_component_props} />
        </div>
    }
}
```