// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use lichtblick_core::settings::ColorScheme;

use crate::player::McapPlayer;

// ============================================================================
// Per-Panel Configuration
// ============================================================================

/// Image panel settings (matching Lichtblick's ImageModeConfig).
#[derive(Clone, Debug)]
pub struct ImagePanelConfig {
    pub calibration_topic: Option<String>,
    pub synchronize: bool,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub rotation: u16, // 0, 90, 180, 270
    pub brightness: f64, // 0..100
    pub contrast: f64,   // 0..100
    // Scene settings
    pub scene_render_stats: bool,
    pub scene_background: String,   // hex color e.g. "#000000"
    pub scene_label_scale: f64,     // default 1.0
    pub scene_ignore_collada_up_axis: bool,
    pub scene_mesh_up_axis: String, // "y_up" or "z_up"
}

impl Default for ImagePanelConfig {
    fn default() -> Self {
        Self {
            calibration_topic: None,
            synchronize: false,
            flip_horizontal: false,
            flip_vertical: false,
            rotation: 0,
            brightness: 50.0,
            contrast: 50.0,
            scene_render_stats: false,
            scene_background: "#000000".to_string(),
            scene_label_scale: 1.0,
            scene_ignore_collada_up_axis: false,
            scene_mesh_up_axis: "y_up".to_string(),
        }
    }
}

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
    /// Per-panel image configs, keyed by NodeId.
    pub image_configs: RwSignal<HashMap<NodeId, ImagePanelConfig>>,
    // --- Layout Manager ---
    /// Name of the currently active layout.
    pub current_layout_name: RwSignal<String>,
    /// List of all saved layout names.
    pub saved_layout_names: RwSignal<Vec<String>>,
    /// Whether current layout has unsaved changes (dirty indicator).
    pub is_dirty: RwSignal<bool>,
    /// Snapshot of the tree at last save (for dirty comparison and revert).
    pub saved_tree_json: RwSignal<String>,
}

impl LayoutState {
    pub fn new() -> Self {
        let default_tree = LayoutNode::Panel {
            id: 1,
            panel_type: PanelType::ThreeDee,
            topic: None,
        };
        let saved_json = layout_node_to_json_internal(&default_tree);

        Self {
            tree: RwSignal::new(default_tree),
            next_id: RwSignal::new(2),
            fullscreen_panel: RwSignal::new(None),
            active_settings_panel: RwSignal::new(None),
            image_configs: RwSignal::new(HashMap::new()),
            current_layout_name: RwSignal::new("Default".to_string()),
            saved_layout_names: RwSignal::new(vec!["Default".to_string()]),
            is_dirty: RwSignal::new(false),
            saved_tree_json: RwSignal::new(saved_json),
        }
    }

    /// Initialize from localStorage - restore last session.
    pub fn restore_from_storage(&self) {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        // Load saved layout names
        if let Ok(Some(names_json)) = storage.get_item("lichtblick:layout_names") {
            let names: Vec<String> = names_json
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();
            if !names.is_empty() {
                self.saved_layout_names.set(names);
            }
        }

        // Load last active layout name
        let active_name = storage
            .get_item("lichtblick:active_layout")
            .ok()
            .flatten()
            .unwrap_or_else(|| "Default".to_string());

        self.current_layout_name.set(active_name.clone());

        // Load the layout tree for this name
        let key = format!("lichtblick:layout:{}", active_name);
        if let Ok(Some(tree_json)) = storage.get_item(&key) {
            if let Some(tree) = parse_layout_node_internal(&tree_json, &mut 1) {
                let next_id = count_nodes_internal(&tree) as u32 + 1;
                self.tree.set(tree);
                self.next_id.set(next_id);
                self.saved_tree_json.set(tree_json);
                self.is_dirty.set(false);
            }
        }
    }

