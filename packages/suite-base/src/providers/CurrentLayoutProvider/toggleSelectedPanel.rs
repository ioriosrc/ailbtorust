```rust
use std::collections::HashSet;

fn toggle_selected_panel(panel_id: &str, containing_tab_id: Option<&str>, config_by_id: &HashMap<String, SavedProps>, selected_panel_ids: &[String]) -> Vec<String> {
    let mut panel_ids_to_deselect: HashSet<&str> = Default::default();

    // If we selected a Tab panel, deselect its children
    if let Some(saved_config) = config_by_id.get(panel_id) {
        if is_tab_panel(panel_id) && saved_config.is_some() {
            let tab_config = &saved_config.as_ref().unwrap();
            let active_tab_idx = tab_config.active_tab_idx;
            let tabs = tab_config.tabs;
            if let Some(active_tab_layout) = tabs[active_tab_idx].layout {
                let children_panel_ids: Vec<&str> = get_all_panel_ids(&active_tab_layout, config_by_id);
                panel_ids_to_deselect.extend(children_panel_ids.iter());
            }
        }
    }

    // If we selected a child, deselect all parent Tab panels
    if let Some(containing_tab_id) = containing_tab_id {
        let mut next_parent_id: Option<&str> = containing_tab_id;
        while let Some(parent_id) = next_parent_id {
            panel_ids_to_deselect.insert(parent_id);
            next_parent_id = config_by_id.get(parent_id).unwrap().parent_tab_id.as_ref();
        }
    }

    let next_selected_panel_ids: Vec<&str> = selected_panel_ids.iter().cloned().collect();

    // Remove the deselected panels from the original selected panel IDs
    let mut next_valid_selected_panel_ids: HashSet<String> = Default::default();
    for &selected_id in &next_selected_panel_ids {
        if !panel_ids_to_deselect.contains(selected_id) {
            next_valid_selected_panel_ids.insert(selected_id.to_string());
        }
    }

    next_valid_selected_panel_ids.into_iter().collect()
}
```

This Rust function implements the same functionality as the TypeScript/React code. It takes a `panelId`, `containingTabId`, `configById`, and `selectedPanelIds` as input, then updates the `selectedPanelIds` based on the deselection rules provided in the TypeScript code.