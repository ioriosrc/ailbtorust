```rust
use crate::context::PanelCatalogContext;
use crate::i18next::{TFunction, Locale as L};
use crate::util::constants::TAB_PANEL_TYPE;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::future::Future;
use std::sync::Arc;

use i18next::ResourceBundle;
use lazy_static::lazy_static;

pub type PanelInfo = crate::context::PanelInfo;

fn get_builtin(t: TFunction<'_, L>) -> Vec<PanelInfo> {
    vec![
        PanelInfo {
            title: t("3D"),
            type_: "3D",
            description: t("3DPanelDescription"),
            thumbnail: Some(include_bytes!("three_dee_render_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/three_dee_render/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("ROSDiagnosticsDetail"),
            type_: "DiagnosticStatusPanel",
            description: t("ROSDiagnosticsDetailDescription"),
            thumbnail: Some(include_bytes!("diagnostic_status_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/diagnostic_status/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("ROSDiagnosticSummary"),
            type_: "DiagnosticSummary",
            description: t("ROSDiagnosticSummaryDescription"),
            thumbnail: Some(include_bytes!("diagnostic_summary_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/diagnostic_summary/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("image"),
            type_: "Image",
            description: t("imageDescription"),
            thumbnail: Some(include_bytes!("image_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/image/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("indicator"),
            type_: "Indicator",
            description: t("indicatorDescription"),
            thumbnail: Some(include_bytes!("indicator_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/indicator/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("gauge"),
            type_: "Gauge",
            description: t("gaugeDescription"),
            thumbnail: Some(include_bytes!("gauge_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/gauge/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("teleop"),
            type_: "Teleop",
            description: t("teleopDescription"),
            thumbnail: Some(include_bytes!("teleop_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/teleop/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("map"),
            type_: "Map",
            description: t("mapDescription"),
            thumbnail: Some(include_bytes!("map_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/map/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("parameters"),
            type_: "Parameters",
            description: t("parametersDescription"),
            thumbnail: Some(include_bytes!("parameters_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/parameters/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("plot"),
            type_: "Plot",
            description: t("plotDescription"),
            thumbnail: Some(include_bytes!("plot_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/plot/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("pieChart"),
            type_: "PieChart",
            description: t("pieChartDescription"),
            thumbnail: Some(include_bytes!("pie_chart_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/pie_chart/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("publish"),
            type_: "Publish",
            description: t("publishDescription"),
            thumbnail: Some(include_bytes!("publish_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/publish/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("callService"),
            type_: "CallService",
            description: t("callServiceDescription"),
            thumbnail: Some(include_bytes!("publish_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/call_service/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("rawMessages"),
            type_: "RawMessages",
            description: t("rawMessagesDescription"),
            thumbnail: Some(include_bytes!("raw_messages_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/raw_messages/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: "Raw Messages Virtual",
            type_: "RawMessagesVirtual",
            description: t("rawMessagesDescription"),
            thumbnail: Some(include_bytes!("raw_messages_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/raw_messages_virtual/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("log"),
            type_: "RosOut",
            description: t("logDescription"),
            thumbnail: Some(include_bytes!("log_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/log/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("stateTransitions"),
            type_: "StateTransitions",
            description: t("stateTransitionsDescription"),
            thumbnail: Some(include_bytes!("state_transitions_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/state_transitions/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("table"),
            type_: "Table",
            description: t("tableDescription"),
            thumbnail: Some(include_bytes!("table_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/table/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("topicGraph"),
            type_: "TopicGraph",
            description: t("topicGraphDescription"),
            thumbnail: Some(include_bytes!("topic_graph_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/topic_graph/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("dataSourceInfo"),
            type_: "SourceInfo",
            description: t("dataSourceInfoDescription"),
            thumbnail: Some(include_bytes!("data_source_info_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/data_source_info/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("variableSlider"),
            type_: "GlobalVariableSliderPanel",
            description: t("variableSliderDescription"),
            thumbnail: Some(include_bytes!("variable_slider_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/variable_slider/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("userScripts"),
            type_: "NodePlayground",
            description: t("userScriptsDescription"),
            thumbnail: Some(include_bytes!("user_script_editor_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/user_script_editor/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("tab"),
            type_: TAB_PANEL_TYPE,
            description: t("tabDescription"),
            thumbnail: Some(include_bytes!("tab_thumbnail.png")),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/tab/mod.rs"
                ))
            },
        },
        PanelInfo {
            title: t("studioPlaybackPerformance"),
            type_: "PlaybackPerformance",
            description: t("studioPlaybackPerformanceDescription"),
            module: async move {
                include!(concat!(
                    env!("CARGO_TARGET_DIR"),
                    "/liblichtblick.so",
                    "/resource_modules/studio_playback_performance/mod.rs"
                ))
            },
        },
    ]
}
```