```rust
use std::collections::{HashMap, VecDeque};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

import EventEmitter from "eventemitter3";
import { produce } from "immer";

type SettingsTreeAction = {
  type: string;
  payload: { path: Path };
};

type SettingsTreeNode = {
  children?: HashMap<String, SettingsTreeNode>;
  error?: LayerErrors;
  label?: string;
  defaultExpansionState?: string;
};

type SettingsTreeNodes = HashMap<String, SettingsTreeNode>;

type ActionHandler = (action: SettingsTreeAction) => void;

export type LayerErrors = {
  errors: { errorAtPath(path: Path): Option<LayerError> };
};

class SettingsManager extends EventEmitter<SettingsManagerEvents> {
  public errors = new LayerErrors();

  #nodesByKey = HashMap::new();
  #root: SettingsTreeNode = { children: HashMap::new() };

  #globalSettingsEntryValidators: Vec<dyn Fn(&SettingsTreeEntry, &mut LayerErrors)> = Vec::new();

  pub fn new(base_tree: SettingsTreeNodes) {
    super();

    self.#root = produce(self.#root, |draft| {
      for (let path in base_tree.keys()) {
        let children = draft.children.get(path);
        if (children.is_none()) {
          draft.children.insert(path.to_string(), HashMap::new());
        }
      }
    });

    self.errors.on("update", self.handleErrorUpdate);
    self.errors.on("remove", self.handleErrorUpdate);
    self.errors.on("clear", self.handleErrorUpdate);
  }

  pub fn set_nodes_for_key(&mut self, key: &str, nodes: Vec<&SettingsTreeNodeWithActionHandler>) {
    nodes.iter().for_each(|node| {
      self.#global_settings_entry_validators
        .iter()
        .for_each(|validator| validator(node));
    });

    self.#root = produce(self.#root, |draft| {
      let prev_nodes = draft.children.get(key).unwrap_or(&HashMap::new());
      for (let path in prev_nodes.keys()) {
        remove_nodeAtPath(draft, key, path);
      }
      for (let node of nodes) {
        node.error = self.errors.errors.error_at_path(path.to_string());
        node.label = node.label.unwrap_or_else(|| path[path.len() - 1]);
        node.default_expansion_state =
          node.default_expansion_state.unwrap_or("collapsed");
        add_nodeAtPath(draft, key, node);
      }
    });

    self.#nodesByKey.insert(key.to_string(), nodes);

    self.emit("update");
  }

  pub fn set_label(&mut self, path: &str, label: &str) {
    self.#root = produce(self.#root, |draft| {
      set_labelAtPath(draft, path, label);
    });

    self.emit("update");
  }

  pub fn clear_children(&mut self, path: &str) {
    self.#root = produce(self.#root, |draft| {
      clear_childrenAtPath(draft, key, path);
    });

    self.emit("update");
  }

  pub fn tree(&self) -> SettingsTreeNodes {
    return HashMap::from_iter(self.#nodesByKey.iter().map(|(k, v)| (k.to_string(), v.clone())));
  }

  pub fn handle_action(&mut self, action: SettingsTreeAction) {
    let path = action.payload.path;

    // Walk the settings tree down to the end of the path, firing any action
    // handlers along the way
    let mut cur_node = self.#root;
    for let segment in &path {
      let next_node = cur_node.children.get(segment);
      if let Some(next_node) = next_node {
        next_node.handler(&action);
        cur_node = next_node;
      } else {
        return;
      }
    }
  }

  /** Add Validator function that can run over nodes `set` on the tree and update error state accordingly */
  pub fn add_node_validator(&mut self, node_validator: &dyn Fn(&SettingsTreeEntry, &mut LayerErrors)) {
    self.#global_settings_entry_validators.push(node_validator);
  }

  pub fn remove_node_validator(&mut self, node_validator: &dyn Fn(&SettingsTreeEntry, &mut LayerErrors)) {
    self.#global_settings_entry_validators.retain(|v| v != node_validator);
  }

  pub fn handle_error_update(&self, key: &str) -> Vec<(Path, Option<LayerError>)> {
    let mut errors = Vec::new();
    for (path, nodes) in self.#nodesByKey.get(key).unwrap_or(&HashMap::new()) {
      for node in nodes.iter() {
        if node.error.is_none() {
          node.error = Some(self.errors.errors.error_at_path(path.to_string()));
        }
        errors.push((path.clone(), node.error.clone()));
      }
    }
    errors
  }
}

fn remove_nodeAtPath(root: &mut SettingsTreeNode, key: &str, path: &str) -> bool {
  if let Some(mut children) = root.children.as_mut() {
    if let Some(prev_children) = children.remove(path) {
      for child in prev_children.iter() {
        remove_nodeAtPath(root, key, child.path());
      }
      return true;
    } else {
      return false;
    }
  }

  false
}

fn clear_childrenAtPath(root: &mut SettingsTreeNode, key: &str, path: &str) -> bool {
  if let Some(mut children) = root.children.as_mut() {
    if let Some(prev_children) = children.remove(path) {
      for child in prev_children.iter() {
        clear_childrenAtPath(root, key, child.path());
      }
      return true;
    } else {
      return false;
    }
  }

  false
}

fn add_nodeAtPath(root: &mut SettingsTreeNode, key: &str, node: &SettingsTreeNode) -> bool {
  if let Some(mut children) = root.children.as_mut() {
    if !children.contains_key(node.path()) {
      children.insert(node.path().to_string(), node.clone());
      return true;
    }
  }

  false
}

fn set_labelAtPath(root: &mut SettingsTreeNode, path: &str, label: &str) -> bool {
  if let Some(mut children) = root.children.as_mut() {
    if let Some(prev_children) = children.get_mut(path) {
      prev_children.label = Some(label.to_string());
      return true;
    }
  }

  false
}
```