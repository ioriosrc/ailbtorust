```rust
use std::collections::{HashMap, HashSet};

use crate::components::{
    diagnostic_status::{DiagnosticInfo, DiagnosticStatusMessage},
    diagnostic_summary::MAX_STRING_LENGTH,
};

pub fn get_diagnostic_id(hardware_id: &str, name: Option<&str>) -> String {
    let trimmed_hardware_id = if hardware_id.starts_with('/') {
        hardware_id[1..].to_string()
    } else {
        hardware_id.to_string()
    };
    format!("|{}|{}", trimmed_hardware_id, name.unwrap_or(""));
}

// ensures the diagnostic status message's name consists of both the hardware id and the name
pub fn compute_diagnostic_info(status: DiagnosticStatusMessage, stamp: f64) -> DiagnosticInfo {
    let display_name = get_display_name(status.hardware_id.clone(), status.name.clone());
    let validated_status = status;
    if status.values.iter().any(|kv| kv.value.len() > MAX_STRING_LENGTH) {
        validated_status = DiagnosticStatusMessage {
            values: status
                .values
                .iter()
                .map(|kv| {
                    if kv.value.len() <= MAX_STRING_LENGTH {
                        kv.clone()
                    } else {
                        kv.to_string()[..MAX_STRING_LENGTH].to_string()
                    }
                })
                .collect(),
        };
    }
    DiagnosticInfo {
        status: validated_status,
        stamp,
        id: get_diagnostic_id(status.hardware_id.clone(), status.name.clone()),
        display_name,
    }
}

pub fn get_diagnostics_by_level(
    diagnostics_by_hardware_id: &HashMap<String, HashMap<&str, DiagnosticInfo>>,
) -> HashMap<i32, Vec<DiagnosticInfo>> {
    let mut ret = HashMap::new();
    for (hardware_id, diagnostic_by_name) in diagnostics_by_hardware_id.iter() {
        for (name, diagnostic) in diagnostic_by_name.iter() {
            if let Some(status_level) = diagnostic.status.level {
                ret.entry(status_level).or_insert(Vec::new()).push(diagnostic.clone());
            }
        }
    }
    ret
}

pub fn filter_and_sort_diagnostics(
    nodes: Vec<DiagnosticInfo>,
    hardware_id_filter: String,
    pinned_ids: HashSet<&str>,
) -> Vec<DiagnosticInfo> {
    let unpinned_nodes = nodes.into_iter().filter(|info| !pinned_ids.contains(&info.id.as_str()));
    if hardware_id_filter.is_empty() {
        return unpinned_nodes.collect();
    }
    // fuzzyFilter sorts by match accuracy.
    fuzzy_filter({
        options: unpinned_nodes,
        filter: hardware_id_filter,
        get_text: |info| info.display_name.clone(),
    })
}
```