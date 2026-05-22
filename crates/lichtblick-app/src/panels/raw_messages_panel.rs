// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Raw Messages panel - displays the raw bytes/fields of messages on a topic.

use leptos::prelude::*;

use crate::state::app_state::{get_player, use_app_state};

/// Raw Messages panel - shows latest message data for a topic.
#[component]
pub fn RawMessagesPanel(#[prop(into)] topic: String) -> impl IntoView {
    let state = use_app_state();
    let frame_tick = state.frame_tick;
    let topic_clone = topic.clone();
    let display_topic = topic.clone();

    let message_info = RwSignal::new(String::new());
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

        // Format message info
        let time_secs = msg.log_time_ns as f64 / 1_000_000_000.0;
        let size = msg.data.len();

        let mut info = format!(
            "Topic: {}\nSchema: {}\nEncoding: {}\nTime: {:.6}s\nSize: {} bytes\n\n",
            msg.topic, msg.schema_name, msg.encoding, time_secs, size
        );

        // Show hex dump of first 256 bytes
        info.push_str("Data (hex):\n");
        let show_len = size.min(256);
        for (i, chunk) in msg.data[..show_len].chunks(16).enumerate() {
            info.push_str(&format!("{:04x}: ", i * 16));
            for byte in chunk {
                info.push_str(&format!("{:02x} ", byte));
            }
            // ASCII representation
            info.push_str(" |");
            for byte in chunk {
                let c = if *byte >= 0x20 && *byte < 0x7f {
                    *byte as char
                } else {
                    '.'
                };
                info.push(c);
            }
            info.push_str("|\n");
        }
        if size > 256 {
            info.push_str(&format!("... ({} more bytes)\n", size - 256));
        }

        message_info.set(info);
    });

    view! {
        <div class="panel-container raw-messages-panel">
            <div class="panel-toolbar">
                <span class="panel-title">{"Raw Messages"}</span>
                <span class="panel-topic">{display_topic}</span>
            </div>
            <div class="panel-content raw-messages-content">
                <pre class="raw-message-text">{move || message_info.get()}</pre>
            </div>
        </div>
    }
}
