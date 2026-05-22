```rust
use std::cmp::{min, max};
use chrono::{DateTime, Utc};

type MessageDatasetArgs = (
    String, // path
    DateTime<Utc>, // startTime
    Box<dyn Fn(String) -> Option<ValidQueriedDataValue>>, // y
    Vec<Vec<MessageAndData>>, // blocks
    bool, // showPoints
);

type ValidQueriedDataValue = i64 | f64 | String | bool;

pub fn messages_to_dataset(args: MessageDatasetArgs) -> ChartDataset {
    let (path, start_time, get_y, blocks, show_points) = args;
    let mut dataset: ChartDataset = ChartDataset {
        borderWidth: 10,
        data: Vec::new(),
        label: path.clone(),
        pointBackgroundColor: "rgba(0, 0, 0, 0.4)",
        pointBorderColor: "transparent",
        pointHoverRadius: 3,
        pointRadius: if show_points { 1.25 } else { 0 },
        pointStyle: "circle",
        showLine: true,
    };

    let mut last_value: Option<ValidQueriedDataValue> = None;
    let mut last_datum: Option<(f64, f64, String)> = None;

    for messages in blocks {
        if messages.is_empty() {
            continue;
        }

        for item_by_path in messages {
            if let Some(queried_data) = extract_queried_data(item_by_path.message_event, path.timestamp_method) {
                if !is_valid_value(&queried_data) {
                    continue;
                }

                let color = get_color(&queried_data);
                let x = (item_by_path.message_event.timestamp - start_time).seconds() as f64;
                let y = get_y(queried_data);

                if last_value != Some(&queried_data) || show_points {
                    dataset.data.push((x, y, color));
                    last_value = Some(&queried_data);
                    last_datum = Some((x, y, color));
                }
            }
        }
    }

    if let Some(last_datum) = last_datum {
        dataset.data.push(last_datum);
    }

    dataset
}

fn extract_queried_data(item_by_path: &MessageAndData) -> Option<ValidQueriedDataValue> {
    item_by_path.queried_data.get(0).cloned()
}

fn is_valid_value(value: &ValidQueriedDataValue) -> bool {
    match value {
        i64 | f64 | String | bool => true,
        _ => false,
    }
}

fn get_color(value: &ValidQueriedDataValue) -> String {
    let value_for_color = if let i64 = value { value } else { value.to_f64().unwrap() };
    base_colors[positive_modulo(value_for_color as u32, base_colors.len())]
}

fn create_label(constant_name: &str, value: ValidQueriedDataValue) -> String {
    format!("{constant_name} ({value})")
}
```

Note that Rust has a different syntax and type system than TypeScript/React. The `ChartDataset` struct is assumed to be defined elsewhere in the codebase. Also, Rust's borrow checker requires more explicit handling of mutable variables, which is done using references (`&`) instead of pointers (`*`).