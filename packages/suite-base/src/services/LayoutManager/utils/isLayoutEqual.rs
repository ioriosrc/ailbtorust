```rust
use std::collections::{HashMap, HashSet};

fn is_layout_equal(baseline: &HashMap<String, HashMap<&str, Option<serde_json::Value>>>, current: &HashMap<String, HashMap<&str, Option<serde_json::Value>>>> -> bool {
    let mut stripped_rest_current = current.clone();

    // Remove keys with undefined values from the rest of current
    for key in stripped_rest_current.keys() {
        if stripped_rest_current.get(key).unwrap().values().all(|v| v.is_none()) {
            stripped_rest_current.remove(key);
        }
    }

    let baseline_values = baseline.values().flatten();
    let current_values = stripped_rest_current.values().flatten();

    // Check if all top-level fields other than configById are equal
    for (key, value) in baseline_values {
        if !current_values.contains(&value) || value.is_none() && current_values.contains(&value) {
            return false;
        }
    }

    // For each key in the baseline's panel configs, check it still has the same value
    for (panel_id, panel_config_baseline) in baseline.values() {
        let panel_config_current = &current_values[&panel_id];

        if !panel_config_current.is_empty() && !panel_config_current.eq(panel_config_baseline) {
            return false;
        }
    }

    true
}
```