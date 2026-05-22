// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use std::cell::RefCell;
use std::rc::Rc;
use leptos::prelude::*;
use lichtblick_core::settings::ColorScheme;

use crate::player::McapPlayer;

thread_local! {
    static PLAYER: RefCell<Option<Rc<McapPlayer>>> = RefCell::new(None);
}

/// Set the global player instance.
pub fn set_player(player: McapPlayer) {
    PLAYER.with(|p| {
        *p.borrow_mut() = Some(Rc::new(player));
    });
}

/// Get a clone of the global player Rc (if any).
pub fn get_player() -> Option<Rc<McapPlayer>> {
    PLAYER.with(|p| p.borrow().clone())
}

// ============================================================================
// Panel Types Registry
// ============================================================================

/// All available panel types.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PanelType {
    ThreeDee,
    Image,
    RawMessages,
    Log,
    Plot,
    DataSourceInfo,
    Diagnostics,
    StateTransitions,
    Teleop,
    TopicList,
    Gauge,
    Indicator,
    Map,
    Parameters,
    PieChart,
    Publish,
    ServiceCall,
    Tab,
    Table,
}

impl PanelType {
    /// Display name for the panel.
    pub fn display_name(&self) -> &'static str {
        match self {
            PanelType::ThreeDee => "3D",
            PanelType::Image => "Image",
            PanelType::RawMessages => "Raw Messages",
            PanelType::Log => "Log",
            PanelType::Plot => "Plot",
            PanelType::DataSourceInfo => "Data Source Info",
            PanelType::Diagnostics => "Diagnostics – Detail (ROS)",
            PanelType::StateTransitions => "State Transitions",
            PanelType::Teleop => "Publish",
            PanelType::TopicList => "Topics",
            PanelType::Gauge => "Gauge",
            PanelType::Indicator => "Indicator",
            PanelType::Map => "Map",
            PanelType::Parameters => "Parameters",
            PanelType::PieChart => "Pie Chart",
            PanelType::Publish => "Publish",
            PanelType::ServiceCall => "Service Call",
            PanelType::Tab => "Tab",
            PanelType::Table => "Table",
        }
    }

    /// All panel types for the "Change panel" menu.
    pub fn all() -> &'static [PanelType] {
        &[
            PanelType::ThreeDee,
            PanelType::DataSourceInfo,
            PanelType::Diagnostics,
            PanelType::Gauge,
            PanelType::Image,
            PanelType::Indicator,
            PanelType::Log,
            PanelType::Map,
            PanelType::Parameters,
            PanelType::PieChart,
            PanelType::Plot,
            PanelType::Publish,
            PanelType::RawMessages,
            PanelType::ServiceCall,
            PanelType::StateTransitions,
            PanelType::Tab,
            PanelType::Table,
            PanelType::Teleop,
            PanelType::TopicList,
        ]
    }
}

// ============================================================================
// Layout Tree
// ============================================================================

/// Unique ID for layout nodes.
pub type NodeId = u32;

/// Split direction.
#[derive(Clone, Debug, PartialEq)]
pub enum SplitDirection {
    Horizontal, // left | right
    Vertical,   // top | bottom
}

/// A node in the layout tree.
#[derive(Clone, Debug)]
pub enum LayoutNode {
    /// A leaf panel.
    Panel {
        id: NodeId,
        panel_type: PanelType,
        /// Optional topic binding for the panel.
        topic: Option<String>,
    },
    /// A split container with two children.
    Split {
        id: NodeId,
        direction: SplitDirection,
        /// Percentage for the first child (0-100).
        ratio: f64,
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
    },
}

impl LayoutNode {
    pub fn id(&self) -> NodeId {
        match self {
            LayoutNode::Panel { id, .. } => *id,
            LayoutNode::Split { id, .. } => *id,
        }
    }
}

/// Mutable layout state managed with signals.
#[derive(Clone, Copy)]
pub struct LayoutState {
    /// The root of the layout tree.
    pub tree: RwSignal<LayoutNode>,
    /// Counter for generating unique node IDs.
    pub next_id: RwSignal<NodeId>,
    /// Which panel is currently fullscreen (None = normal view).
    pub fullscreen_panel: RwSignal<Option<NodeId>>,
    /// Which panel currently has its settings open in the sidebar (None = no settings shown).
    pub active_settings_panel: RwSignal<Option<NodeId>>,
}

