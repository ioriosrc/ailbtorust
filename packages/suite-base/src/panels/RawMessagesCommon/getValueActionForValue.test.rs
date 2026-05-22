```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

fn get_value_action_for_value(values: &[serde_json::Value], structure_item: &MessagePathStructureItem, path: Vec<&str>) -> Option<HashMap<String, String>> {
    let primitive_type = get_primitive_type(structure_item);
    if !primitive_type.is_none() && values.len() == 1 {
        return Some(get_slice_path_filter(values[0].as_str(), &path));
    }

    let mut filter_path = "".to_string();
    let mut multi_slice_path = "".to_string();
    let mut single_slice_path = "".to_string();

    for (i, val) in values.iter().enumerate() {
        let slice_name = format!("{:?}{:?}", i, primitive_type.unwrap());
        let slice_filter = get_slice_path_filter(val.as_str(), &path);
        filter_path.push_str(&slice_filter.filter_path);
        multi_slice_path.push_str(&slice_filter.multi_slice_path);
        single_slice_path.push_str(&slice_filter.single_slice_path);

        if !slice_filter.filter_path.is_empty() && i != 0 {
            filter_path.push(',');
            multi_slice_path.push(',');
            single_slice_path.push(',');
        }
    }

    Some(HashMap::from([
        ("filterPath".to_string(), filter_path),
        ("multiSlicePath".to_string(), multi_slice_path),
        ("primitiveType".to_string(), primitive_type.unwrap().to_string()),
        ("singleSlicePath".to_string(), single_slice_path),
    ]))
}

fn get_primitive_type(structure_item: &MessagePathStructureItem) -> Option<&str> {
    match structure_item.datatype.as_str() {
        "uint32" | "int32" | "uint64" | "int64" => Some(structure_item.primitive_type.as_str()),
        _ => None,
    }
}

fn get_slice_path_filter(value: &str, path: &[&str]) -> HashMap<String, String> {
    let mut filter_path = "".to_string();
    let mut multi_slice_path = "".to_string();
    let mut single_slice_path = "".to_string();

    for (i, slice_name) in path.iter().enumerate() {
        if i > 0 {
            filter_path.push(',');
            multi_slice_path.push(',');
            single_slice_path.push(',');
        }

        let value_str = serde_json::Value::String(value.to_string());
        let slice_filter = get_value_action_for_value(&[&value_str], structure_item, [slice_name]);
        filter_path.push_str(&slice_filter.filter_path);
        multi_slice_path.push_str(&slice_filter.multi_slice_path);
        single_slice_path.push_str(&slice_filter.single_slice_path);
    }

    HashMap::from([
        ("filterPath".to_string(), filter_path),
        ("multiSlicePath".to_string(), multi_slice_path),
        ("primitiveType".to_string(), "uint32".to_string()),
        ("singleSlicePath".to_string(), single_slice_path),
    ])
}

fn get_structure_item_for_path(structure_item: &MessagePathStructureItem, path: Vec<&str>) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for (i, slice_name) in path.iter().enumerate() {
        if i > 0 {
            result.insert(":".to_string(), ",{:?}".to_string().replace("{:?}", &slice_name));
        }

        match structure_item.next_by_name.get(slice_name) {
            Some(next_structure_item) => {
                let mut next_result = get_structure_item_for_path(next_structure_item, [slice_name]);
                for (key, value) in next_result.into_iter() {
                    result.insert(format!("{:?}{:?}", i, key), value);
                }
            },
            None => {
                let primitive_type = match structure_item.primitive_type.as_str() {
                    "uint32" | "int32" | "uint64" | "int64" => Some(structure_item.datatype.as_str()),
                    _ => None,
                };

                let mut result_path = "".to_string();
                for (i, slice_name) in path.iter().enumerate() {
                    if i > 0 {
                        result_path.push(',');
                    }

                    result_path.push_str(&slice_name);
                }

                result.insert(format!(":{}{}", i, primitive_type.unwrap()), "unknown".to_string());
            },
        }
    }

    result
}
```