```rust
use crate::workspace::{DataSource, WorkspaceActions};
use crate::PlayerSelection;
use crate::analytics::AppEvent;

#[derive(Default)]
struct ConnectionState {
    active_data_source: Option<DataSource>,
    selected_source_index: usize,
    field_errors: std::collections::HashMap<String, String>,
    field_values: std::collections::HashMap<String, String>,
}

impl ConnectionState {
    fn from_workspace_state(workspace_store: &WorkspaceStore) -> Self {
        let available_sources = workspace_store.available_sources();
        let connection_sources = available_sources
            .iter()
            .filter(|&source| source.type_ == "connection" && !source.hidden)
            .collect::<Vec<&DataSource>>();

        let enabled_sources_first =
            connection_sources.iter().cloned().chain(connection_sources.iter().rev()).collect::<Vec<&DataSource>>();

        let selected_connection_idx = if let Some(active_data_source) = workspace_store.active_data_source() {
            enabled_sources_first
                .iter()
                .enumerate()
                .find(|&(_, source)| source == active_data_source)
                .map_or(0, |(idx, _)| idx)
        } else {
            0
        };

        let selected_source = enabled_sources_first[selected_connection_idx];

        ConnectionState {
            active_data_source,
            selected_source_index,
            field_errors: std::collections::HashMap::new(),
            field_values: std::collections::HashMap::new(),
        }
    }

    fn update_workspace_state(&mut self, workspace_actions: &WorkspaceActions) {
        if let Some(active_data_source) = &self.active_data_source {
            workspace_actions.update_active_data_source(Some(*active_data_source));
        }

        for idx in 0..enabled_sources_first.len() {
            workspace_actions.update_connection_status(
                enabled_sources_first[idx].id,
                !enabled_sources_first[idx].disabled_reason.is_some(),
                None,
            );
        }
    }

    fn update_field_errors(&mut self, field_id: &str, error: String) {
        if !error.is_empty() {
            self.field_errors.insert(field_id.to_string(), error);
        } else {
            self.field_errors.remove(field_id);
        }
    }

    fn update_field_values(&mut self, field_id: &str, value: Option<String>) {
        if let Some(value) = value {
            self.field_values.insert(field_id.to_string(), value.clone());
        } else {
            self.field_values.remove(field_id);
        }
    }

    fn update_workspace_actions(&self, workspace_actions: &WorkspaceActions) {
        for idx in 0..enabled_sources_first.len() {
            if enabled_sources_first[idx] == self.active_data_source.unwrap() {
                workspace_actions.update_connection_status(
                    enabled_sources_first[idx].id,
                    !enabled_sources_first[idx].disabled_reason.is_some(),
                    Some(self.field_values.clone()),
                );
            }
        }
    }

    fn update_active_data_source(&mut self, active_data_source: Option<&DataSource>) {
        if let Some(active_data_source) = active_data_source {
            self.active_data_source = Some(*active_data_source);
            self.selected_source_index = enabled_sources_first
                .iter()
                .enumerate()
                .find(|&(_, source)| source == active_data_source)
                .map_or(0, |(idx, _)| idx);
        } else {
            self.active_data_source = None;
            self.selected_source_index = 0;
        }
    }

    fn update_connection_status(&mut self, connection_id: &str, enabled: bool, new_values: Option<HashMap<String, String>>) {
        if let Some(source) = enabled_sources_first.iter().find(|&source| source.id == connection_id) {
            if !enabled && source.disabled_reason.is_none() {
                source.disabled_reason = Some("Disabled by user".to_string());
            } else if enabled && source.disabled_reason.is_some() {
                source.disabled_reason.take();
            }

            self.field_errors.clear();
            if let Some(new_values) = new_values {
                for (field_id, value) in new_values.iter() {
                    self.update_field_values(field_id, Some(value));
                }
            }
        }
    }
}
```