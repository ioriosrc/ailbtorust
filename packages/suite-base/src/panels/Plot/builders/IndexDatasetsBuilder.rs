```rust
use crate::utils::{get_chart_value, is_chart_value};
use crate::utils::datum::*;
use crate::constants::MATH_FUNCTIONS;
use crate::model::*;
use super::*;

pub struct IndexDatasetsBuilder {
    series_by_key: HashMap<SeriesConfigKey, CurrentFrameSeriesItem>,
}

impl IDatasetsBuilder for IndexDatasetsBuilder {
    fn handle_player_state(&mut self, state: Immutable<PlayerState>) -> HandlePlayerStateResult | None {
        if let Some(active_data) = state.active_data {
            let msg_events = active_data.messages;
            if msg_events.is_empty() {
                return None;
            }

            let mut range = Bounds1D { min: 0, max: 0 };
            let mut datasets_changed = false;

            for series in self.series_by_key.values_mut() {
                let math_fn = series.parsed.modifier.map(|modifier| MATH_FUNCTIONS.get(modifier));
                let msg_event = last_matching_topic(msg_events, &series.parsed.topic_name);
                if msg_event.is_none() {
                    continue;
                }

                datasets_changed = true;

                let items = simple_get_message_path_data_items(msg_event.unwrap(), &series.parsed);
                let path_items = items
                    .iter()
                    .map(|item| {
                        if !is_chart_value(&item) {
                            return None;
                        }
                        let chart_value = get_chart_value(&item);
                        let math_modified_value =
                            math_fn.as_ref().and_then(|fn_map| fn_map.get(chart_value));
                        Some({
                            x: items.iter().position(|&i| i == item).unwrap(),
                            y: math_modified_value.unwrap_or(chart_value),
                            receive_time: msg_event.unwrap().receive_time,
                            value: math_modified_value.unwrap_or(item),
                        })
                    })
                    .filter_map(Option::Some)
                    .collect();

                series.dataset.data = path_items;
                range.max = std::cmp::max(range.max, path_items.len() - 1);
            }

            self.range = Some(range);
        }

        Some({
            range: self.range.clone(),
            datasets_changed,
        })
    }

    fn set_series(&mut self, series: Immutable<SeriesItem[]>) {
        self.series_by_key = set_series(self.series_by_key, series);
    }

    async fn get_viewport_datasets(&self) -> GetViewportDatasetsResult {
        build_viewport_datasets(&self.series_by_key)
    }

    async fn get_csv_data(&self) -> CsvDataset[] {
        let mut datasets: Vec<CsvDataset> = Vec::new();
        for series in self.series_by_key.values() {
            if !series.enabled {
                continue;
            }
            datasets.push({
                label: &series.message_path,
                data: series.dataset.data.clone(),
            });
        }

        datasets
    }
}
```