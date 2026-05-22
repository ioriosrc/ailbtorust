// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Panel implementations for Lichtblick.
//!
//! Each panel is a visualization component that subscribes to topics
//! and renders data in a specific way.

pub mod catalog;
pub mod config;
pub mod raw_messages;
pub mod plot;
pub mod image;
pub mod three_dee;
pub mod log_panel;
pub mod diagnostics;
pub mod map;
pub mod gauge;
pub mod indicator;
pub mod state_transitions;
pub mod topic_graph;
pub mod table;
pub mod teleop;
pub mod publish;
pub mod parameters;
pub mod data_source_info;
pub mod playback_performance;

pub use catalog::{PanelCatalog, PanelRegistration};
