```rust
use std::cmp::{max, min};
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use crate::{
    datasource_info::Topics,
    settings_tree::{action_handler, PanelSettingsTreeUpdate},
    utils::{get_diagnostics_with_stales, get_diagnostics_by_level},
};
use derive_more::From;

pub struct DiagnosticSummaryConfig {
    min_level: String,
    topic_to_render: Option<String>,
    hardware_id_filter: String,
}

#[derive(Clone)]
pub struct DiagnosticInfo {
    id: String,
    status: DiagnosticStatusConfig,
}

#[derive(Debug, From)]
struct DiagnosticStatusConfig {
    hardware_id: String,
    name: String,
    level: u8,
}

impl DiagnosticSummary {
    pub fn new(config: DiagnosticSummaryConfig) -> Self {
        // Implement the constructor logic here
    }

    pub fn render(&self) -> Html {
        let diagnostics = self.get_diagnostics();
        let filtered_diagnostics = self.filter_by_topic_and_level(diagnostics);

        if filtered_diagnostics.is_empty() {
            return html! {
                <div class="empty-state">
                    Waiting for { self.topic_to_render.as_ref().unwrap_or("unknown") } messages
                </div>
            };
        }

        let sorted_diagnostics = self.sort_by_level(filtered_diagnostics);
        let pinned_diagnostics = self.filter_pinned(sorted_diagnostics);

        let nodes = self.render_nodes(pinned_diagnostics);

        html! {
            <Stack flex="auto">
                <PanelToolbar>
                    <Stack flex="auto" direction="row" gap={1}>
                        <Select
                            value={self.min_level.clone()}
                            id="status-filter-menu"
                            color="secondary"
                            size="small"
                            onChange={|event| self.set_min_level(event.target.value)}
                            MenuProps={{ MenuListProps: { dense: true } }}
                        >
                            {KNOWN_LEVELS.iter().map(|level| html! {
                                <MenuItem key={level} value={level}>
                                    <Typography variant="inherit" color={MESSAGE_COLORS[level]}>
                                        {LEVEL_NAMES[level]?.toUpperCase()}
                                    </Typography>
                                </MenuItem>
                            })}
                        </Select>
                        <InputBase
                            value=self.hardware_id_filter.clone()
                            placeholder="Filter"
                            onChange={|event| self.set_hardware_id_filter(event.target.value)}
                            style={{ flex: "auto", font: "inherit" }}
                        />
                    </Stack>
                </PanelToolbar>
                <Stack flex="auto">{nodes}</Stack>
            </Stack>
        }
    }

    fn get_diagnostics(&self) -> Vec<DiagnosticInfo> {
        // Implement the logic to retrieve diagnostics here
    }

    fn filter_by_topic_and_level(&self, diagnostics: Vec<DiagnosticInfo>) -> Vec<DiagnosticInfo> {
        let filtered_diagnostics = diagnostics.into_iter()
            .filter(|diagnostic| diagnostic.status.hardware_id == self.topic_to_render)
            .collect();

        filtered_diagnostics
    }

    fn sort_by_level(&self, diagnostics: Vec<DiagnosticInfo>) -> Vec<DiagnosticInfo> {
        let levels: HashMap<String, Vec<&DiagnosticInfo>> = diagnostics.iter()
            .group_by(|diagnostic| diagnostic.status.level.to_string())
            .map_values(|items| items.collect())
            .collect();

        let mut sorted_diagnostics = Vec::new();
        for level in KNOWN_LEVELS {
            if let Some(diagnostics) = levels.get(&level) {
                sorted_diagnostics.extend_from_slice(diagnostics);
            }
        }

        sorted_diagnostics
    }

    fn filter_pinned(&self, diagnostics: Vec<DiagnosticInfo>) -> Vec<DiagnosticInfo> {
        diagnostics.into_iter()
            .filter(|diagnostic| diagnostic.id.contains(&format!("|{}", self.hardware_id_filter)))
            .collect()
    }

    fn render_nodes(&self, nodes: Vec<&DiagnosticInfo>) -> Html {
        nodes.into_iter().map(|node| {
            html! {
                <div class="diagnostic-node-row" data-testid={`diagnostic-summary-node-row-${node.id}`}>
                    <DiagnosticNodeRow
                        key={node.id}
                        info={node.clone()}
                        is_pinned=node.id.contains(&format!("|{}", self.hardware_id_filter))
                        onClick={self.on_click_details}
                        onClick_pin={self.on_click_pin}
                    />
                </div>
            }
        }).collect()
    }

    fn set_min_level(&mut self, min_level: String) {
        // Implement the logic to update the minimum level here
    }

    fn set_hardware_id_filter(&mut self, hardware_id_filter: String) {
        // Implement the logic to update the hardware ID filter here
    }

    fn on_click_details(&self, diagnostic: &DiagnosticInfo) {
        // Implement the logic to open the details panel for a diagnostic here
    }

    fn on_click_pin(&self, diagnostic: &DiagnosticInfo) {
        // Implement the logic to toggle pinning a diagnostic here
    }
}
```