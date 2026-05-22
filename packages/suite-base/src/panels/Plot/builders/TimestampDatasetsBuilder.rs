```rust
use std::sync::{Arc, Mutex};
use parking_lot::MutexGuard;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use crate::types::{ComlinkRemote, ComlinkRemoteObject, UpdateDataAction};
use crate::util::math_functions::MATH_FUNCTIONS;
use crate::data_items::Datum;
use crate::math_functions::get_chart_value;
use crate::math_functions::is_chart_value;

// If the datasets builder is garbage collected we also need to cleanup the worker
// This registry ensures the worker is cleaned up when the builder is garbage collected
struct TimestampDatasetsBuilderCleanup {
    dispose: Arc<dyn Fn()>,
}

impl TimestampDatasetsBuilderCleanup {
    fn new(dispose: impl Fn() + 'static) -> Self {
        Self { dispose: Arc::new(dispose) }
    }

    fn dispose(&self) {
        (self.dispose)();
    }
}

type TimestampSeriesItem = crate::data_items::TimestampSeriesItem;

/**
 * TimestampDatasetsBuilder builds timeseries datasets.
 *
 * It supports full (preload) data and current frame data. The series datums are extracted from
 * input player states and sent to the worker. The worker accumulates the data and provides
 * downsampled data.
 */
pub struct TimestampDatasetsBuilder {
    datasets_builder_remote: ComlinkRemote<ComlinkRemoteObject<TimestampDatasetsBuilderImpl>>,

    pending_dispatch: Mutex<Vec<UpdateDataAction>>,
    last_seek_time: i64,
    has_range_source: bool,

    series: Arc<Mutex<Vec<Crate::data_items::TimestampSeriesItem>>>,
}

impl TimestampDatasetsBuilder {
    pub fn new() -> Self {
        let worker = std::thread::spawn(move || {
            crate::worker::run();
        });

        let cleanup = TimestampDatasetsBuilderCleanup::new(|| {
            worker.join().unwrap();
        });

        let remote = ComlinkWrap::new(worker);
        let datasets_builder_remote = remote;

        Arc::new(TimestampDatasetsBuilder {
            datasets_builder_remote,
            pending_dispatch: Mutex::new(Vec::new()),
            last_seek_time: 0,
            has_range_source: false,
            series: Arc::new(Mutex::new(vec![])),
        })
    }

    pub fn handle_player_state(&self, state: crate::data_items::PlayerState) -> Option<crate::dataset_builders::HandlePlayerStateResult> {
        let active_data = state.active_data;
        if !active_data.is_empty() {
            let did_seek = active_data.last_seek_time != self.last_seek_time;
            self.last_seek_time = active_data.last_seek_time;

            let msg_events = active_data.messages;
            let mut datasets_changed = false;
            if !self.has_range_source && !msg_events.is_empty() {
                let (series_actions, series_changed) = crate::dataset_builders::build_current_series_actions(
                    self.series.lock().unwrap(),
                    did_seek,
                    |config| {
                        match config.parsed.modifier {
                            Some(modifier) => MATH_FUNCTIONS.contains_key(&modifier),
                            None => true,
                        }
                    },
                );

                self.pending_dispatch.lock().unwrap().extend(series_actions.iter().cloned());
                datasets_changed ||= series_changed;
            }

            return Some({
                range: { min: 0, max: crate::utils::to_sec(crate::timestamp::subtract_time(active_data.endTime, active_data.startTime)) },
                datasets_changed,
            });
        }

        None
    }

    pub fn handle_message_range(&self, messages: Vec<crate::data_items::MessageEvent>, options: { is_reset: bool }, start_time: crate::data_items::Time) {
        self.has_range_source = true;
        let topic = messages[0].topic.as_ref();
        if topic.is_none() {
            return;
        }

        let actions = crate::dataset_builders::build_full_series_actions(
            self.series.lock().unwrap(),
            topic,
            options,
            |config| {
                match config.parsed.modifier {
                    Some(modifier) => MATH_FUNCTIONS.contains_key(&modifier),
                    None => true,
                }
            },
        );

        self.pending_dispatch.lock().unwrap().extend(actions.iter().cloned());
    }

    pub fn set_series(&self, series: Vec<crate::data_items::TimestampSeriesItem>) {
        *self.series.lock().unwrap() = series;
        self.pending_dispatch.lock().unwrap().push({
            type: "update-series-config",
            series_items: series,
        });
    }

    pub async fn get_viewport_datasets(&self, viewport: crate::data_items::Viewport) -> Result<crate::dataset_builders::GetViewportDatasetsResult, Box<dyn std::error::Error>> {
        let dispatch = self.pending_dispatch.lock().unwrap();
        if !dispatch.is_empty() {
            self.pending_dispatch.lock().unwrap().clear();
            await self.datasets_builder_remote.apply_actions(dispatch);
        }

        let datasets = self.datasets_builder_remote.get_viewport_datasets(viewport).await?;
        Ok({
            datasets_by_config_index: datasets,
            paths_with_mismatched_data_lengths: Vec::new(),
        })
    }

    pub async fn get_csv_data(&self) -> Result<Vec<crate::data_items::CsvDataset>, Box<dyn std::error::Error>> {
        let dispatch = self.pending_dispatch.lock().unwrap();
        if !dispatch.is_empty() {
            self.pending_dispatch.lock().unwrap().clear();
            await self.datasets_builder_remote.apply_actions(dispatch);
        }

        let datasets = self.datasets_builder_remote.get_csv_data().await?;
        Ok(datasets)
    }
}

fn read_message_path_items(
    events: Vec<crate::data_items::MessageEvent>,
    path: crate::data_items::MessagePath,
    timestamp_method: crate::utils::TimestampMethod,
    start_time: crate::data_items::Time,
    math_function: Option<&dyn Fn(f64) -> f64>,
) -> Result<Vec<crate::data_items::Datum>, Box<dyn std::error::Error>> {
    let mut out = Vec::new();
    for event in events {
        if event.topic != path.topic_name.as_ref() {
            continue;
        }

        let items = crate::utils::simple_get_message_path_data_items(event, &path);
        for item in items {
            if !crate::data_items::is_chart_value(item) {
                continue;
            }
            let chart_value = crate::data_items::get_chart_value(item)?;
            if chart_value.is_none() {
                continue;
            }

            let header_stamp = crate::utils::get_timestamp_for_message(event.message);
            let timestamp = if timestamp_method == "receive_time" { event.receive_time } else { header_stamp };
            if !timestamp.is_some() {
                continue;
            }

            let x_value = crate::utils::to_sec(crate::timestamp::subtract_time(timestamp.as_ref(), &start_time));
            let math_modified = match math_function {
                Some(func) => func(chart_value.unwrap()),
                None => chart_value.unwrap(),
            };
            out.push({
                x: x_value,
                y: math_modified,
                receive_time: event.receive_time,
                header_stamp,
                value: chart_value.unwrap(),
            });
        }
    }

    Ok(out)
}
```