```rust
use std::collections::{HashMap, VecDeque};
use chrono::{NaiveDateTime, Utc};

type Datum = (f64, f64, i32);
type FullDatum = (f64, f64, i32, NaiveDateTime, Option<NaiveDateTime>, f64);

pub type DataItem = Datum & {
  receive_time: NaiveDateTime,
  header_stamp: Option<NaiveDateTime>,
};

type Series = {
  config: Immutable<SeriesItem>;
  current: VecDeque<(i32, f64, f64, NaiveDateTime, Option<NaiveDateTime>, f64)>;
  full: VecDeque<(i32, f64, f64, NaiveDateTime, Option<NaiveDateTime>, f64)>;
};

type ResetSeriesFullAction = {
  type: "reset-full";
  series: SeriesConfigKey;
};

type ResetSeriesCurrentAction = {
  type: "reset-current";
  series: SeriesConfigKey;
};

type UpdateSeriesCurrentAction = {
  type: "append-current";
  series: SeriesConfigKey;
  items: Vec<DataItem>;
};

type UpdateSeriesFullAction = {
  type: "append-full";
  series: SeriesConfigKey;
  items: Vec<DataItem>;
};

type UpdateSeriesConfigAction = {
  type: "update-series-config";
  series_items: SeriesItem[];
};

pub type UpdateDataAction =
  | UpdateSeriesConfigAction
  | ResetSeriesFullAction
  | ResetSeriesCurrentAction
  | UpdateSeriesCurrentAction
  | UpdateSeriesFullAction;

const MAX_CURRENT_DATUMS_PER_SERIES: usize = 50_000;

fn compare_datum(a: &Datum, b: &Datum) -> f64 {
  a.0 - b.0
}

pub struct TimestampDatasetsBuilderImpl {
  series_by_key: HashMap<SeriesConfigKey, Series>;

  pub fn get_viewport_datasets(&self, viewport: Immutable<Viewport>) -> Vec<Dataset> {
    let mut datasets = Vec::new();
    for (series_key, series) in &self.series_by_key {
      if !series.config.enabled {
        continue;
      }

      let config = &series.config;

      let dataset: Dataset = Dataset {
        border_color: config.color,
        show_line: config.line_size != 0.0,
        fill: false,
        borderWidth: series.config.line_size as u32,
        point_radius: series.config.line_size * 1.2,
        point_hover_radius: 3,
        point_background_color: if series.config.show_line { config.contrast_color } else { config.color },
        point_border_color: "transparent",
        data: Vec::new(),
      };

      datasets.push(dataset);

      let mut all_data = vec![];
      all_data.extend_from(&series.full);
      all_data.extend_from(&series.current);

      let start_idx = 0;
      let end_idx = all_data.len();

      let x_bounds: Bounds1D = { min: f64::INFINITY, max: f64::NEG_INFINITY };
      let y_bounds: Bounds1D = { min: f64::INFINITY, max: f64::NEG_INFINITY };

      for i in 0..all_data.len() {
        let (x, y, index, receive_time, header_stamp, value) = &all_data[i];
        index = i as i32;

        if config.parsed.modifier == "derivative" {
          if i == 0 {
            start_idx = 1;
            prev_x = *x;
            prev_y = *y;
            continue;
          }

          let dx = *x - prev_x;
          let newY = dx == 0.0f64 ? f64::NAN : (*y - prev_y) / dx;
          all_data[i] = (new_x, newY, index, receive_time, header_stamp, value);
          prev_x = *x;
          prev_y = *y;
        }

        if viewport.bounds.x.min != None && x < viewport.bounds.x.min {
          start_idx = i;
          continue;
        }

        if !x.is_finite() {
          continue;
        }
        extend_bounds1D(&mut x_bounds, *x);

        if viewport.bounds.y.min != None && y < viewport.bounds.y.min {
          end_idx = i;
          break;
        }

        if !y.is_finite() {
          end_idx = i;
          break;
        }

        if viewport.bounds.x.max != None && x > viewport.bounds.x.max {
          end_idx = i;
          break;
        }
      }

      let items = all_data[start_idx..=end_idx].to_vec();

      let downsampled_viewport = Viewport::new(viewport.size.width, viewport.size.height);

      let max_points = MAX_POINTS / self.series_by_key.len() as usize;

      if items.len() < min_points {
        let downsampled_indices = items.iter().enumerate().collect::<Vec<(i32, &Datum)>>()
          .into_iter()
          .map(|(idx, item)| (idx, *item))
          .collect::<Vec<_>>();

        let downsampled_data =
          downsample_scatter(&downsampled_indices, &downsampled_viewport, max_points);
      } else {
        let downsampled_data = downsample_timeseries(&items, &downsampled_viewport, max_points);
      }

      if downsampled_data.len() < items.len() && config.show_line {
        dataset.point_radius = 0.0;
      }

      for (idx, item) in downsampled_data.iter().enumerate() {
        let (x, y, index, receive_time, header_stamp, value) = item;

        if let Some(receive_time) = receive_time {
          dataset.data.push(Datum(
            x,
            y,
            index as i32,
            *receive_time,
            *header_stamp,
            value,
          ));
        } else {
          dataset.data.push(Datum(x, y, index as i32, Utc::now(), None, value));
        }
      }

      if config.timestamp_method == "header_stamp" {
        dataset.data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
      }
    }

    datasets
  }
}
```