// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use lichtblick_core::panel::{PanelCategory, PanelInfo, PanelType};
use std::collections::HashMap;

/// A panel registration entry.
pub struct PanelRegistration {
    pub info: PanelInfo,
    pub default_config: serde_json::Value,
}

/// The panel catalog holds all available panel types.
pub struct PanelCatalog {
    panels: HashMap<String, PanelRegistration>,
}

impl PanelCatalog {
    /// Create a new catalog with all built-in panels registered.
    pub fn new() -> Self {
        let mut catalog = Self {
            panels: HashMap::new(),
        };
        catalog.register_builtin_panels();
        catalog
    }

    /// Register all built-in panels.
    fn register_builtin_panels(&mut self) {
        let builtins = vec![
            ("3D", "3D Render", PanelCategory::Visualization, "Visualize 3D data including transforms, markers, point clouds, and meshes"),
            ("Plot", "Plot", PanelCategory::Visualization, "Plot numeric values over time"),
            ("Image", "Image", PanelCategory::Visualization, "Display camera images"),
            ("RawMessages", "Raw Messages", PanelCategory::Utility, "Browse message contents in a tree view"),
            ("RawMessagesVirtual", "Raw Messages (Virtual)", PanelCategory::Utility, "Browse large messages with virtualized rendering"),
            ("RosOut", "Logs", PanelCategory::Diagnostics, "View ROS log messages"),
            ("DiagnosticStatusPanel", "Diagnostic Detail", PanelCategory::Diagnostics, "View diagnostic status details"),
            ("DiagnosticSummary", "Diagnostic Summary", PanelCategory::Diagnostics, "Overview of all diagnostic statuses"),
            ("map", "Map", PanelCategory::Visualization, "Display geographic data on a map"),
            ("Gauge", "Gauge", PanelCategory::Visualization, "Display a numeric value as a gauge"),
            ("Indicator", "Indicator", PanelCategory::Visualization, "Display a boolean state indicator"),
            ("StateTransitions", "State Transitions", PanelCategory::Visualization, "Track discrete state changes over time"),
            ("TopicGraph", "Topic Graph", PanelCategory::Utility, "Network graph of topics and nodes"),
            ("Table", "Table", PanelCategory::Visualization, "Display data in a table"),
            ("Teleop", "Teleop", PanelCategory::Teleop, "Manual control interface for robots"),
            ("Publish", "Publish", PanelCategory::Teleop, "Publish messages to topics"),
            ("CallService", "Call Service", PanelCategory::Teleop, "Make ROS service calls"),
            ("Parameters", "Parameters", PanelCategory::Utility, "View and edit ROS parameters"),
            ("Tab", "Tab", PanelCategory::Utility, "Container panel with multiple tabs"),
            ("VariableSlider", "Variable Slider", PanelCategory::Utility, "Interactive slider for global variables"),
            ("UserScriptEditor", "User Script", PanelCategory::Utility, "Write data transformation scripts"),
            ("DataSourceInfo", "Data Source Info", PanelCategory::Utility, "Display current data source metadata"),
            ("PlaybackPerformance", "Playback Performance", PanelCategory::Utility, "Monitor playback performance metrics"),
            ("PieChart", "Pie Chart", PanelCategory::Visualization, "Display data as a pie chart"),
        ];

        for (type_id, title, category, description) in builtins {
            self.panels.insert(
                type_id.to_string(),
                PanelRegistration {
                    info: PanelInfo {
                        panel_type: PanelType(type_id.to_string()),
                        title: title.to_string(),
                        description: Some(description.to_string()),
                        category,
                        has_settings: true,
                    },
                    default_config: serde_json::json!({}),
                },
            );
        }
    }

    /// Get all registered panels.
    pub fn get_all(&self) -> Vec<&PanelRegistration> {
        self.panels.values().collect()
    }

    /// Get a panel by type.
    pub fn get(&self, panel_type: &str) -> Option<&PanelRegistration> {
        self.panels.get(panel_type)
    }

    /// Register a custom panel (from extensions).
    pub fn register(&mut self, type_id: String, registration: PanelRegistration) {
        self.panels.insert(type_id, registration);
    }

    /// Get panels filtered by category.
    pub fn get_by_category(&self, category: PanelCategory) -> Vec<&PanelRegistration> {
        self.panels
            .values()
            .filter(|p| p.info.category == category)
            .collect()
    }
}

impl Default for PanelCatalog {
    fn default() -> Self {
        Self::new()
    }
}
