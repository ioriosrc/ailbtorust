```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

mod index_datasets_builder;
use index_datasets_builder::IndexDatasetsBuilder;

use message_path::{MessagePath, ParseMessagePathError};
use suite_base::players::{PlayerPresence, PlayerStateActiveData};

struct SeriesItem {
    config_index: usize,
    parsed: MessagePath,
    color: &'static str,
    contrast_color: &'static str,
    enabled: bool,
    timestamp_method: &'static str,
    key: &'static str,
    line_size: f64,
    message_path: &'static str,
    show_line: bool,
}

fn build_series_items(paths: Vec<(MessagePath, Option<bool>)>) -> Vec<SeriesItem> {
    paths
        .into_iter()
        .map(|(item, enabled)| {
            let key = item.to_string();
            SeriesItem {
                config_index: 0,
                parsed: item,
                color: "red",
                contrast_color: "blue",
                enabled: enabled.unwrap_or(true),
                timestamp_method: "receiveTime",
                key,
                line_size: 1.2,
                message_path: item.to_string(),
                show_line: true,
            }
        })
        .collect()
}

fn build_player_state(active_data_override: Option<PlayerStateActiveData>) -> PlayerState {
    PlayerState {
        active_data: PlayerStateActiveData {
            messages: vec![MessagePath::from("/bar.val")],
            current_time: { sec: 0, nsec: 0 },
            end_time: { sec: 0, nsec: 0 },
            last_seek_time: 1,
            topics: vec!["/foo".to_string(), "/bar".to_string()],
            speed: 1.0,
            is_playing: false,
            topic_stats: HashMap::new(),
            startTime: { sec: 0, nsec: 0 },
            datatypes: HashMap::new(),
            total_bytes_received: 0,
            ..active_data_override.unwrap_or_default()
        },
        capabilities: Vec::new(),
        presence: PlayerPresence::PRESENT,
        profile: None,
        playerId: "1",
        progress: {
            fully_loaded_fraction_ranges: Vec::new(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_series_items() {
        let series_items = build_series_items(vec![
            (MessagePath::from("/bar.val"), Some(true)),
            (MessagePath::from("/bar.val[:]"), None),
        ]);

        assert_eq!(
            series_items,
            vec![
                SeriesItem {
                    config_index: 0,
                    parsed: MessagePath::from("/bar.val"),
                    color: "red",
                    contrast_color: "blue",
                    enabled: true,
                    timestamp_method: "receiveTime",
                    key: "/bar.val".to_string(),
                    line_size: 1.2,
                    message_path: "/bar.val".to_string(),
                    show_line: true,
                },
                SeriesItem {
                    config_index: 0,
                    parsed: MessagePath::from("/bar.val[:]"),
                    color: "red",
                    contrast_color: "blue",
                    enabled: false,
                    timestamp_method: "receiveTime",
                    key: "/bar.val[:]".to_string(),
                    line_size: 1.2,
                    message_path: "/bar.val[:]".to_string(),
                    show_line: true,
                },
            ]
        );
    }

    #[test]
    async fn test_build_player_state() {
        let player_state = build_player_state(None);

        assert_eq!(
            player_state.active_data.messages,
            vec![MessagePath::from("/bar.val")],
        );

        assert_eq!(player_state, PlayerState {
            active_data: PlayerStateActiveData {
                messages: vec![MessagePath::from("/bar.val")],
                current_time: { sec: 0, nsec: 0 },
                end_time: { sec: 0, nsec: 0 },
                last_seek_time: 1,
                topics: vec!["/foo".to_string(), "/bar".to_string()],
                speed: 1.0,
                is_playing: false,
                topic_stats: HashMap::new(),
                startTime: { sec: 0, nsec: 0 },
                datatypes: HashMap::new(),
                total_bytes_received: 0,
            },
            capabilities: Vec::new(),
            presence: PlayerPresence::PRESENT,
            profile: None,
            playerId: "1",
            progress: {
                fully_loaded_fraction_ranges: Vec::new(),
            },
        });
    }
}
```