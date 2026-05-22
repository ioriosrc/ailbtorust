```rust
use std::sync::{Arc, RwLock};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use log::info;

use crate::{
    dataset::{CustomDatasetsSeriesItem, CsvDataset},
    message_path::MessagePath,
    player_state::PlayerState,
    utils::datum::{get_chart_value, is_chart_value},
    utils::math_functions as math,
};

type CustomDatasetsBuilder = Arc<RwLock<dyn IDatasetsBuilderImpl>>;

#[derive(Debug)]
pub struct CustomDatasetsBuilderImpl {
    datasets_builder_remote: Comlink.Remote<Comlink.RemoteObject<Self>>,
    x_parsed_path: Option<MessagePath>,
    pending_dispatch: Vec<UpdateDataAction>,
    last_seek_time: u64,
    series: Vec<CustomDatasetsSeriesItem>,
    x_current_bounds: Option<Bounds1D>,
    x_full_bounds: Option<Bounds1D>,
    has_range_source: bool,
}

impl CustomDatasetsBuilder {
    pub fn new() -> Self {
        let worker = Comlink::from_worker(
            std::fs::File::open("./CustomDatasetsBuilderImpl.worker").unwrap(),
        );
        let datasets_builder_remote = Comlink::wrap(worker).unwrap();

        Arc::new(RwLock::new(CustomDatasetsBuilderImpl {
            datasets_builder_remote,
            x_parsed_path: None,
            pending_dispatch: Vec::new(),
            last_seek_time: 0,
            series: vec![],
            x_current_bounds: None,
            x_full_bounds: None,
            has_range_source: false,
        }))
    }

    pub fn handle_player_state(&self, state: PlayerState) -> Option<HandlePlayerStateResult> {
        let active_data = state.active_data;
        if active_data.is_none() {
            return None;
        }

        let did_seek = active_data.last_seek_time != self.last_seek_time;
        self.last_seek_time = active_data.last_seek_time;

        let mut datasets_changed = false;

        if !self.has_range_source && !active_data.messages.is_empty() {
            if did_seek {
                self.pending_dispatch.push(UpdateDataAction::ResetCurrentX);
                self.x_current_bounds = None;
            }

            // Read the x-axis values
            if let Some(x_parsed_path) = &self.x_parsed_path {
                let path_items = parse_xpath_items(&active_data.messages, x_parsed_path);

                self.pending_dispatch.push(
                    UpdateDataAction::AppendCurrentX(path_items.clone()),
                );

                if !path_items.is_empty() {
                    datasets_changed = true;
                    self.x_current_bounds = compute_bounds(self.x_current_bounds, &path_items);
                }
            }

            let (series_actions, series_changed) = build_current_series_actions(
                &self.series,
                did_seek,
                |config| match config.parsed.modifier {
                    Some(modifier) => Some(math_functions[modifier]),
                    None => None,
                },
            );
            self.pending_dispatch.push(
                series_actions
                    .iter()
                    .cloned()
                    .collect::<Vec<UpdateDataAction>>(),
            );

            datasets_changed ||= series_changed;
        }

        if !self.x_current_bounds.is_none() {
            return Some(HandlePlayerStateResult {
                range: self.x_full_bounds.unwrap_or_default(),
                datasets_changed,
            });
        }

        if !self.x_full_bounds.is_none() {
            return Some(HandlePlayerStateResult {
                range: self.x_full_bounds.clone(),
                datasets_changed,
            });
        }

        None
    }

    pub fn handleMessage_range(&self, messages: Vec<MessageEvent>, options: HandlePlayerStateOptions) -> () {
        self.has_range_source = true;
        let topic = messages[0].topic();
        if topic.is_none() {
            return;
        }

        let is_x_batch = topic.unwrap() == self.x_parsed_path.as_ref().map(|p| p.topic_name).unwrap();

        if is_x_batch {
            if options.is_reset {
                self.pending_dispatch.push(UpdateDataAction::ResetFullX);
                self.x_full_bounds = None;
            }
            if let Some(x_parsed_path) = &self.x_parsed_path {
                let path_items = parse_xpath_items(&messages, x_parsed_path);

                if !path_items.is_empty() {
                    self.pending_dispatch
                        .push(UpdateDataAction::AppendFullX(path_items.clone()));
                }
            }
        } else {
            let actions = build_full_series_actions(
                &self.series,
                topic.unwrap(),
                options,
                |config| match config.parsed.modifier {
                    Some(modifier) => Some(math_functions[modifier]),
                    None => None,
                },
            );

            self.pending_dispatch.push(
                actions
                    .iter()
                    .cloned()
                    .collect::<Vec<UpdateDataAction>>(),
            );
        }
    }

    pub fn set_x_topic(&self, path: MessagePath) {
        if !path.is_eq(&self.x_parsed_path.as_ref()) {
            self.x_parsed_path = Some(path);
            self.x_full_bounds = None;
            self.x_current_bounds = None;

            self.pending_dispatch.push(UpdateDataAction::ResetCurrentX);

            self.pending_dispatch.push(UpdateDataAction::ResetFullX);
        }
    }

    pub fn set_series(&self, series: Vec<SeriesItem>) {
        self.series = series
            .iter()
            .map(|item| CustomDatasetsSeriesItem {
                config: item.clone(),
            })
            .collect();

        self.pending_dispatch.push(UpdateDataAction::UpdateSeriesConfig(self.series.clone()));
    }

    pub async fn get_viewport_datasets(&self, viewport: Viewport) -> CsvDatasetVec {
        let dispatch = Arc::clone(&self.datasets_builder_remote);
        if !dispatch.write().unwrap().pending_dispatch.is_empty() {
            dispatch.write().unwrap().pending_dispatch.clear();
            await dispatch.write().unwrap().update_data(dispatch.read().unwrap().pending_dispatch.clone());
        }

        await dispatch.write().unwrap().get_viewport_datasets(viewport)
    }

    pub async fn get_csv_data(&self) -> CsvDatasetVec {
        let dispatch = Arc::clone(&self.datasets_builder_remote);
        if !dispatch.write().unwrap().pending_dispatch.is_empty() {
            dispatch.write().unwrap().pending_dispatch.clear();
            await dispatch.write().unwrap().update_data(dispatch.read().unwrap().pending_dispatch.clone());
        }

        await dispatch.write().unwrap().get_csv_data()
    }
}

#[derive(Debug)]
pub enum HandlePlayerStateOptions {
    IsReset(bool),
}

#[derive(Debug)]
pub struct UpdateDataAction {
    type_: &'static str,
    items: Vec<ValueItem>,
}

#[derive(Debug)]
pub struct SeriesItem {
    parsed: MessagePathParsed,
}

#[derive(Debug)]
struct ValueItem {
    value: f64,
    original_value: ValueItem,
    receive_time: u64,
}

impl UpdateDataAction {
    pub fn reset_current_x(&self) {}
    pub fn append_current_x(mut self, items: Vec<ValueItem>) -> Self {
        self.items.extend(items);
        self
    }
    pub fn update_series_config(mut self, series_items: Vec<SeriesItem>) -> Self {
        self.series = series_items;
        self
    }
}

#[derive(Debug)]
struct CsvDatasetVec(Vec<CsvDataset>);

#[derive(Debug)]
pub struct Viewport {
    // Define the Viewport structure as needed
}
```