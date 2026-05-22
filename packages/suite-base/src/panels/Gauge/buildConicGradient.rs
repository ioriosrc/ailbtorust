```rust
use std::f64;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

fn build_conic_gradient(config: &Config, gauge_angle: f64, height: f64, width: f64) -> String {
    let { color_map, color_mode, gradient, reverse } = config;

    let mut color_stops: Vec<ColorStop> = Vec::new();

    match color_mode {
        ColorModeConfig::COLORMAP => {
            match color_map {
                ColorMapConfig::RED_YELLOW_GREEN => color_stops.extend_from_slice(&COLOR_MAPS[ColorMapConfig::RED_YELLOW_GREEN]),
                ColorMapConfig::RAINBOW => color_stops.extend_from_slice(&COLOR_MAPS[ColorMapConfig::RAINBOW]),
                ColorMapConfig::TURBO => {
                    let num_stops = 20;
                    color_stops.resize(num_stops, ColorStop {
                        color: turbo_color_string(i as f64 / (num_stops - 1)),
                        location: i as f64 / (num_stops - 1),
                    });
                }
            }
        }
        ColorModeConfig::GRADIENT => {
            color_stops.push(ColorStop { color: gradient[0], location: 0.0 });
            color_stops.push(ColorStop { color: gradient[1], location: 1.0 });
        }
    }

    if reverse {
        color_stops.reverse();
    }

    let angle_and_position = format!(
        "from {}rad at 50% {:.2}%",
        gauge_angle * std::f64::consts::PI / 180.0,
        (width / 2.0) / (height * 1.0)
    );
    let angular_color_stop: String = color_stops
        .iter()
        .map(|stop| {
            format!(
                "{} {}rad",
                stop.color,
                stop.location * std::f64::consts::PI * 2.0 / gauge_angle
            )
        })
        .join(",");
    let color = color_stops.first().map_or("transparent", |stop| stop.color);

    format!("conic-gradient({} {}, {})", angle_and_position, angular_color_stop, color)
}
```