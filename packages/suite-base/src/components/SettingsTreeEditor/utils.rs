```rust
use std::collections::{HashMap, VecDeque};

pub fn prepare_settings_nodes(
  roots: HashMap<String, SettingsTreeNode>,
) -> Vec<(String, SettingsTreeNode)> {
  let mut filtered_roots: Vec<(String, SettingsTreeNode)> = Vec::new();
  
  for (key, node) in roots {
    if node.is_some() {
      filtered_roots.push((key.clone(), node));
      
      // Include node in results if any children match the filter.
      let filtered_children = filter_tree_nodes(node.unwrap().children, key.clone());
      if !filtered_children.is_empty() {
        filtered_roots.push((key.clone(), SettingsTreeNode::from(filtered_children)));
      }
    }
  }

  filtered_roots.sort_by(|a, b| a.1.order.cmp(&b.1.order));
  
  filtered_roots
}

pub fn filter_tree_nodes(
  nodes: HashMap<String, SettingsTreeNode>,
  filter: &str,
) -> Vec<SettingsTreeNode> {
  let mut filtered_nodes = Vec::new();
  
  for (key, node) in nodes {
    if node.is_some() {
      // Include node in results if any children match the filter.
      let filtered_children = filter_tree_nodes(node.unwrap().children, key.clone());
      if !filtered_children.is_empty() {
        filtered_nodes.push(SettingsTreeNode::from(filtered_children));
      }
    }

    // Match on label or key in tree.
    let string_to_match = node.label().to_lowercase();
    if string_to_match.contains(filter.to_lowercase()) {
      filtered_nodes.push(node);
    }
  }

  filtered_nodes
}
```