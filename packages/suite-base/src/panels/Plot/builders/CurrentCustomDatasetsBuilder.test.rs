```rust
use std::collections::HashSet;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use lichtblick_monads::{unwrap};
use lichtblick_message_path::parse_message_path;
use lichtblick_suite_base_players::players::types::*;
use current_custom_datasets_builder::CurrentCustomDatasetsBuilder;
use idatasets_builder::{PlotPath, SeriesItem};
use utils::config::ViewportDatasets;

fn build_series_items(
    paths: Vec<(&str, &str)>,
) -> Vec<SeriesItem> {
    paths
        .iter()
        .map(|(key, value)| {
            let parsed = parse_message_path(value).unwrap();
            SeriesItem {
                config_index: *key.parse().unwrap(),
                parsed,
                color: "red",
                contrast_color: "blue",
                enabled: true,
                timestamp_method: "receiveTime",
                key: key.to_string(),
                line_size: 1,
                message_path: value.to_string(),
                show_line: true,
            }
        })
        .collect()
}

fn build_player_state(active_data_override: Option<PlayerStateActiveData>) -> PlayerState {
    PlayerState {
        active_data: {
            messages: vec![
                Message {
                    topic: "/foo".to_string(),
                    schema_name: "foo".to_string(),
                    receive_time: { sec: 0, nsec: 0 },
                    size_in_bytes: 0,
                    message: Value { val: 4 },
                },
                Message {
                    topic: "/bar".to_string(),
                    schema_name: "foo".to_string(),
                    receive_time: { sec: 0, nsec: 0 },
                    size_in_bytes: 0,
                    message: Value { val: -3 },
                },
            ],
            currentTime: { sec: 0, nsec: 0 },
            endTime: { sec: 0, nsec: 0 },
            last_seek_time: 1,
            topics: vec![],
            speed: 1,
            is_playing: false,
            topic_stats: std::collections::HashMap::new(),
            startTime: { sec: 0, nsec: 0 },
            datatypes: std::collections::HashMap::new(),
            total_bytes_received: 0,
            ...active_data_override,
        },
        capabilities: vec![],
        presence: PlayerPresence::PRESENT,
        profile: None,
        playerId: "1",
        progress: {
            fully_loaded_fraction_ranges: vec![],
        },
    }
}

mod current_custom_datasets_builder {
    pub struct CurrentCustomDatasetsBuilder;

    impl CurrentCustomDatasetsBuilder {
        pub fn new() -> Self {
            Self {}
        }

        pub fn set_x_path(&mut self, path: &str) {
            // Implementation
        }

        pub fn set_series(&mut self, series: Vec<SeriesItem>) {
            // Implementation
        }

        pub async fn handle_player_state(
            &self,
            player_state: PlayerState,
        ) -> ViewportDatasets {
            // Implementation
        }
    }
}

mod idatasets_builder {
    pub struct PlotPath;

    pub struct SeriesItem {
        config_index: usize,
        parsed: PlotPath,
        color: &'static str,
        contrast_color: &'static str,
        enabled: bool,
        timestamp_method: &'static str,
        key: String,
        line_size: f64,
        message_path: String,
        show_line: bool,
    }
}

mod utils {
    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }

    pub enum PlotPath {
        // Implementation
    }
}

mod current_custom_datasets_builder {
    use super::{build_series_items, build_player_state};

    impl CurrentCustomDatasetsBuilder {
        pub async fn get_viewport_datasets(
            &self,
        ) -> ViewportDatasets {
            let paths = vec![("/foo.val", "/bar.val")];
            let series = build_series_items(paths);
            let player_state = build_player_state(None);
            let viewport_datasets = handle_player_state(player_state);

            viewport_datasets
        }
    }
}

mod idatasets_builder {
    use super::SeriesItem;

    pub struct ViewportDatasets {
        paths_with_mismatched_data_lengths: HashSet<&str>,
        datasets_by_config_index: Vec<Option<SeriesItem>>,
    }

    impl ViewportDatasets {
        fn new() -> Self {
            Self {
                paths_with_mismatched_data_lengths: HashSet::new(),
                datasets_by_config_index: vec![None; 0],
            }
        }

        fn add_dataset(
            &mut self,
            series_item: Option<SeriesItem>,
        ) {
            if let Some(series_item) = series_item {
                self.datasets_by_config_index.push(Some(series_item));
            }
        }

        fn update_paths_with_mismatched_data_lengths(&mut self, paths: Vec<&str>) {
            // Implementation
        }
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value {
        val: i32,
    }

    pub struct Message {
        topic: String,
        schema_name: String,
        receive_time: { sec: u32, nsec: u32 },
        size_in_bytes: u64,
        message: Value,
    }
}

mod utils {
    use super::{Value, Message};

    pub struct Value