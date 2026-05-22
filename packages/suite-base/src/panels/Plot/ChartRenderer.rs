```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use web_sys::{
    self as js,
    event_target::EventTarget,
};

struct ChartRenderer {
    canvas: Box<dyn EventTarget>,
    chart_instance: ChartType,
}

impl ChartRenderer {
    pub fn new(args: ChartRendererProps) -> Self {
        let fake_node = {
            add_event_listener(&self.canvas, "wheel", self.on_wheel_event.as_ref());
            add_event_listener(&self.canvas, "panstart", self.on_pan_start_event.as_ref());
            add_event_listener(&self.canvas, "panmove", self.on_pan_move_event.as_ref());
            add_event_listener(&self.canvas, "panend", self.on_pan_end_event.as_ref());

            web_sys::NodeBox::new()
        };

        let chart_options = get_chart_options({
            device_pixel_ratio: args.device_pixel_ratio,
            grid_color: args.grid_color,
            tick_color: args.tick_color,
        });

        let orig_zoom_start = ZoomPlugin.start.unwrap_or_else(|e| {
            panic!("ZoomPlugin.start is not available on this environment");
        });
        ZoomPlugin.start = Box::new(move |chartInstance, start_args, plugin_options| {
            // swap the canvas with our fake dom node canvas to support zoom plugin addEventListener
            let ctx = chart_instance.ctx;
            chart_instance.ctx = {
                canvas: &fake_node,
            };
            let res = orig_zoom_start(chart_instance as Chart, start_args, plugin_options);
            chart_instance.ctx = ctx;
            res
        });

        // ChartJS supports offscreen canvas however the type definitions do not so we need to cast and
        // fool the constructor.
        //
        // https://www.chartjs.org/docs/latest/general/performance.html#parallel-rendering-with-web-workers-chromium-only
        let canvas = args.canvas as Box<dyn EventTarget>;
        let chart_instance = Chart::new({
            ctx: &canvas,
            type: "scatter",
            data: {
                datasets: [],
            },
            options: chart_options,
            plugins: [ZoomPlugin],
        });

        ZoomPlugin.start = orig_zoom_start;
        Self { canvas, chart_instance }
    }

    pub fn update(&mut self, action: UpdateAction) -> Bounds | Option<Bounds> {
        if action.size.is_some() {
            self.canvas.set_width(action.size.unwrap().width);
            self.canvas.set_height(action.size.unwrap().height);
        }

        if let Some(y_bounds) = &action.y_bounds {
            let scale_option = match &self.chart_instance.options.scales.get("y") {
                Some(s) => s,
                None => return Option::None,
            };
            if scale_option.min != y_bounds.min {
                scale_option.set_min(y_bounds.min);
            }
            if scale_option.max != y_bounds.max {
                scale_option.set_max(y_bounds.max);
            }
        }

        if let Some(x_bounds) = &action.x_bounds {
            let scale_option = match &self.chart_instance.options.scales.get("x") {
                Some(s) => s,
                None => return Option::None,
            };
            if scale_option.min != x_bounds.min {
                scale_option.set_min(x_bounds.min);
            }

            if scale_option.max != x_bounds.max {
                scale_option.set_max(x_bounds.max);
            }
        }

        if let Some(interaction_events) = &action.interaction_events {
            for event in interaction_events {
                self.apply_interaction_event(event);
            }
        }

        if action.zoom_mode.is_some() {
            self.chart_instance.options.plugins.get("zoom")?.get("zoom").unwrap().mode = action.zoom_mode.unwrap();
        }

        if let Some(reference_lines) = &action.reference_lines {
            let annotation = self.chart_instance.options.plugins.get("annotation");
            if let Some(annotation) = annotation {
                let new_annotations: Vec<AnnotationOptions> = reference_lines
                    .iter()
                    .map(|config| {
                        AnnotationOptions {
                            ..DEFAULT_ANNOTATION
                        }
                        .with_border_color(config.color)
                        .with_value(config.value),
                    })
                    .collect();

                annotation.as_mut().unwrap().annotations = new_annotations;
            }
        }

        // NOTE: "none" disables animations - this is important for chart performance because we update
        // the entire data set which does not preserve history for the chart animations
        self.chart_instance.update("none");
        Some(self.get_x_scale())
    }

    pub fn get_elements_at_pixel(&self, pixel: { x: f64; y: f64 }) -> Vec<HoverElement> {
        let x = pixel.x;
        let y = pixel.y;

        let ev = {
            native: true,
            x,
            y,
        };

        // ev is cast to any because the typings for getElementsAtEventForMode are wrong
        // ev is specified as a dom Event - but the implementation does not require it for the basic platform
        let elements = self.chart_instance.get_elements_at_event_for_mode(
            &ev as Box<dyn EventTarget>,
            self.chart_instance.options.interaction.mode.unwrap_or("intersect"),
            self.chart_instance.options.interaction.unwrap_or(&{}),
            false,
        );

        let mut out: Vec<HoverElement> = Vec::new();

        // sort elements by proximity to the cursor so the closer items are earlier in the list
        elements.sort_by(|a, b| {
            let dxA = x - a.element.x;
            let dyA = y - a.element.y;
            let dxB = x - b.element.x;
            let dyB = y - b.element.y;
            let dist_squared_a = dxA * dxA + dyA * dyA;
            let dist_squared_b = dxB * dxB + dyB * dyA;
            dist_squared_a.cmp(&dist_squared_b)
        });

        for element in elements {
            let data = self.chart_instance.data.datasets[element.dataset_index].data[element.index];
            if data.is_none() || data.unwrap() == 0.0_f64 {
                continue;
            }

            out.push(HoverElement { data });
        }

        out
    }

    pub fn update_datasets(&mut self, datasets: Vec<Dataset>) -> Option<Scale> {
        self.chart_instance.data.datasets = datasets;

        // While the chartjs API doesn't indicate update should be called after resize, in practice
        // we've found that performing a resize after an update sometimes results in a blank chart.
        //
        // NOTE: "none" disables animations - this is important for chart performance because we update
        // the entire data set which does not preserve history for the chart animations
        self.chart_instance.update("none");
        Some(self.get_x_scale())
    }

    fn get_x_scale(&self) -> Option<Scale> {
        if let Some(scales) = &self.chart_instance.options.scales {
            scales.get("x").map(|scale| {
                Scale {
                    min: scale.min(),
                    max: scale.max(),
                    left: scale.left(),
                    right: scale.right(),
                }
            })
        } else {
            None
        }
    }

    fn apply_interaction_event(&self, event: &InteractionEvent) {
        match event.type.as_ref() {
            "wheel" => self.on_wheel_event.emit(event),
            "panstart" => maybe_cast<ZoomableChart>(&self.chart_instance)?.$zoom.pan_start_handler(event),
            "panmove" => maybe_cast<ZoomableChart>(&self.chart_instance)?.$zoom.pan_handler(event),
            "panend" => maybe_cast<ZoomableChart>(&self.chart_instance)?.$zoom.pan_end_handler(),
            _ => (),
        }
    }

    fn on_wheel_event(&mut self, event: &web_sys::WheelEvent) {
        self.fake_node_events.emit("wheel", event);
    }

    fn on_pan_start_event(&mut self, event: &web_sys::MouseEvent) {
        let bounding_client_rect = event.target().bounding_client_rect();
        maybe_cast<ZoomableChart>(&self.chart_instance)?.$zoom.pan_start_handler({
            center: web_sys::Point { x: event.offset_x(), y: event.offset_y() },
            deltaX: 0.0,
            deltaY: 0.0,
            target: web_sys::NodeBox {
                get_bounding_client_rect: || bounding_client_rect,
            },
        });
    }

    fn on_pan_move_event(&mut self, event: &web_sys::MouseEvent) {
        let bounding_client_rect = event.target().bounding_client_rect();
        maybe_cast<ZoomableChart>(&self.chart_instance)?.$zoom.pan_handler({
            center: web_sys::Point { x: event.offset_x(), y: event.offset_y() },
        });
    }

    fn on_pan_end_event(&mut self, _: &web_sys::MouseEvent) {
        maybe_cast<ZoomableChart>(&self.chart_instance)?.$zoom.pan_end_handler();
    }
}
```