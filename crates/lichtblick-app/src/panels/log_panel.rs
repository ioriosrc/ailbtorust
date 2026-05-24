// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Log panel - displays messages from /rosout or similar log topics.

use leptos::prelude::*;

use crate::state::app_state::{get_player, use_app_state};

/// Log panel - shows log messages (rosout).
#[component]
pub fn LogPanel(#[prop(into)] topic: String) -> impl IntoView {
    let state = use_app_state();
    let frame_tick = state.frame_tick;
    let topic_clone = topic.clone();

    let log_entries = RwSignal::new(Vec::<String>::new());
    let last_time = RwSignal::new(0u64);

    Effect::new(move |_| {
        let _tick = frame_tick.get();

        let player = match get_player() {
            Some(p) => p,
            None => return,
        };

        let msg = match player.get_current_message(&topic_clone) {
            Some(m) => m,
            None => return,
        };

        if msg.log_time_ns == last_time.get_untracked() {
            return;
        }
        last_time.set(msg.log_time_ns);

        // Try to extract text from the message
        // For rosout, the message field is a string after header + level fields
        let time_secs = msg.log_time_ns as f64 / 1_000_000_000.0;
        let text = extract_log_text(&msg.data, &msg.encoding);

        let entry = format!("[{:.3}] {}", time_secs, text);

        log_entries.update(|entries| {
            entries.push(entry);
            // Keep last 100 entries
            if entries.len() > 100 {
                entries.drain(0..entries.len() - 100);
            }
        });
    });

    view! {
        <div class="log-panel-content">
            <div class="log-entries">
                {move || log_entries.get().into_iter().map(|entry| {
                    view! { <div class="log-entry">{entry}</div> }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Try to extract readable text from a log message.
fn extract_log_text(data: &[u8], encoding: &str) -> String {
    // Simple heuristic: find longest ASCII string in the data
    let start = match encoding {
        "cdr" => 4,
        _ => 0,
    };

    if start >= data.len() {
        return "(empty)".to_string();
    }

    // Look for printable ASCII sequences
    let mut best_start = start;
    let mut best_len = 0;
    let mut cur_start = start;
    let mut cur_len = 0;

    for i in start..data.len() {
        if data[i] >= 0x20 && data[i] < 0x7f {
            if cur_len == 0 {
                cur_start = i;
            }
            cur_len += 1;
        } else {
            if cur_len > best_len && cur_len > 4 {
                best_start = cur_start;
                best_len = cur_len;
            }
            cur_len = 0;
        }
    }
    if cur_len > best_len && cur_len > 4 {
        best_start = cur_start;
        best_len = cur_len;
    }

    if best_len > 0 {
        String::from_utf8_lossy(&data[best_start..best_start + best_len]).to_string()
    } else {
        format!("({} bytes)", data.len())
    }
}