    /// Save current layout to localStorage.
    pub fn save_current(&self) {
        let name = self.current_layout_name.get_untracked();
        let tree = self.tree.get_untracked();
        let tree_json = layout_node_to_json_internal(&tree);

        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        let key = format!("lichtblick:layout:{}", name);
        storage.set_item(&key, &tree_json).ok();
        storage.set_item("lichtblick:active_layout", &name).ok();

        // Update saved names list
        self.saved_layout_names.update(|names| {
            if !names.contains(&name) {
                names.push(name.clone());
            }
        });
        self.persist_layout_names();

        self.saved_tree_json.set(tree_json);
        self.is_dirty.set(false);
    }

    /// Revert to last saved state.
    pub fn revert(&self) {
        let saved_json = self.saved_tree_json.get_untracked();
        if let Some(tree) = parse_layout_node_internal(&saved_json, &mut 1) {
            let next_id = count_nodes_internal(&tree) as u32 + 1;
            self.tree.set(tree);
            self.next_id.set(next_id);
            self.is_dirty.set(false);
        }
    }

    /// Rename the current layout.
    pub fn rename_current(&self, new_name: String) {
        let old_name = self.current_layout_name.get_untracked();

        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        // Move data from old key to new key
        let old_key = format!("lichtblick:layout:{}", old_name);
        let new_key = format!("lichtblick:layout:{}", new_name);
        if let Ok(Some(data)) = storage.get_item(&old_key) {
            storage.set_item(&new_key, &data).ok();
            storage.remove_item(&old_key).ok();
        }

        // Update names list
        self.saved_layout_names.update(|names| {
            if let Some(pos) = names.iter().position(|n| *n == old_name) {
                names[pos] = new_name.clone();
            }
        });
        self.persist_layout_names();

        self.current_layout_name.set(new_name.clone());
        storage.set_item("lichtblick:active_layout", &new_name).ok();
    }

    /// Delete a layout by name.
    pub fn delete_layout(&self, name: &str) {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        let key = format!("lichtblick:layout:{}", name);
        storage.remove_item(&key).ok();

        self.saved_layout_names.update(|names| {
            names.retain(|n| n != name);
            if names.is_empty() {
                names.push("Default".to_string());
            }
        });
        self.persist_layout_names();

        // If we deleted the active layout, switch to first available
        if self.current_layout_name.get_untracked() == name {
            let first = self.saved_layout_names.get_untracked()[0].clone();
            self.current_layout_name.set(first.clone());
            storage.set_item("lichtblick:active_layout", &first).ok();
            // Load that layout (or save current tree if it doesn't exist yet, e.g. "Default" fallback)
            let key = format!("lichtblick:layout:{}", first);
            if let Ok(Some(json)) = storage.get_item(&key) {
                if let Some(tree) = parse_layout_node_internal(&json, &mut 1) {
                    let next_id = count_nodes_internal(&tree) as u32 + 1;
                    self.tree.set(tree);
                    self.next_id.set(next_id);
                    self.saved_tree_json.set(json);
                }
            } else {
                // No saved data for this layout (e.g. freshly created "Default"), save current tree
                let tree = self.tree.get_untracked();
                let tree_json = layout_node_to_json_internal(&tree);
                storage.set_item(&key, &tree_json).ok();
                self.saved_tree_json.set(tree_json);
            }
            self.is_dirty.set(false);
        }
    }

    /// Duplicate a layout: copy its saved JSON under a new name and switch to it.
    pub fn duplicate_layout(&self, source_name: &str) {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        // Read the source layout JSON
        let source_key = format!("lichtblick:layout:{}", source_name);
        let tree_json = match storage.get_item(&source_key) {
            Ok(Some(json)) => json,
            _ => {
                // If no saved json, use current tree if it's the active layout
                if self.current_layout_name.get_untracked() == source_name {
                    let tree = self.tree.get_untracked();
                    layout_node_to_json_internal(&tree)
                } else {
                    return;
                }
            }
        };

        // Generate unique name
        let new_name = format!("{} copy", source_name);

        // Save under new name
        let new_key = format!("lichtblick:layout:{}", new_name);
        storage.set_item(&new_key, &tree_json).ok();

        // Add to names list
        self.saved_layout_names.update(|names| {
            if !names.contains(&new_name) {
                names.push(new_name.clone());
            }
        });
        self.persist_layout_names();

        // Switch to the new layout
        self.switch_to_layout(&new_name);
    }

