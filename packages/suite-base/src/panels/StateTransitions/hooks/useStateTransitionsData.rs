```rust
use std::collections::HashMap;

type PathState = (String, bool);

fn state_transitions_data(
    paths: Vec<PathTransition>,
    start_time: Option<Time>,
    items_by_path: HashMap<String, MessageDataItemsByPath>,
    decoded_blocks: Vec<MessageDataItemsByPath>,
    // eslint-disable-next-line @lichtblick/no-boolean-parameters
    show_points: bool,
) -> HashMap<String, (Vec<(i32, i64)>, bool)> {
    let mut out_min_y = None;
    let mut out_datasets: HashMap<String, (Vec<(i32, i64)>, bool)> = HashMap::new();
    let mut out_path_state: Vec<PathState> = Vec::new();

    paths.iter().for_each(|path| {
        let y = -(path.index() as i32 + 1) * ROW_SPACING;
        out_min_y = out_min_y.map_or(y, |min_y| min_y.min(y - ROW_MARGIN));

        let blocks_for_path = decoded_blocks
            .iter()
            .filter(|decoded_block| decoded_block.contains_key(&path.id()))
            .map(|decoded_block| decoded_block[path.id().clone()]);

        let new_block_dataset = messages_to_dataset(
            &paths,
            path,
            path.index(),
            start_time.clone(),
            y,
            show_points,
        );

        if items_by_path.contains_key(&path.id()) {
            out_datasets.insert(path.id().to_string(), (new_block_dataset.0, new_block_dataset.1));
        }

        let mut items = items_by_path.get(&path.id()).unwrap_or(&Default::default());
        let isArray = dataset_contains_array(vec![items.clone()]);

        out_path_state.push((path.id().clone(), isArray));

        if items.is_empty() {
            continue;
        }

        let new_path_dataset = messages_to_dataset(
            &paths,
            path,
            path.index(),
            start_time.clone(),
            y,
            show_points,
        );

        out_datasets.insert(path.id().to_string(), (new_path_dataset.0, new_path_dataset.1));
    });

    HashMap::from_iter(out_path_state.into_iter().zip(out_datasets.into_iter()))
}
```

Note: The Rust version of the code uses iterators and collections for better readability and performance compared to TypeScript. It also handles optional values by returning `None` for missing keys in the `items_by_path` map.