// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! State Transitions Panel - displays categorical state blocks over time.
//! Shows finite state machine transitions as colored horizontal bars.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::state::app_state::{get_player, use_app_state};

/// A state transition entry.
#[derive(Clone, Debug)]
struct StateEntry {
    time_ns: u64,
    label: String,
}

/// Extract state transitions from message data (attempts to read string value).
fn extract_state_from_message(data: &[u8], encoding: &str) -> Option<String> {
    if data.len() < 8 {
        return None;
    }

    let offset = if encoding == "cdr" || encoding == "CDR" {
        4
    } else {
        0
    };

    let d = &data[offset..];

    // Try to read as a simple string message (std_msgs/String)
    if d.len() < 4 {
        return None;
    }
    let str_len = u32::from_le_bytes([d[0], d[1], d[2], d[3]]) as usize;

    if str_len > 0 && str_len < 256 && d.len() >= 4 + str_len {
        let s = String::from_utf8_lossy(&d[4..4 + str_len.saturating_sub(1)]).to_string();
        if !s.is_empty() && s.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
            return Some(s);
        }
    }

    // Try extracting as enum (u8/u32 value)
    if d.len() >= 4 {
        let val = u32::from_le_bytes([d[0], d[1], d[2], d[3]]);
        if val < 100 {
            return Some(format!("State_{}", val));
        }
    }

    None
}

/// Get a color for a state label (deterministic hash-based).
fn state_color(label: &str) -> (f64, f64, f64) {
    let hash: u32 = label.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
    let hue: f64 = (hash % 360) as f64;
    let saturation: f64 = 0.6;
    let lightness: f64 = 0.5;

    // HSL to RGB
    let c: f64 = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x: f64 = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m: f64 = lightness - c / 2.0;

    let (r, g, b) = match hue as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (r + m, g + m, b + m)
}

/// State Transitions panel component.
#[component]
pub fn StateTransitionsPanel(#[prop(into)] topic: String) -> impl IntoView {
    let state = use_app_state();
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let topic_owned = topic.clone();

    // Render on frame tick
    Effect::new(move |_| {
        let _tick = state.frame_tick.get();

        let Some(canvas_el) = canvas_ref.get() else {
            return;
        };
        let canvas: HtmlCanvasElement = canvas_el.into();

        let parent = canvas.parent_element().unwrap();
        let w = parent.client_width().max(100) as u32;
        let h = parent.client_height().max(50) as u32;
        canvas.set_width(w);
        canvas.set_height(h);

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // Clear
        ctx.set_fill_style_str("#1e1e2e");
        ctx.fill_rect(0.0, 0.0, w as f64, h as f64);

        let Some(player) = get_player() else {
            return;
        };

        let topics = player.topics();
        let topic_info = topics.iter().find(|t| t.name == topic_owned);
        if topic_info.is_none() {
            ctx.set_fill_style_str("#888");
            ctx.set_font("12px monospace");
            ctx.fill_text("No state data", 10.0, h as f64 / 2.0).ok();
            return;
        }

        // Collect all state transitions from messages
        let (start_time, end_time) = player.time_range();
        let duration = (end_time - start_time) as f64;
        if duration <= 0.0 {
            return;
        }

        // Extract states from messages on this topic
        let all_msgs = player.get_topic_messages_until_now(&topic_owned);
        let mut entries: Vec<StateEntry> = Vec::new();
        for msg in &all_msgs {
            if let Some(label) = extract_state_from_message(&msg.data, &msg.encoding) {
                entries.push(StateEntry {
                    time_ns: msg.log_time_ns,
                    label,
                });
            }
        }

        if entries.is_empty() {
            ctx.set_fill_style_str("#888");
            ctx.set_font("12px monospace");
            ctx.fill_text("No state transitions found", 10.0, h as f64 / 2.0).ok();
            return;
        }

        // Draw state blocks
        let bar_height = (h as f64 - 30.0).max(20.0);
        let bar_y = 20.0;

        for i in 0..entries.len() {
            let entry = &entries[i];
            let x_start = ((entry.time_ns - start_time) as f64 / duration) * w as f64;
            let x_end = if i + 1 < entries.len() {
                ((entries[i + 1].time_ns - start_time) as f64 / duration) * w as f64
            } else {
                w as f64
            };

            let (r, g, b) = state_color(&entry.label);
            ctx.set_fill_style_str(&format!(
                "rgb({},{},{})",
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8
            ));
            ctx.fill_rect(x_start, bar_y, (x_end - x_start).max(1.0), bar_height);

            // Draw label if block is wide enough
            let block_width = x_end - x_start;
            if block_width > 40.0 {
                ctx.set_fill_style_str("#fff");
                ctx.set_font("11px sans-serif");
                ctx.set_text_align("center");
                let label_x = x_start + block_width / 2.0;
                ctx.fill_text(&entry.label, label_x, bar_y + bar_height / 2.0 + 4.0).ok();
            }
        }

        // Draw current time indicator
        let current_time = player.current_time_ns();
        let cursor_x = ((current_time - start_time) as f64 / duration) * w as f64;
        ctx.set_stroke_style_str("#ff4444");
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.move_to(cursor_x, 0.0);
        ctx.line_to(cursor_x, h as f64);
        ctx.stroke();

        // Draw topic name
        ctx.set_fill_style_str("#ccc");
        ctx.set_font("11px sans-serif");
        ctx.set_text_align("left");
        ctx.fill_text(&topic_owned, 4.0, 14.0).ok();
    });

    view! {
        <div class="panel-state-transitions-content">
            <canvas node_ref=canvas_ref class="state-transitions-canvas" />
        </div>
    }
}
