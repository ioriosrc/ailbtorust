```rust
fn state_transition_path_display_name(path: &StateTransitionPath, index: usize) -> String {
    if let Some(ref label) = path.label {
        return label.clone();
    } else if !path.value.is_empty() {
        return path.value.to_string();
    }
    "Series 1".to_string()
}

fn dataset_contains_array(dataset: &[ImmutableDataset]) -> bool {
    for dataset in dataset {
        if let Some(&message_and_data) = dataset.get(0) {
            return !message_and_data.queried_data.is_empty() && message_and_data.queried_data.len() > 1;
        }
    }
    false
}

// Rest of the code remains unchanged
```