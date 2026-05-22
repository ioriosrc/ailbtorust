// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Plot panel - displays time-series numeric data.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::state::app_state::{get_player, use_app_state};

/// Simple plot panel - shows numeric data over time using canvas.
#[component]
pub fn PlotPanel(#[prop(into)] topic: String) -> impl IntoView {
    let state = use_app_state();
    let frame_tick = state.frame_tick;
    let topic_clone = topic.clone();
    let display_topic = topic.clone();

    // Store data points: (relative_time_s, value)
    let data_points = RwSignal::new(Vec::<(f64, f64)>::new());
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    // Collect data points
    let topic_for_effect = topic_clone.clone();
    Effect::new(move |_| {
        let _tick = frame_tick.get();

        let player = match get_player() {
            Some(p) => p,
            None => return,
        };

        let msg = match player.get_current_message(&topic_for_effect) {
            Some(m) => m,
            None => return,
        };

        // Try to extract a numeric value
        let value = extract_numeric_value(&msg.data, &msg.encoding);
        if let Some(val) = value {
            let time_s = msg.log_time_ns as f64 / 1_000_000_000.0;
            data_points.update(|pts| {
                pts.push((time_s, val));
                // Keep last 500 points
                if pts.len() > 500 {
                    pts.drain(0..pts.len() - 500);
                }
            });
        }
    });

    // Render the plot on canvas
    Effect::new(move |_| {
        let pts = data_points.get();
        if pts.is_empty() {
            return;
        }

        let canvas_el = match canvas_ref.get() {
            Some(el) => el,
            None => return,
        };

        let canvas: &web_sys::HtmlCanvasElement = canvas_el.as_ref();
        let width = canvas.client_width() as u32;
        let height = canvas.client_height() as u32;
        if width == 0 || height == 0 {
            return;
        }
        canvas.set_width(width);
        canvas.set_height(height);

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // Clear
        ctx.set_fill_style_str("#1e1e2e");
        ctx.fill_rect(0.0, 0.0, width as f64, height as f64);

        if pts.len() < 2 {
            return;
        }

        // Find bounds
        let min_t = pts.first().unwrap().0;
        let max_t = pts.last().unwrap().0;
        let min_v = pts.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
        let max_v = pts.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);

        let t_range = (max_t - min_t).max(0.001);
        let v_range = (max_v - min_v).max(0.001);

        let pad = 20.0;
        let plot_w = width as f64 - pad * 2.0;
        let plot_h = height as f64 - pad * 2.0;

        // Draw grid lines
        ctx.set_stroke_style_str("#333344");
        ctx.set_line_width(0.5);
        for i in 0..5 {
            let y = pad + plot_h * (i as f64 / 4.0);
            ctx.begin_path();
            ctx.move_to(pad, y);
            ctx.line_to(pad + plot_w, y);
            ctx.stroke();
        }

        // Draw data line
        ctx.set_stroke_style_str("#89b4fa");
        ctx.set_line_width(1.5);
        ctx.begin_path();

        for (i, (t, v)) in pts.iter().enumerate() {
            let x = pad + (t - min_t) / t_range * plot_w;
            let y = pad + plot_h - (v - min_v) / v_range * plot_h;

            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();

        // Draw current value text
        if let Some(last) = pts.last() {
            ctx.set_fill_style_str("#cdd6f4");
            ctx.set_font("11px monospace");
            let text = format!("{:.4}", last.1);
            ctx.fill_text(&text, pad + 4.0, pad + 12.0).ok();
        }
    });

    view! {
        <div class="panel-container plot-panel">
            <div class="panel-toolbar">
                <span class="panel-title">{"Plot"}</span>
                <span class="panel-topic">{display_topic}</span>
            </div>
            <div class="panel-content plot-panel-content">
                <canvas node_ref=canvas_ref class="plot-canvas"></canvas>
            </div>
        </div>
    }
}

/// Extract a numeric value from message data.
fn extract_numeric_value(data: &[u8], encoding: &str) -> Option<f64> {
    let offset = match encoding {
        "cdr" => 4, // skip CDR header
        _ => 0,
    };

    if data.len() < offset + 8 {
        if data.len() >= offset + 4 {
            // Try as f32
            let bytes = &data[offset..offset + 4];
            let val = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            if val.is_finite() {
                return Some(val as f64);
            }
        }
        return None;
    }

    // Try as f64
    let bytes = &data[offset..offset + 8];
    let val = f64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]);
    if val.is_finite() && val.abs() < 1e15 {
        Some(val)
    } else {
        // Try as f32
        let bytes = &data[offset..offset + 4];
        let val = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if val.is_finite() {
            Some(val as f64)
        } else {
            None
        }
    }
}