impl LayoutState {
    pub fn new() -> Self {
        Self {
            tree: RwSignal::new(LayoutNode::Panel {
                id: 1,
                panel_type: PanelType::ThreeDee,
                topic: None,
            }),
            next_id: RwSignal::new(2),
            fullscreen_panel: RwSignal::new(None),
            active_settings_panel: RwSignal::new(None),
        }
    }

    /// Toggle settings sidebar for a given panel.
    pub fn toggle_settings(&self, node_id: NodeId) {
        let current = self.active_settings_panel.get_untracked();
        if current == Some(node_id) {
            self.active_settings_panel.set(None);
        } else {
            self.active_settings_panel.set(Some(node_id));
        }
    }

    /// Update the topic for a given panel node.
    pub fn set_panel_topic(&self, node_id: NodeId, new_topic: Option<String>) {
        self.tree.update(|tree| {
            set_topic_in_tree(tree, node_id, new_topic);
        });
    }

    /// Generate a new unique node ID.
    pub fn gen_id(&self) -> NodeId {
        let id = self.next_id.get_untracked();
        self.next_id.set(id + 1);
        id
    }

    /// Change the panel type at a given node ID.
    pub fn change_panel(&self, node_id: NodeId, new_type: PanelType) {
        self.tree.update(|tree| {
            change_panel_in_tree(tree, node_id, new_type);
        });
    }

    /// Split a panel into two.
    pub fn split_panel(&self, node_id: NodeId, direction: SplitDirection) {
        let new_id = self.gen_id();
        let split_id = self.gen_id();
        self.tree.update(|tree| {
            split_panel_in_tree(tree, node_id, direction, new_id, split_id);
        });
    }

    /// Remove a panel from the layout (replace parent split with sibling).
    pub fn remove_panel(&self, node_id: NodeId) {
        self.tree.update(|tree| {
            remove_panel_from_tree(tree, node_id);
        });
    }

    /// Create the default initial layout based on detected topics.
    pub fn set_default_layout(&self, image_topic: Option<String>, has_point_cloud: bool, first_other_topic: Option<String>) {
        let mut next_id = 1u32;
        let mut gen = || { let id = next_id; next_id += 1; id };

        let left_panel = if let Some(topic) = image_topic {
            LayoutNode::Panel { id: gen(), panel_type: PanelType::Image, topic: Some(topic) }
        } else {
            LayoutNode::Panel { id: gen(), panel_type: PanelType::ThreeDee, topic: None }
        };

        let right_top = if has_point_cloud {
            LayoutNode::Panel { id: gen(), panel_type: PanelType::ThreeDee, topic: None }
        } else {
            LayoutNode::Panel { id: gen(), panel_type: PanelType::DataSourceInfo, topic: None }
        };

        let right_bottom = LayoutNode::Panel {
            id: gen(),
            panel_type: PanelType::RawMessages,
            topic: first_other_topic,
        };

        let right_split = LayoutNode::Split {
            id: gen(),
            direction: SplitDirection::Vertical,
            ratio: 50.0,
            first: Box::new(right_top),
            second: Box::new(right_bottom),
        };

        let root = LayoutNode::Split {
            id: gen(),
            direction: SplitDirection::Horizontal,
            ratio: 65.0,
            first: Box::new(left_panel),
            second: Box::new(right_split),
        };

        self.next_id.set(next_id);
        self.tree.set(root);
    }
}

// Tree manipulation helpers

fn change_panel_in_tree(node: &mut LayoutNode, target_id: NodeId, new_type: PanelType) {
    match node {
        LayoutNode::Panel { id, panel_type, topic } => {
            if *id == target_id {
                *panel_type = new_type;
                *topic = None; // Reset topic on panel change
            }
        }
        LayoutNode::Split { first, second, .. } => {
            change_panel_in_tree(first, target_id, new_type.clone());
            change_panel_in_tree(second, target_id, new_type);
        }
    }
}

