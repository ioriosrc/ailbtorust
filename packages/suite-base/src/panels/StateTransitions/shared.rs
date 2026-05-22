```rust
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImmutableDataset {
    data: Vec<MessageAndData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageAndData {
    queried_data: Vec<QueryResult>,
    timestamp: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    // Define the fields of your query result here
}

fn presence<T>(value: Option<&T>) -> Option<&T> {
    value.or(None)
}

fn state_transition_path_display_name(
    path: &StateTransitionPath,
    index: usize,
) -> String {
    let label = presence(&path.label).unwrap_or("");
    let value = presence(&path.value).unwrap_or("");

    format!("Series {}", index + 1)
}

pub fn dataset_contains_array(dataset: ImmutableDataset) -> bool {
    let mut data_counts: HashSet<usize> = HashSet::new();

    for data in &dataset.data {
        if !data.queried_data.is_empty() {
            data_counts.insert(data.queried_data.len());
        }
    }

    !data_counts.is_empty() && data_counts.iter().all(|&count| count > 1)
}

// Assuming StateTransitionPath and QueryResult are defined elsewhere
```

Note: The provided code assumes that `StateTransitionPath` and `QueryResult` are already defined elsewhere in your Rust project. If they are not, you need to define them before using the `dataset_contains_array` function.