```rust
use serde_json::{self, Value};
use std::collections::HashSet;

#[derive(Debug)]
struct NodeExpansion {
    key: String,
    state: Option<NodeState>,
}

#[derive(Debug)]
enum NodeState {
    Expanded,
    Collapsed,
}

#[derive(Debug)]
enum PrimitiveType {
    Int64,
    Bool,
    Int32,
    String,
}

// Implement your logic for these functions here
fn isTypedArray(obj: &Value) -> bool {
    obj.is_array() && !obj.as_array().unwrap().iter().all(|item| item.is_null())
}

fn invert(value: NodeState) -> NodeState {
    match value {
        NodeState::Expanded => NodeState::Collapsed,
        NodeState::Collapsed => NodeState::Expanded,
    }
}

// Implement your logic for these functions here
fn toggle_expansion(
    state: &mut HashMap<String, Option<NodeState>>,
    paths: HashSet<&str>,
    key: &str,
) -> Result<(), String> {
    if state.contains_key(key) && state[key].is_some() {
        let prev_state = state[key].unwrap();
        let new_state = prev_state == NodeState::Expanded
            .then(|| NodeState::Collapsed)
            .or_else(|| NodeState::Expanded);
        state.insert(key.to_string(), Some(new_state));
    }
    Ok(())
}

// Implement your logic for these functions here
fn generate_deep_key_paths(obj: &Value) -> HashSet<String> {
    let mut keys = HashSet::new();
    fn recurse_map_keys(path: Vec<&str>, nested_obj: &Value) {
        if nested_obj.is_null() || !nested_obj.is_object() && !nested_obj.is_array() {
            return;
        }

        if isTypedArray(nested_obj) {
            return;
        }

        let key = match path.last() {
            Some(k) => k.to_string(),
            None => "".to_string(),
        };

        keys.insert(key);

        for (k, v) in nested_obj.as_object().unwrap() {
            recurse_map_keys(vec![&key, &k], v);
        }
    }

    recurse_map_keys(vec![], obj);
    keys
}

// Implement your logic for these functions here
fn get_change_counts(
    data: Value,
    starting_counts: &mut HashMap<&str, i32>,
) {
    for (_, value) in data.as_object().unwrap() {
        match value {
            serde_json::Value::Object(ref map) => {
                get_change_counts(map.clone(), starting_counts);
            }
            _ => {
                *starting_counts.entry(value.to_string()).or_insert(0) += 1;
            }
        }
    }
}

const foxglove_docs_links_by_datatype: HashMap<&str, &str> = [
    ("foxglove_msgs/TurtlePose", "https://docs.ros.org/api/turtlesim/html/msg/TurtlePose.html"),
    // Add more mappings as needed
].iter()
    .cloned()
    .collect();

fn get_message_documentation_link(datatype: &str) -> Option<&str> {
    foxglove_docs_links_by_datatype.get(datatype)
}

fn get_constant_nameByKeyPath(key_path: Vec<String>, queried_data: &[MessagePathDataItem]) -> Option<&str> {
    if key_path.len() > 0 && key_path[0].parse::<usize>().is_ok() {
        return queried_data[key_path[0].parse::<usize>()?]?.constant_name;
    }

    None
}

fn is_single_elem_array(value: &Value) -> bool {
    value.is_array() && !value.as_array().unwrap().iter().all(|item| item.is_null())
}

fn data_without_wrapping_array(value: &Value) -> &Value {
    if is_single_elem_array(value) && value.as_array().unwrap()[0].is_object() {
        return &value.as_array().unwrap()[0];
    }

    value
}

fn get_single_value(value: &Value, queried_data: &[MessagePathDataItem]) -> &Value {
    if !is_single_elem_array(value) {
        return value;
    }

    if let Some(item) = queried_data.get(key_path[0].parse::<usize>()?) {
        return item.value();
    }

    value
}

fn get_value_labels({
    constant_name,
    label,
    item_value,
    key_path,
}: &ValueLabelsProps) -> ValueLabels {
    let mut item_label = label;
    let arr_label = "";

    if let Some(bigint_value) = item_value.as_bigint() {
        item_label = bigint_value.to_string();
    }

    // Handle typed arrays (binary data preview)
    if isTypedArray(item_value) && !(item_value.as_array().unwrap()[0].is_object()) {
        let array = item_value.as_array().unwrap();
        let preview_items: Vec<&Value> = array
            .iter()
            .take(DATA_ARRAY_PREVIEW_LIMIT)
            .collect();
        let has_more = array.len() > DATA_ARRAY_PREVIEW_LIMIT;

        arr_label = format!(
            "({}) [{}{}]",
            array.len(),
            preview_items.iter().map(|i| serde_json::to_string(i).unwrap()).join(", "),
            if has_more { ", …" } else { "" }
        );

        item_label = item_value.as_str().unwrap();
    }

    // Append constant name if available
    if let Some(constant_name) = constant_name {
        item_label = format!("{} ({})", item_label, constant_name);
    }

    // Pad nanosecond fields to 9 digits for better readability
    if key_path[0] == "nsec" && item_value.is_i64() && item_label.is_string() {
        item_label = format!("{:09}", item_value.as_i64().unwrap());
    }

    ValueLabels { arr_label, item_label }
}

fn get_copy_action(
    { copied }: &CopyAction,
    item_value: &Value,
    handle_copy: impl Fn(&str) -> (),
) -> CopyAction {
    CopyAction {
        key: "Copy",
        active_color: if copied { "success" } else { "primary" },
        tooltip: if copied { "Copied" } else { "Copy to Clipboard" },
        icon: if copied { <CheckIcon as ReactElement>::default() } else { <CopyAllIcon as ReactElement>::default() },
        onClick: move || handle_copy(json!(item_value)),
    }
}

fn get_filter_action(on_filter: impl Fn()) -> CopyAction {
    CopyAction {
        key: "Filter",
        tooltip: "Filter on this value",
        icon: <FilterIcon as ReactElement>::default(),
        onClick: on_filter,
    }
}

fn get_line_chart_action(
    single_slice_path: &str,
    open_plot_panel: impl Fn(String) -> (),
) -> CopyAction {
    CopyAction {
        key: "line",
        tooltip: "Plot this value on a line chart",
        icon: <LineChartIcon as ReactElement>::default(),
        onClick: move || open_plot_panel(single_slice_path.to_string()),
    }
}

fn get_scatter_plot_action(
    multi_slice_path: &str,
    open_plot_panel: impl Fn(String) -> (),
) -> CopyAction {
    CopyAction {
        key: "scatter",
        tooltip: "Plot this value on a scatter plot",
        icon: <ScatterPlotIcon as ReactElement>::default(),
        onClick: move || open_plot_panel(multi_slice_path.to_string()),
    }
}

fn get_state_transitions_action(
    single_slice_path: &str,
    open_state_transitions_panel: impl Fn(String) -> (),
) -> CopyAction {
    CopyAction {
        key: "stateTransitions",
        tooltip: "View state transitions for this value",
        icon: <StateTransitionsIcon as ReactElement>::default(),
        onClick: move || open_state_transitions_panel(single_slice_path.to_string()),
    }
}

fn deduce_primitive_type(value: &Value) -> Option<PrimitiveType> {
    match value {
        serde_json::Value::Number(ref num) => Some(match num.as_i64() {
            Some(i64_value) => {
                if i64_value > i32::MAX as i64 || i64_value < i32::MIN as i64 {
                    PrimitiveType::Int64
                } else {
                    PrimitiveType::Int32
                }
            }
            None => {
                if num.is_i128() && num.as_i128().unwrap() > i32::MAX as i128 || num.is_i128() < i32::MIN as i128 {
                    PrimitiveType::Int64
                } else {
                    PrimitiveType::Int32
                }
            }
        },
        serde_json::Value::Bool(_) => Some(PrimitiveType::Bool),
        serde_json::Value::String(_) => Some(PrimitiveType::String),
        _ => None,
    }
}

fn is_object_element(
    value: &Value,
    path_item: &str,
    structure_item: Option<&MessagePathStructureItem>,
) -> bool {
    if let Ok(index) = path_item.parse::<usize>() {
        return structure_item.is_some()
            && (structure_item.as_ref().unwrap().structure_type == "message"
                || structure_item.as_ref().unwrap().structure_type == "array");
    }

    false
}

fn isArray_element(
    value: &Value,
    path_item: &str,
    structure_item: Option<&MessagePathStructureItem>,
) -> bool {
    if let Ok(index) = path_item.parse::<usize>() {
        return structure_item.is_some()
            && (structure_item.as_ref().unwrap().structure_type == "array");
    }

    false
}

fn format_value_for_filter(value: &Value) -> String {
    match value {
        serde_json::Value::Number(ref num) => num.to_string(),
        _ => serde_json::to_string(value).unwrap(),
    }
}
```

Note that this code assumes the existence of `serde_json` and `ReactElement` types for the parts that use these libraries. You'll need to adjust the code based on your specific requirements and dependencies.