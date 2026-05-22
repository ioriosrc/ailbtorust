```rust
use std::collections::{HashMap, HashSet};
use tracing::Logger;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub type Path = Vec<&'static str>;

const TOPIC_PATH: &[&str] = &["topics", ""];

#[derive(Debug)]
struct NodeError {
    path: Path,
    errors_by_id: Option<HashMap<String, String>>,
    children: Option<HashMap<String, Box<NodeError>>>,
}

impl NodeError {
    pub fn new(path: Path) -> Self {
        Self {
            path,
            errors_by_id: None,
            children: None,
        }
    }

    pub fn error_message(&self) -> Option<&str> {
        if let Some(errors_map) = &self.errors_by_id {
            errors_map.values().collect::<Vec<_>>().join("\n")
        } else {
            None
        }
    }

    pub fn error_at_path(&self, path: Path) -> Option<&str> {
        let mut node = self;
        for segment in path {
            if let Some(children_map) = &node.children {
                match children_map.get(segment) {
                    Some(child_node) => node = child_node,
                    None => return None,
                }
            } else {
                return None;
            }
        }
        node.error_message()
    }

    pub fn clone(&self) -> Self {
        let new_children: HashMap<String, Box<NodeError>> = self
            .children
            .as_ref()
            .map(|children| children.iter().cloned())
            .collect::<HashMap<_, _>>();
        NodeError {
            path: self.path.clone(),
            errors_by_id: self.errors_by_id.clone(),
            children: Some(new_children),
        }
    }
}

pub type LayerErrorEvents = HashMap<String, String>;

const log = Logger::new();

pub struct LayerErrors {
    errors: NodeError,
}

impl LayerErrors {
    pub fn new() -> Self {
        Self {
            errors: NodeError::new(Vec::new()),
        }
    }

    pub fn add(&mut self, path: Path, error_id: String, errorMessage: &str) {
        let mut node = &mut self.errors;
        for segment in path {
            if !node.children.contains_key(segment) {
                node.children.insert(segment.to_string(), Box::new(NodeError::new(vec![segment])));
            }
            node = node.children.get_mut(segment).unwrap();
        }

        if let Some(errors_map) = &mut node.errors_by_id {
            errors_map.insert(error_id, errorMessage.to_string());
        } else {
            node.errors_by_id = Some(HashMap::from([(error_id, errorMessage.to_string())]));
        }

        log.warn!("[{:?}] {}", path.join(" > "), errorMessage);
    }

    pub fn add_to_topic(&mut self, topic_id: &str, error_id: String, errorMessage: &str) {
        let mut new_path = TOPIC_PATH.to_vec();
        new_path.push(topic_id);

        self.add(new_path, error_id, errorMessage);
    }

    pub fn has_error(&self, path: Path, error_id: &str) -> bool {
        if let Some(node) = &self.errors {
            let current_node = node;
            for segment in path {
                current_node = match current_node.children.get(segment) {
                    Some(child_node) => child_node,
                    None => return false,
                }
            }
            if let Some(errors_map) = &current_node.errors_by_id {
                errors_map.contains_key(error_id)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn remove(&mut self, path: Path, error_id: &str) {
        if let Some(node) = &mut self.errors {
            let mut current_node = node;
            for segment in path {
                current_node = match current_node.children.get(segment) {
                    Some(child_node) => child_node,
                    None => return,
                }
            }

            if let Some(errors_map) = &mut current_node.errors_by_id {
                errors_map.remove(error_id);
            } else {
                return;
            }
        }
    }

    pub fn remove_from_topic(&mut self, topic_id: &str, error_id: &str) {
        let mut new_path = TOPIC_PATH.to_vec();
        new_path.push(topic_id);

        self.remove(new_path, error_id);
    }

    /**
     * If value is falsy then add error to path, otherwise remove error from settings path
     * @param value - value to check, if false, add error, if true, remove error
     * @param path  - path to add/remove error
     * @param errorId - id unique to error
     * @param errorMessage - error message
     */
    // eslint-disable-next-line @lichtblick/no-boolean-parameters
    pub fn error_if_false(&mut self, value: bool, path: Path, error_id: &str, errorMessage: &str) {
        if !value {
            self.add(path, error_id, errorMessage);
        } else {
            self.remove(path, error_id);
        }
    }

    pub fn clear_path(&mut self, path: Path) {
        let mut node = &mut self.errors;
        for segment in path {
            if let Some(children_map) = &mut node.children {
                match children_map.get(segment) {
                    Some(child_node) => {
                        child_node.clear_path();
                        children_map.remove(segment);
                    }
                    None => return,
                }
            } else {
                return;
            }
        }

        if let Some(errors_map) = &mut node.errors_by_id {
            errors_map.clear();
        }
    }

    pub fn clear_topic(&mut self, topic_id: &str) {
        let mut new_path = TOPIC_PATH.to_vec();
        new_path.push(topic_id);

        self.clear_path(new_path);
    }

    pub fn clear(&mut self) {
        self.clear_path(Vec::new());
    }
}
```