```rust
use std::collections::{HashMap};

type LayoutData = HashMap<String, String>; // Example layout data structure

fn replace_panel_in_layout(
  mut layout: HashMap<String, String>,
  old_id: &str,
  new_id: &str,
) -> HashMap<String, String> {
  if let Some(value) = layout.get(old_id) {
    *layout.entry(new_id).or_insert(String::new()) = value.clone();
    return layout;
  }
  layout
}

fn replace_panel(
  panels_state: LayoutData,
  old_id: &str,
  new_id: &str,
  new_config: HashMap<String, String>,
) -> LayoutData {
  let mut new_panels_state = panels_state;
  if let Some(config) = new_config.get(old_id) {
    new_panels_state.insert(new_id.clone(), config.clone());
    delete(&mut new_panels_state, old_id);
  }
  if let Some(layout) = new_panels_state.remove(&"layout".to_string()) {
    new_panels_state.insert("layout".to_string(), replace_panel_in_layout(layout, old_id, new_id));
  }
  for id in new_panels_state.keys() {
    if let Some(config) = new_panels_state.get(id) {
      if let Some(tab_config) = config.get("tabs").and_then(|tab_config| tab_config.as_ref()) {
        let mut tabs = tab_config.clone();
        for tab in tabs.iter_mut() {
          if let Some(layout) = tab.get("layout").and_then(|layout| layout.as_ref()) {
            tab.insert("layout", replace_panel_in_layout(layout, old_id, new_id));
          }
        }
      }
    }
  }
  new_panels_state
}

fn delete(map: &mut HashMap<String, String>, key: &str) {
  if map.contains_key(key) {
    map.remove(key);
  }
}
```