    /// Switch to a different saved layout.
    pub fn switch_to_layout(&self, name: &str) {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        let key = format!("lichtblick:layout:{}", name);
        if let Ok(Some(json)) = storage.get_item(&key) {
            if let Some(tree) = parse_layout_node_internal(&json, &mut 1) {
                let next_id = count_nodes_internal(&tree) as u32 + 1;
                self.tree.set(tree);
                self.next_id.set(next_id);
                self.saved_tree_json.set(json);
                self.current_layout_name.set(name.to_string());
                self.is_dirty.set(false);
                storage.set_item("lichtblick:active_layout", name).ok();
            }
        } else {
            // Layout exists in list but has no saved JSON yet - save current tree under this name
            let tree = self.tree.get_untracked();
            let tree_json = layout_node_to_json_internal(&tree);
            storage.set_item(&key, &tree_json).ok();
            self.saved_tree_json.set(tree_json);
            self.current_layout_name.set(name.to_string());
            self.is_dirty.set(false);
            storage.set_item("lichtblick:active_layout", name).ok();
        }
    }

    /// Export current layout as JSON string.
    pub fn export_json(&self) -> String {
        let tree = self.tree.get_untracked();
        let tree_json = layout_node_to_json_internal(&tree);
        let name = self.current_layout_name.get_untracked();
        format!(
            r#"{{"name":"{}","layout":{}}}"#,
            name, tree_json
        )
    }

