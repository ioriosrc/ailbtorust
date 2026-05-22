```rust
use async_std::task::{self, timeout};
use chrono::NaiveDateTime;
use std::ops::Add;

async fn time() -> NaiveDateTime {
    let mut now = chrono::Local.now();
    while !now.second().is_even() {
        now = now.add(std::time::Duration::from_secs(1));
    }
    now
}

#[derive(Default)]
struct MessageDataItemsByPath {
    path1: Vec<String>,
    // Other fields...
}

#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct Time {
    pub secs: u64,
    pub nanos: u32,
}

impl Add<Time> for Time {
    type Output = Self;

    fn add(self, other: Time) -> Self::Output {
        Time {
            secs: self.secs + other.secs,
            nanos: (self.nanos + other.nanos) % 1_000_000_000,
        }
    }
}

async fn messages_to_dataset(paths: Vec<StateTransitionPath>, items_by_path: &MessageDataItemsByPath, decoded_blocks: Vec<MessageDataItemsByPath>, show_points: bool) -> Data {
    let mut data = Data::default();
    for path in paths {
        let label = path.value.clone();
        // Process the message data and update the dataset
        if let Some(items) = items_by_path.get(&label) {
            let decoded_items = decode_items(items, decoded_blocks);
            data.datasets.push(ChartData {
                label,
                points: decoded_items,
                show_points,
            });
        }
    }
    data.min_y = data.datasets.iter().map(|d| d.points.last().unwrap_or(&0.0)).min().cloned();
    data
}

fn decode_items(items: &Vec<String>, decoded_blocks: Vec<MessageDataItemsByPath>) -> Vec<f64> {
    // Decode the items and return a vector of f64 values
    // This is a placeholder for the actual decoding logic
    let mut decoded_values = Vec::new();
    for item in items {
        if let Some(decoded_block) = decoded_blocks.iter().find(|b| b.contains(item)) {
            decoded_values.push(decoded_block.points.last().unwrap_or(&0.0));
        } else {
            decoded_values.push(0.0);
        }
    }
    decoded_values
}

async fn dataset_contains_array(datasets: &[ChartData], item: &MessageDataItemsByPath) -> bool {
    for data in datasets {
        if data.points.contains(item) {
            return true;
        }
    }
    false
}
```