```rust
use std::collections::{HashMap, VecDeque};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use chrono::NaiveTime;

struct Series {
    config: SeriesItem,
    current: VecDeque<FullDatum>,
    full: Vec<FullDatum>,
}

struct FullDatum {
    x: f64,
    y: f64,
    index: usize,
    receive_time: NaiveTime,
    value: OriginalValue,
}

struct CustomDatasetsBuilderImpl {
    x_values: HashMap<String, Series>;
    series_by_key: HashMap<SeriesConfigKey, Series>;

    max_current_datums_per_series: usize;

    // When accumulating datums into the current buffer we cap each series to this number of datums so
    // we do not grow the memory for accumulated current data indefinitely
}

impl CustomDatasetsBuilderImpl {
    fn new(max_current_datums_per_series: usize) -> Self {
        Self {
            x_values: HashMap::new(),
            series_by_key: HashMap::new(),
            max_current_datums_per_series,
        }
    }

    fn update_data(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.apply_action(action);
        }
    }

    fn get_viewport_datasets(&self, viewport: Viewport) -> GetViewportDatasetsResult {
        let mut datasets_by_config_index: HashMap<usize, Dataset> = HashMap::new();
        let mut paths_with_mismatched_data_lengths: HashSet<String> = HashSet::new();

        for series in self.series_by_key.values() {
            if !series.config.enabled {
                continue;
            }

            let dataset: Dataset = Dataset {
                borderColor: series.config.color,
                showLine: series.config.show_line,
                fill: false,
                borderWidth: series.config.line_size,
                pointRadius: series.config.line_size * 1.2,
                pointHoverRadius: 3,
                pointBackgroundColor: if series.config.show_line { series.config.contrast_color } else { series.config.color },
                pointBorderColor: "transparent",
                data: Vec::new(),
            };

            datasets_by_config_index.insert(series.config.config_index, dataset);

            // Create the full dataset by pairing full y-values with their x-value peers
            // And then pairing current y-values with their x-value peers

            let mut all_data: Vec<FullDatum> = Vec::new();

            let x_bounds = Bounds1D { min: f64::INFINITY, max: f64::NEG_INFINITY };
            let y_bounds = Bounds1D { min: f64::INFINITY, max: f64::NEG_INFINITY };

            for (idx, &x_value) in series.full.iter().zip(&self.x_values.get(series.config.key).unwrap_or_default().full.iter()) {
                if x_value == None || *x_value.unwrap() == 0.0 { // Assuming receiveTime is the key for comparison
                    continue;
                }

                all_data.push(FullDatum {
                    x: *x_value,
                    y: series.full[idx].value,
                    index: idx,
                    receive_time: *x_value,
                    value: series.full[idx].original_value,
                });

                extend_bounds1D(&mut x_bounds, *x_value);
                extend_bounds1D(&mut y_bounds, series.full[idx].value);
            }

            let full_length = all_data.len();
            for (idx, &x_value) in series.current.iter().zip(&self.x_values.get(series.config.key).unwrap_or_default().current.iter()) {
                if x_value == None || *x_value.unwrap() == 0.0 { // Assuming receiveTime is the key for comparison
                    continue;
                }

                all_data.push(FullDatum {
                    x: *x_value,
                    y: series.current[idx].value,
                    index: idx + full_length,
                    receive_time: *x_value,
                    value: series.current[idx].original_value,
                });

                extend_bounds1D(&mut x_bounds, *x_value);
                extend_bounds1D(&mut y_bounds, series.current[idx].value);
            }

            // Downsample scatter is designed for scatter plots without points since it culls values
            // outside of the viewport and these are needed when connecting the points with lines.
            if dataset.show_line {
                for datum in all_data {
                    dataset.data.push(Datum {
                        x: datum.x,
                        y: datum.y,
                        value: datum.value,
                    });
                }
            } else {
                let downsample_viewport = Viewport {
                    width: viewport.size.width as f64,
                    height: viewport.size.height as f64,
                    bounds: Bounds1D {
                        x: {
                            min: viewport.bounds.x.map(|v| v.min).unwrap_or(x_bounds.min),
                            max: viewport.bounds.x.map(|v| v.max).unwrap_or(x_bounds.max),
                        },
                        y: {
                            min: viewport.bounds.y.map(|v| v.min).unwrap_or(y_bounds.min),
                            max: viewport.bounds.y.map(|v| v.max).unwrap_or(y_bounds.max),
                        },
                    },
                };

                let mut current_data = VecDeque::new();
                for &x_value in series.full.iter() {
                    if x_value == None { // Assuming receiveTime is the key for comparison
                        continue;
                    }

                    let datum = FullDatum {
                        x: *x_value,
                        y: 0.0, // Assuming this value will be filled later
                        index: 0, // Assuming this index will be filled later
                        receive_time: *x_value,
                        value: OriginalValue::default(),
                    };
                    current_data.push_back(datum);
                }

                for &x_value in series.current.iter() {
                    if x_value == None { // Assuming receiveTime is the key for comparison
                        continue;
                    }

                    let datum = FullDatum {
                        x: *x_value,
                        y: 0.0, // Assuming this value will be filled later
                        index: 0, // Assuming this index will be filled later
                        receive_time: *x_value,
                        value: OriginalValue::default(),
                    };
                    current_data.push_back(datum);
                }

                let mut trimmed_current = VecDeque::new();
                for &datum in current_data.iter() {
                    if !trimmed_current.is_empty()
                        && datum.receive_time > trimmed_current.back().unwrap().receive_time
                    {
                        trimmed_current.pop_back();
                    }
                    trimmed_current.push_front(datum);
                }

                for datum in trimmed_current.into_iter() {
                    dataset.data.push(Datum {
                        x: datum.x,
                        y: datum.y,
                        value: datum.value,
                    });
                }
            }
        }

        GetViewportDatasetsResult {
            datasets_by_config_index,
            paths_with_mismatched_data_lengths,
        }
    }

    fn apply_action(&mut self, action: Action) {
        match action {
            Action::ResetCurrentSeries(key) => {
                if let Some(series) = self.series_by_key.get_mut(&key) {
                    series.current.clear();
                }
            },
            Action::ResetFullSeries(key) => {
                if let Some(series) = self.series_by_key.get_mut(&key) {
                    series.full.clear();
                }
            },
            Action::AppendCurrentSeries(key, items) => {
                let last_full_receive_time = match &self.x_values.get(key).unwrap().full.last() {
                    Some(x_value) => x_value.receive_time,
                    None => NaiveTime::default(),
                };

                for item in items {
                    if compare(&item.receive_time, &last_full_receive_time) <= 0 {
                        continue;
                    }

                    self.x_values.get_mut(key).unwrap().current.push_back(FullDatum {
                        x: item.receive_time,
                        y: item.value,
                        index: 0, // Assuming this index will be filled later
                        receive_time: item.receive_time,
                        value: item.original_value,
                    });
                }
            },
            Action::AppendFullSeries(key, items) => {
                for &item in items {
                    self.x_values.get_mut(key).unwrap().full.push(FullDatum {
                        x: item.receive_time,
                        y: item.value,
                        index: 0, // Assuming this index will be filled later
                        receive_time: item.receive_time,
                        value: item.original_value,
                    });
                }
            },
            Action::AppendCurrent(key, items) => {
                let last_full_receive_time = match &self.x_values.get(key).unwrap().full.last() {
                    Some(x_value) => x_value.receive_time,
                    None => NaiveTime::default(),
                };

                for item in items {
                    if compare(&item.receive_time, &last_full_receive_time) <= 0 {
                        continue;
                    }

                    self.x_values.get_mut(key).unwrap().current.push_back(FullDatum {
                        x: item.receive_time,
                        y: item.value,
                        index: 0, // Assuming this index will be filled later
                        receive_time: item.receive_time,
                        value: item.original_value,
                    });
                }
            },
            Action::AppendFull(key, items) => {
                for &item in items {
                    self.x_values.get_mut(key).unwrap().full.push(FullDatum {
                        x: item.receive_time,
                        y: item.value,
                        index: 0, // Assuming this index will be filled later
                        receive_time: item.receive_time,
                        value: item.original_value,
                    });
                }
            },
            Action::UpdateSeriesConfig(series_items) => self.update_series_config_action(&series_items),
        }
    }

    fn update_series_config_action(&mut self, series_items: &[SeriesItem]) {
        let mut new_series = HashMap::new();

        for config in series_items {
            let existing_series = self.series_by_key.get(config.key);
            if existing_series.is_none() {
                existing_series = Some(Series {
                    config: *config,
                    current: VecDeque::new(),
                    full: VecDeque::new(),
                });
            }
            new_series.insert(config.key, existing_series.unwrap());
        }

        self.series_by_key = new_series;
    }
}
```