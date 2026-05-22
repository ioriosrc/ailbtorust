```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::{HashMap};

#[derive(Clone)]
struct CurrentCustomDatasetsBuilder {
    x_values: Vec<f64>,
    series_by_key: HashMap<SeriesConfigKey, CurrentFrameSeriesItem>,
}

impl CurrentCustomDatasetsBuilder {
    fn handle_player_state(&mut self, state: &PlayerState) -> Option<HandlePlayerStateResult> {
        if !state.active_data.is_some() || !self.x_parsed_path.is_some() {
            return None;
        }

        let active_data = state.active_data.as_ref()?;
        let msg_events = active_data.messages;

        if msg_events.is_empty() {
            return None;
        }

        let mut datasets_changed = false;
        {
            // Process the latest messages from the player state to extract any updated x or y values
            //
            // Datasets are built when y-values arrive though this could be expanded to also build
            // when x-values arrive.
            let mut x_values = Vec::new();

            let msg_event = last_matching_topic(msg_events, &self.x_parsed_path.topic_name);
            if let Some(msg_event) = msg_event {
                for item in simple_get_message_path_data_items(&msg_event, &self.x_parsed_path) {
                    if !is_chart_value(&item) {
                        continue;
                    }

                    let chart_value = get_chart_value(&item);

                    if chart_value.is_none() {
                        continue;
                    }

                    x_values.push(self.x_parse_fn(chart_value));
                }
            }

            datasets_changed |= x_values.len() > 0;
            self.x_values = x_values;
        }

        for series in self.series_by_key.values_mut() {
            let mut math_fn = None;

            if let Some(parsed) = &series.parsed {
                match parsed.modifier.as_deref() {
                    Some("identity") => {}
                    _ => math_fn = Some(math_functions.get(parsed.modifier.as_str())?),
                }
            }

            let msg_event = last_matching_topic(msg_events, &series.parsed.topic_name);
            if let Some(msg_event) = msg_event {
                for item in simple_get_message_path_data_items(&msg_event, &series.parsed) {
                    if !is_chart_value(&item) {
                        continue;
                    }

                    let chart_value = get_chart_value(&item);

                    let math_modified_value = math_fn.map(|fn| fn(chart_value));

                    series.dataset.data.push({
                        x: self.x_values.iter().next().copied(),
                        y: chart_value.is_none()
                            .then(|| NaN)
                            .unwrap_or_else(|| (math_modified_value.unwrap_or(chart_value)) as f64),
                        receive_time: msg_event.receive_time,
                        value: math_modified_value.or(Some(item)),
                    });
                }
            }

            if series.dataset.data.len() == self.x_values.len() {
                self.path_with_mismatched_data_lengths.remove(&series.message_path);
            } else {
                self.path_with_mismatched_data_lengths.insert(series.message_path);
            }
        }

        Some({
            range: None,
            datasets_changed,
        })
    }

    fn set_xpath(&mut self, path: &MessagePath) {
        if !self.x_parsed_path.is_some() || path.topic_name == self.x_parsed_path.topic_name {
            return;
        }

        self.x_parsed_path = path;
        for series in &mut self.series_by_key {
            series.dataset.data.clear();
        }
        self.path_with_mismatched_data_lengths.clear();
    }

    fn set_series(&mut self, series: SeriesItem[]) {
        let mut new_series_map = HashMap::new();
        for series_item in series {
            new_series_map.insert(series_item.message_path.clone(), series_item);
        }

        for (key, item) in &mut self.series_by_key {
            if new_series_map.contains_key(&item.message_path) {
                *item = new_series_map.get(&item.message_path).unwrap().clone();
            }
        }
    }

    // We don't use the viewport because we do not do any downsampling on the assumption that
    // one message won't produce so many points that we need to downsample.
    //
    // If that assumption changes then downsampling can be revisited.
    async fn get_viewport_datasets(&self) -> GetViewportDatasetsResult {
        build_viewport_datasets(&self.series_by_key, &self.path_with_mismatched_data_lengths)
    }

    async fn get_csv_data(&self) -> CsvDataset[] {
        let mut datasets: Vec<CsvDataset> = Vec::new();

        for series in &self.series_by_key {
            if !series.enabled {
                continue;
            }

            datasets.push({
                label: series.message_path.clone(),
                data: series.dataset.data.iter().map(|x| x.value as f64).collect::<Vec<f64>>(),
            });
        }

        datasets
    }
}

fn main() {
    // Example usage
}
```