fn split_panel_in_tree(node: &mut LayoutNode, target_id: NodeId, direction: SplitDirection, new_panel_id: NodeId, split_id: NodeId) {
    match node {
        LayoutNode::Panel { id, panel_type, topic } => {
            if *id == target_id {
                let original = LayoutNode::Panel {
                    id: *id,
                    panel_type: panel_type.clone(),
                    topic: topic.clone(),
                };
                // New panel inherits the same type AND topic from original
                let new_panel = LayoutNode::Panel {
                    id: new_panel_id,
                    panel_type: panel_type.clone(),
                    topic: topic.clone(),
                };
                *node = LayoutNode::Split {
                    id: split_id,
                    direction,
                    ratio: 50.0,
                    first: Box::new(original),
                    second: Box::new(new_panel),
                };
            }
        }
        LayoutNode::Split { first, second, .. } => {
            split_panel_in_tree(first, target_id, direction.clone(), new_panel_id, split_id);
            split_panel_in_tree(second, target_id, direction, new_panel_id, split_id);
        }
    }
}

fn set_topic_in_tree(node: &mut LayoutNode, target_id: NodeId, new_topic: Option<String>) {
    match node {
        LayoutNode::Panel { id, topic, .. } => {
            if *id == target_id {
                *topic = new_topic;
            }
        }
        LayoutNode::Split { first, second, .. } => {
            set_topic_in_tree(first, target_id, new_topic.clone());
            set_topic_in_tree(second, target_id, new_topic);
        }
    }
}

fn remove_panel_from_tree(node: &mut LayoutNode, target_id: NodeId) {
    // Only works on splits - find the split containing the target panel
    if let LayoutNode::Split { first, second, .. } = node {
        if first.id() == target_id {
            // Replace this split with the second child
            *node = *second.clone();
            return;
        }
        if second.id() == target_id {
            // Replace this split with the first child
            *node = *first.clone();
            return;
        }
        // Recurse
        remove_panel_from_tree(first, target_id);
        remove_panel_from_tree(second, target_id);
    }
}

// ============================================================================
// App State
// ============================================================================

/// Global application state, provided at the root level.
#[derive(Clone, Copy)]
pub struct AppState {
    // UI state
    pub left_sidebar_open: RwSignal<bool>,
    pub right_sidebar_open: RwSignal<bool>,
    pub data_source_dialog_open: RwSignal<bool>,

    // Playback state
    pub is_playing: RwSignal<bool>,
    pub playback_progress: RwSignal<f64>,
    pub playback_speed: RwSignal<f64>,
    pub current_time_display: RwSignal<String>,
    pub duration_display: RwSignal<String>,
    pub has_active_layout: RwSignal<bool>,
    pub topic_count: RwSignal<usize>,
    pub message_count: RwSignal<usize>,
    /// Frame tick counter - incremented every animation frame to trigger panel re-renders.
    pub frame_tick: RwSignal<u64>,

    // Settings
    pub color_scheme: RwSignal<ColorScheme>,
}

/// Provide the global app state to the component tree.
pub fn provide_app_state() {
    let state = AppState {
        left_sidebar_open: RwSignal::new(false),
        right_sidebar_open: RwSignal::new(false),
        data_source_dialog_open: RwSignal::new(false),
        is_playing: RwSignal::new(false),
        playback_progress: RwSignal::new(0.0),
        playback_speed: RwSignal::new(1.0),
        current_time_display: RwSignal::new("0:00.000".to_string()),
        duration_display: RwSignal::new("0:00.000".to_string()),
        has_active_layout: RwSignal::new(false),
        topic_count: RwSignal::new(0),
        message_count: RwSignal::new(0),
        frame_tick: RwSignal::new(0),
        color_scheme: RwSignal::new(ColorScheme::Dark),
    };

    provide_context(state);
    provide_context(LayoutState::new());
}

/// Access the global app state from any component.
pub fn use_app_state() -> AppState {
    expect_context::<AppState>()
}

/// Access the layout state from any component.
pub fn use_layout_state() -> LayoutState {
    expect_context::<LayoutState>()
}
