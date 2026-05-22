```rust
use std::collections::{HashMap, HashSet};

fn get_diagnostic_id(hardware_id: &str, name: &str) -> String {
    let mut parts = Vec::new();
    if hardware_id.starts_with('/') {
        parts.push(&hardware_id[1..]);
    } else {
        parts.push(hardware_id);
    }
    if name.is_empty() {
        parts.pop(); // Remove the empty string if present
    } else {
        parts.push(name);
    }
    parts.join("|")
}

fn get_diagnostics_by_level(diagnostics: &HashMap<&str, HashMap<&str, DiagnosticInfo>>) -> HashMap<i32, Vec<DiagnosticInfo>> {
    diagnostics.values().flatten()
        .group_by(|diagnostic| diagnostic.status.level)
        .map(|(level, group)| (level as i32, group.collect()))
        .collect()
}

fn filter_and_sort_diagnostics(nodes: &Vec<DiagnosticInfo>, hardware_id_filter: &str, pin_values: &HashSet<&str>) -> Vec<DiagnosticInfo> {
    nodes.into_iter()
        .filter(|diagnostic| diagnostic.hardware_id.starts_with(hardware_id_filter) || !hardware_id_filter.is_empty())
        .collect::<HashSet<_>>()
        .iter()
        .map(|hardware_id| {
            let mut sorted_nodes = nodes
                .iter()
                .filter(|diagnostic| diagnostic.hardware_id == *hardware_id)
                .sorted_by_key(|diagnostic| diagnostic.status.name.len())
                .cloned()
                .collect::<Vec<_>>();

            if !pin_values.is_empty() {
                sorted_nodes.retain(|node| pin_values.contains(&node.id));
            }

            sorted_nodes
        })
        .flatten()
        .collect()
}

fn compute_diagnostic_info(diagnostic: &DiagnosticInfo, stamp: i64) -> DiagnosticInfo {
    let mut trimmed_value = diagnostic.status.values.first().unwrap_or_default().value.clone();
    if trimmed_value.len() > MAX_STRING_LENGTH - 2 {
        trimmed_value.truncate(MAX_STRING_LENGTH - 3);
        trimmed_value.push('...');
    }
    DiagnosticInfo {
        hardware_id: diagnostic.hardware_id.to_string(),
        level: diagnostic.status.level,
        message: diagnostic.status.message.to_string(),
        name: diagnostic.status.name.to_string(),
        stamp: stamp,
        status: diagnostic.status.clone(),
        values: vec![DiagnosticValue {
            key: diagnostic.status.values.first().unwrap_or_default().key.to_string(),
            value: trimmed_value,
        }],
    }
}

fn main() {
    // Example usage
}
```