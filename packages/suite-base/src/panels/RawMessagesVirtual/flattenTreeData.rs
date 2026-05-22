```rust
use std::collections::{HashSet, HashMap};
use serde_json::{Value};

fn is_expandable(value: &serde_json::Value) -> bool {
    if value.is_null() || !value.is_object() {
        return false;
    }

    if let Some(ArrayOrObject(ref array_or_obj)) = value.as_array_or_object() {
        return array_or_obj.len() > 0;
    }

    if let Some(StringRef(ref string)) = value.as_string_ref() {
        return !string.is_empty();
    }

    return false;
}

pub fn flatten_tree_data(
    data: &serde_json::Value,
    expanded_nodes: HashSet<String>,
    parent_path: String,
    depth: usize,
    key_path: Vec<&str>,
) -> Vec<TreeNode> {
    let mut nodes = Vec::new();

    if data.is_null() || !data.is_object() {
        return nodes;
    }

    if let Some(ArrayOrObject(ref array_or_obj)) = data.as_array_or_object() {
        for (index, item) in array_or_obj.iter().enumerate() {
            let current_key_path: Vec<&str> = [key_path.clone(), index.to_string()].concat();
            let node_path = format!("{parent_path}{PATH_NAME_AGGREGATOR}{current_key_path.join("")}");

            if expanded_nodes.contains(&node_path) {
                let children = flatten_tree_data(item, expanded_nodes, node_path, depth + 1, current_key_path);
                nodes.extend(children);
            } else {
                nodes.push(TreeNode {
                    key: node_path,
                    label: index.to_string(),
                    value: item.clone(),
                    depth,
                    is_expandable: is_expandable(item),
                    key_path: current_key_path,
                    parent_path,
                });
            }
        }
    } else if let Some(StringRef(ref string)) = data.as_string_ref() {
        if !string.is_empty() {
            nodes.push(TreeNode {
                key: format!("{parent_path}{PATH_NAME_AGGREGATOR}{key_path.join("")}"),
                label: string.clone(),
                value: serde_json::Value::String(string.clone()),
                depth,
                is_expandable: false, // Strings are considered leaf nodes
                key_path: current_key_path,
                parent_path,
            });
        }
    } else if let Some(ObjectRef(ref object)) = data.as_object_ref() {
        for (key, value) in object.iter() {
            let current_key_path = [key_path.clone(), key.to_string()].concat();
            let node_path = format!("{parent_path}{PATH_NAME_AGGREGATOR}{current_key_path.join("")}");

            if expanded_nodes.contains(&node_path) {
                let children = flatten_tree_data(value, expanded_nodes, node_path, depth + 1, current_key_path);
                nodes.extend(children);
            } else {
                nodes.push(TreeNode {
                    key: node_path,
                    label: key.to_string(),
                    value: value.clone(),
                    depth,
                    is_expandable: is_expandable(value),
                    key_path: current_key_path,
                    parent_path,
                });
            }
        }
    }

    return nodes;
}
```

Note: This Rust code assumes the existence of a `TreeNode` struct, similar to the TypeScript/React version. Additionally, it uses `serde_json` for handling JSON data types, which needs to be installed via Cargo (`cargo add serde_json`) if not already done.