    /// Mark layout as dirty (called after any tree mutation).
    pub fn mark_dirty(&self) {
        let tree = self.tree.get_untracked();
        let current_json = layout_node_to_json_internal(&tree);
        let saved_json = self.saved_tree_json.get_untracked();
        self.is_dirty.set(current_json != saved_json);
        // Auto-save working state to localStorage so refresh doesn't lose it
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                storage.set_item("lichtblick:working_tree", &current_json).ok();
            }
        }
    }

    /// Persist the layout names list to localStorage.
    fn persist_layout_names(&self) {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };
        let names = self.saved_layout_names.get_untracked();
        let joined = names.join("\n");
        storage.set_item("lichtblick:layout_names", &joined).ok();
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
        self.mark_dirty();
    }

    /// Get image config for a panel (creates default if not present).
    pub fn get_image_config(&self, node_id: NodeId) -> ImagePanelConfig {
        self.image_configs.with_untracked(|configs| {
            configs.get(&node_id).cloned().unwrap_or_default()
        })
    }

    /// Update image config for a panel.
    pub fn update_image_config(&self, node_id: NodeId, f: impl FnOnce(&mut ImagePanelConfig)) {
        self.image_configs.update(|configs| {
            let config = configs.entry(node_id).or_insert_with(ImagePanelConfig::default);
            f(config);
        });
    }

    /// Reset image config to defaults.
    pub fn reset_image_config(&self, node_id: NodeId) {
        self.image_configs.update(|configs| {
            configs.insert(node_id, ImagePanelConfig::default());
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
        self.mark_dirty();
    }

    /// Split a panel into two.
    pub fn split_panel(&self, node_id: NodeId, direction: SplitDirection) {
        let new_id = self.gen_id();
        let split_id = self.gen_id();
        self.tree.update(|tree| {
            split_panel_in_tree(tree, node_id, direction, new_id, split_id);
        });
        self.mark_dirty();
    }

    /// Remove a panel from the layout (replace parent split with sibling).
    pub fn remove_panel(&self, node_id: NodeId) {
        self.tree.update(|tree| {
            remove_panel_from_tree(tree, node_id);
        });
        self.mark_dirty();
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
// Layout JSON Serialization (internal)
// ============================================================================

/// Serialize a layout node to a compact JSON representation.
pub fn layout_node_to_json_internal(node: &LayoutNode) -> String {
    match node {
        LayoutNode::Panel { id, panel_type, topic } => {
            let type_str = panel_type_to_str(panel_type);
            let topic_json = match topic {
                Some(t) => format!(r#","topic":"{}""#, t),
                None => String::new(),
            };
            format!(r#"{{"type":"panel","id":{},"panelType":"{}"{}}}"#, id, type_str, topic_json)
        }
        LayoutNode::Split { id, direction, ratio, first, second } => {
            let dir_str = match direction {
                SplitDirection::Horizontal => "row",
                SplitDirection::Vertical => "column",
            };
            format!(
                r#"{{"type":"split","id":{},"direction":"{}","ratio":{:.1},"first":{},"second":{}}}"#,
                id,
                dir_str,
                ratio,
                layout_node_to_json_internal(first),
                layout_node_to_json_internal(second)
            )
        }
    }
}

fn panel_type_to_str(pt: &PanelType) -> &'static str {
    match pt {
        PanelType::Image => "Image",
        PanelType::ThreeDee => "3D",
        PanelType::RawMessages => "RawMessages",
        PanelType::Log => "Log",
        PanelType::Plot => "Plot",
        PanelType::DataSourceInfo => "DataSourceInfo",
        PanelType::Diagnostics => "Diagnostics",
        PanelType::StateTransitions => "StateTransitions",
        PanelType::Teleop => "Teleop",
        PanelType::TopicList => "TopicList",
        PanelType::Gauge => "Gauge",
        PanelType::Indicator => "Indicator",
        PanelType::Map => "Map",
        PanelType::Parameters => "Parameters",
        PanelType::PieChart => "PieChart",
        PanelType::Publish => "Publish",
        PanelType::ServiceCall => "ServiceCall",
        PanelType::Tab => "Tab",
        PanelType::Table => "Table",
    }
}

fn str_to_panel_type(s: &str) -> PanelType {
    match s {
        "Image" => PanelType::Image,
        "3D" => PanelType::ThreeDee,
        "RawMessages" => PanelType::RawMessages,
        "Log" | "RosOut" => PanelType::Log,
        "Plot" => PanelType::Plot,
        "DataSourceInfo" => PanelType::DataSourceInfo,
        "Diagnostics" | "DiagnosticStatusPanel" => PanelType::Diagnostics,
        "StateTransitions" => PanelType::StateTransitions,
        "Teleop" => PanelType::Teleop,
        "TopicList" => PanelType::TopicList,
        "Gauge" => PanelType::Gauge,
        "Indicator" => PanelType::Indicator,
        "Map" => PanelType::Map,
        "Parameters" => PanelType::Parameters,
        "PieChart" => PanelType::PieChart,
        "Publish" => PanelType::Publish,
        "ServiceCall" => PanelType::ServiceCall,
        "Tab" => PanelType::Tab,
        "Table" => PanelType::Table,
        _ => PanelType::RawMessages,
    }
}

/// Parse a layout node from our internal JSON format.
pub fn parse_layout_node_internal(json: &str, next_id: &mut u32) -> Option<LayoutNode> {
    let json = json.trim();
    if !json.starts_with('{') {
        return None;
    }

    let type_val = extract_string_field(json, "type")?;
    match type_val {
        "panel" => {
            let id = extract_num_field(json, "id").unwrap_or_else(|| {
                let id = *next_id;
                *next_id += 1;
                id
            });
            let panel_type_str = extract_string_field(json, "panelType").unwrap_or("RawMessages");
            let panel_type = str_to_panel_type(panel_type_str);
            let topic = extract_string_field(json, "topic").map(|s| s.to_string());
            if id >= *next_id { *next_id = id + 1; }
            Some(LayoutNode::Panel { id, panel_type, topic })
        }
        "split" => {
            let id = extract_num_field(json, "id").unwrap_or_else(|| {
                let id = *next_id;
                *next_id += 1;
                id
            });
            let direction = if extract_string_field(json, "direction") == Some("row") {
                SplitDirection::Horizontal
            } else {
                SplitDirection::Vertical
            };
            let ratio = extract_float_field(json, "ratio").unwrap_or(50.0);
            let first_json = extract_object_field(json, "first")?;
            let second_json = extract_object_field(json, "second")?;
            let first = parse_layout_node_internal(first_json, next_id)?;
            let second = parse_layout_node_internal(second_json, next_id)?;
            if id >= *next_id { *next_id = id + 1; }
            Some(LayoutNode::Split {
                id,
                direction,
                ratio,
                first: Box::new(first),
                second: Box::new(second),
            })
        }
        _ => None,
    }
}

fn count_nodes_internal(node: &LayoutNode) -> usize {
    match node {
        LayoutNode::Panel { .. } => 1,
        LayoutNode::Split { first, second, .. } => 1 + count_nodes_internal(first) + count_nodes_internal(second),
    }
}

/// Extract a string field value from JSON (simple parser, no serde dependency).
fn extract_string_field<'a>(json: &'a str, field: &str) -> Option<&'a str> {
    let pattern = format!(r#""{}":""#, field);
    let start = json.find(&pattern)? + pattern.len();
    let remaining = &json[start..];
    let end = remaining.find('"')?;
    Some(&remaining[..end])
}

/// Extract a numeric u32 field from JSON.
fn extract_num_field(json: &str, field: &str) -> Option<u32> {
    let pattern = format!(r#""{}":"#, field);
    let start = json.find(&pattern)? + pattern.len();
    let remaining = &json[start..];
    let end = remaining.find(&[',', '}', ' '][..]).unwrap_or(remaining.len());
    remaining[..end].parse::<u32>().ok()
}

/// Extract a float field from JSON.
fn extract_float_field(json: &str, field: &str) -> Option<f64> {
    let pattern = format!(r#""{}":"#, field);
    let start = json.find(&pattern)? + pattern.len();
    let remaining = &json[start..];
    let end = remaining.find(&[',', '}', ' '][..]).unwrap_or(remaining.len());
    remaining[..end].parse::<f64>().ok()
}

/// Extract a nested object field from JSON (balanced braces).
fn extract_object_field<'a>(json: &'a str, field: &str) -> Option<&'a str> {
    let pattern = format!(r#""{}":"#, field);
    let start = json.find(&pattern)? + pattern.len();
    let remaining = &json[start..];
    if !remaining.starts_with('{') {
        return None;
    }
    let mut depth = 0;
    for (i, c) in remaining.chars().enumerate() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&remaining[..=i]);
                }
            }
            _ => {}
        }
    }
    None
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

    /// Left sidebar active tab: 0=Panel, 1=Topics, 2=Alerts, 3=Layouts
    pub left_sidebar_tab: RwSignal<u8>,

    /// Currently loaded file name (shown in app bar)
    pub current_file_name: RwSignal<Option<String>>,

    /// Global variables: list of (name, json_value_string) pairs
    pub global_variables: RwSignal<Vec<(String, String)>>,
}

/// Provide the global app state to the component tree.
pub fn provide_app_state() {
    let state = AppState {
        left_sidebar_open: RwSignal::new(true),
        right_sidebar_open: RwSignal::new(true),
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
        left_sidebar_tab: RwSignal::new(1),
        current_file_name: RwSignal::new(None),
        global_variables: RwSignal::new(Vec::new()),
    };

    provide_context(state);

    let layout = LayoutState::new();
    layout.restore_from_storage();
    provide_context(layout);
}

/// Access the global app state from any component.
pub fn use_app_state() -> AppState {
    expect_context::<AppState>()
}

/// Access the layout state from any component.
pub fn use_layout_state() -> LayoutState {
    expect_context::<LayoutState>()
}
