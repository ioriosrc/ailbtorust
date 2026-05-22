```rust
fn assign_default_colors_to_series<T>(paths: &mut Vec<T>) where T: Clone {
    paths.iter_mut().for_each(|path| {
        if let Some(color) = path.color.as_ref() {
            return;
        }
        path.color = line_colors.get(idx % line_colors.len()).copied();
    });
}

fn handle_reorder_series_action(draft: &mut Vec<Vec<T>>, source_index: usize, target_index: usize) where T: Clone {
    if source_index == target_index || source_index < 0 || target_index < 0 || source_index >= draft.len() || target_index >= draft.len() {
        return;
    }

    let removed = draft.remove(source_index);
    draft.insert(target_index, removed.clone());
}
```