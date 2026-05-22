```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use lighthousesuite::settings::{SettingsTreeNode, SettingsTreeNodeActionItem};
use lighthousesuite_base::panels::PlotConfig;
use lighthousesuite_base::panels::shared::constants as shared_constants;
use lighthousesuite_base::panels::testing::builders::PlotBuilder;
use lighthousesuite_base::utils::plot_colors;
use lighthousesuite_test_builders::{BasicBuilder, PlotBuilder};

use serde_json::{Map, Value};
use std::collections::HashMap;

fn build_settings_tree(config: &PlotConfig, t: impl Fn(&str) -> String) -> Map<String, Value> {
    let paths = [
        PlotBuilder::path()
            .with_color(BasicBuilder::string())
            .with_label(BasicBuilder::string())
            .with_show_line(BasicBuilder::boolean())
            .with_line_size(BasicBuilder::number()),
        PlotBuilder::path(),
    ];

    let config_json = serde_json::to_value(&config).unwrap();
    let mut tree: Map<String, Value> = serde_json::from_value(config_json).unwrap();

    // paths
    if !paths.is_empty() {
        let paths_node = lighthousesuite_base::panels::shared::constants::PLOT_PATH;
        let default_plot_path = PlotBuilder::path()
            .with_color(BasicBuilder::string())
            .with_label(BasicBuilder::string())
            .with_show_line(BasicBuilder::boolean())
            .with_line_size(BasicBuilder::number());
        tree.insert(paths_node, serde_json::to_value(default_plot_path).unwrap());

        let paths_node = lighthousesuite_base::panels::shared::constants::PLOT_PATH;
        let mut children: HashMap<String, Value> = HashMap::new();
        for i in 0..paths.len() {
            let path = &paths[i];
            let value = serde_json::to_value(path).unwrap();
            children.insert(i.to_string(), value);
        }
        tree.insert(paths_node, serde_json::to_value(children).unwrap());
    }

    // paths.actions
    if !paths.is_empty() {
        let actions_node = lighthousesuite_base::panels::shared::constants::PLOT_PATH;
        let actions: Vec<SettingsTreeNodeActionItem> = vec![SettingsTreeNodeActionItem {
            id: "add-series".to_string(),
            ...lighthousesuite_base::panels::shared::constants::ACTION_ADD_SERIES,
        }];
        tree.insert(actions_node, serde_json::to_value(actions).unwrap());
    }

    // paths.children
    if !paths.is_empty() {
        let children_node = lighthousesuite_base::panels::shared::constants::PLOT_PATH;
        let mut children: HashMap<String, Value> = HashMap::new();
        for i in 0..paths.len() {
            let path = &paths[i];
            let value = serde_json::to_value(path).unwrap();
            children.insert(i.to_string(), value);
        }
        tree.insert(children_node, serde_json::to_value(children).unwrap());
    }

    tree
}
```