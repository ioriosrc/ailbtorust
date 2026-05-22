```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use serde_json::{Value, Map};

type CsvDatum = {
  x: f64;
  y: f64;
  receive_time: u64; // Assuming Time is represented as a Unix timestamp in milliseconds
  header_stamp: Option<u64>; // Assuming Time is represented as a Unix timestamp in milliseconds
  value: String; // Assuming OriginalValue is represented as a string
};

type Size = { width: i32; height: i32 };

/**
 * Identifier used to determine whether previous data can be reused when the config changes.
 * Compare with deep equality.
 */
pub type SeriesConfigKey = &str;

pub type SeriesItem = {
  key: SeriesConfigKey;
  /** The original index of this series in config.paths */
  config_index: usize;
  message_path: String;
  parsed: Map<String, Value>; // Assuming MessagePath is represented as a JSON object
  color: String;
  /** Used for points when lines are also shown to provide extra contrast */
  contrast_color: String;
  timestamp_method: u64; // Assuming TimestampMethod is represented as a Unix timestamp in milliseconds
  show_line: bool;
  line_size: f64;
  enabled: bool;
};

pub type Viewport = {
  /**
   * The data bounds of the viewport. The bounds hint which data will be visible to the user. When
   * undefined, assumes that all data is visible in the viewport.
   */
  x_bounds: Option<(f64, f64)>;
  y_bounds: Option<(f64, f64)>;
  /** The pixel size of the viewport */
  size: Size;
};

pub type CsvDataset = {
  label: String;
  data: Vec<CsvDatum>;
};

pub type GetViewportDatasetsResult = {
  /**
   * Indices correspond to original indices of series in `config.paths`. Array may be sparse if
   * series are invalid (parsing fails) or if they are disabled.
   */
  datasets_by_config_index: Vec<Option<Dataset>>;
  paths_with_mismatched_data_lengths: HashSet<String>;
};

pub type HandlePlayerStateResult = {
  /**
   * The x-axis range of the dataset if it is known.
   *
   * Setting the range to None indicates the builder does not know the range or does not want
   * to impose a specific range.
   */
  range: Option<(f64, f64)>;
  /** True if the datasets were changed (i.e. the builder extracted new data from the state) */
  datasets_changed: bool;
};

/**
 * IDatasetBuilder defines methods for updating the building a dataset.
 *
 * Dataset updates (via new player state, and config) are synchronous and the callers do not expect
 * to wait on any promise. While getting the viewport datasets and csv data are async to allow them
 * to happen on a worker.
 */
pub trait IDatasetsBuilder {
  fn handle_player_state(&mut self, state: Value) -> Option<HandlePlayerStateResult>;

  /**
   * The builder can provide an implementation of this method to incrementally accumulate full
   * message history delivered via `subscribeMessageRange`. Each call provides a batch of messages
   * for a single topic, together with a `startTime` used to compute relative x-axis offsets.
   *
   * When `options.is_reset` is true the builder should discard any previously accumulated data for
   * that topic before processing the new batch, signalling the start of a fresh range subscription
   * (e.g. after a seek). Subsequent calls with `is_reset: false` append to the existing data.
   */
  fn handle_message_range(&mut self, messages: Vec<(Value, u64)>, options: { is_reset: bool }, startTime: Option<u64>) -> Result<(), String>;

  fn set_series(&mut self, series: Vec<SeriesItem>);

  /**
   * Optional: return the x-axis topic name if this builder uses a separate x-axis topic
   * (e.g. custom x-axis mode). The coordinator will subscribe to this topic in addition to
   * the y-series topics so that handleMessageRange receives batches for it.
   */
  fn get_x_topic(&self) -> Option<&str>;

  async fn get_viewport_datasets(&mut self, viewport: Viewport) -> Result<GetViewportDatasetsResult, String>;

  async fn get_csv_data(&self) -> Result<Vec<CsvDataset>, String>;
}
```