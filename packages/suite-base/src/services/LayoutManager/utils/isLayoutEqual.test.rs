```rust
use crate::data::LayoutData;
use crate::layout_builder::{BasicBuilder, LayoutBuilder};
use crate::test_builders::BasicBuilder;

fn is_layout_equal(layout: &LayoutData, other: &LayoutData) -> bool {
    let mut result = true;

    // Compare top-level fields
    if layout.global_variables != other.global_variables {
        result = false;
    }

    // Compare configById fields (with additive tolerance)
    for (panel_id, config) in layout.config_by_id.iter() {
        let config_in_other = other.config_by_id.get(panel_id);
        if config_in_other.is_none() || !config_eq(config, config_in_other.unwrap()) {
            result = false;
            break;
        }
    }

    // Compare additional fields (with additive tolerance)
    for key in layout.keys().filter(|k| !["global_variables", "config_by_id"].contains(k)) {
        let value_in_baseline = layout.get(key);
        let value_in_current = other.get(key);
        if value_in_baseline.is_some() && value_in_current.is_none()
            || (value_in_current.is_some() && !eq(value_in_baseline.unwrap(), value_in_current.unwrap()))
        {
            result = false;
            break;
        }
    }

    result
}

fn config_eq(config1: &LayoutData, config2: &LayoutData) -> bool {
    // Implement your own logic to compare layout config values with additive tolerance
    // This is a simplified example and should be replaced with actual comparison logic
    config1.topic == config2.topic && config1.panel_id == config2.panel_id
}
```