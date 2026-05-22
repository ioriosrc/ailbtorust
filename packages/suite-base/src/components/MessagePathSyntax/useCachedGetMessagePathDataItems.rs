```rust
use std::collections::{HashMap, HashSet};
use std::fmt;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2018-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
struct MessagePathDataItem {
    value: serde_json::Value,
    path: String,
    constant_name: Option<String>,
}

// Given a set of message paths, this returns a function that you can call to resolve a single path
// and message to an array of `MessagePathDataItem` objects. The array+objects will be the same by
// reference, as long as topics/datatypes/global variables haven't changed in the meantime.
pub fn use_cached_get_message_path_data_items(
    paths: &[&str],
) -> impl Fn(&str, &serde_json::Value) -> Vec<MessagePathDataItem> + '_ {
    let provider_topics = PanelAPI::use_data_source_info().topics;
    let global_variables = use_global_variables();
    let memoized_paths = use_shallow_memo(paths);

    let parsed_paths: HashMap<&str, MessagePath> = memoized_paths
        .iter()
        .map(|&path| {
            let ros_path = parse_message_path(path).unwrap();
            (ros_path.topic_name(), ros_path)
        })
        .collect();

    let cached_get_message_path_data_items =
        move |&path, message| cached_get_message_path_data_items_internal(&parsed_paths, global_variables, path, message);

    cached_get_message_path_data_items
}

fn cached_get_message_path_data_items_internal(
    parsed_paths: &HashMap<&str, MessagePath>,
    global_variables: &GlobalVariables,
    path: &str,
    message: &serde_json::Value,
) -> Vec<MessagePathDataItem> {
    let mut queried_data: Vec<MessagePathDataItem> = vec![];

    if path == "" || parsed_paths.get(path).is_none() {
        return queried_data;
    }

    let ros_path = parsed_paths[path].clone();

    for name in names_in_message(message) {
        let next_path_item = &ros_path.next_by_name[name];

        if next_path_item.is_some() {
            traverse(
                message[name],
                path.to_string(),
                &next_path_item.unwrap().path,
                name,
                global_variables,
            );
        }
    }

    queried_data
}

fn names_in_message(message: &serde_json::Value) -> HashSet<&str> {
    let mut names = HashSet::new();

    if let serde_json::Value::Object(fields) = message {
        for (field, value) in fields {
            match value {
                serde_json::Value::String(name) => names.insert(name.as_str()),
                serde_json::Value::Number(_) | serde_json::Value::Bool(_) | serde_json::Value::Null => {}
                _ => break,
            }
        }
    }

    names
}

// A struct to hold the parsed topics and global variables
struct GlobalVariables {
    // Implement this struct with your actual implementation
}

// A struct to hold the parsed message path
#[derive(Debug, Serialize, Deserialize)]
struct MessagePath {
    topic_name: String,
    next_by_name: HashMap<&str, NameAndPath>,
}

// A struct to hold a name and its associated path
#[derive(Debug, Serialize, Deserialize)]
struct NameAndPath {
    name: &str,
    path: String,
}
```