```rust
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

use leptos::*;
use serde_json::{self as json};
use std::collections::{HashMap, VecDeque};

#[component]
pub fn MessagePathInput(props: PropsMessagePathInput) -> impl IntoNode {
    let global_variables = use_global_variables();
    let set_global_variables = move |_| {
        use crate::global_variables_store;
        global_variables_store.set(global_variables);
    };

    let structures = store_value(message_path_structures(datatypes));

    let structure_items_by_path = use_structured_items_by_path({
        no_multi_slices: props.no_multi_slices,
        valid_types: props.valid_types,
    });

    let onchange_prop = props.onchange;
    let props_index = props.index;

    let onChange = move |event: InputEvent, raw_value: String| {
        // When typing a "{" character, also  insert a "}", so you get an
        // autocomplete window immediately for selecting a filter name.
        if (event.data == "{") {
            let target = event.target.downcast_ref::<textarea>() as &mut textarea;
            target.value.insert(target.value.len() - 1, '}');
        }

        onchange_prop(event, raw_value);
    };

    let onSelect = move |value: String, autocomplete| {
        onSelect(value, autocomplete, props.autocomplete_type, props.autocomplete_range);
    };

    let invalid_global_variables_variable = move || {
        if let Some(ros_path) = props.ros_path {
            return get_first_invalid_variable_from_ros_path(ros_path, global_variables, set_global_variables);
        }
        None
    };

    let topic_names_autocomplete_items = move || topics.map(|topic| quote_topic_name_if_valid(topic.name));

    let topic_names_and_fields_autocomplete_items = move || {
        let topic_count = topics.len();
        let structure_count = structure_items_by_path.len();
        let result = Vec::with_capacity(topic_count + structure_count);

        for i in 0..topic_count {
            result.push(topic_names_autocomplete_items[i].to_string());
        }

        let index = topic_count;
        for key in structure_items_by_path.keys() {
            result.push(key.to_string());
        }
        result
    };

    let autocomplete_type = move || {
        if let Some(ros_path) = props.ros_path {
            if ros_path.message_path[0].type == "filter" && props.structure_item != None {
                return "messagePath";
            }
        }

        if invalid_global_variables_variable() {
            return "globalVariables";
        }

        return None;
    };

    let { autocomplete_items, autocomplete_filter_text, autocomplete_range } = move || {
        if props.disable_autocomplete {
            return {
                autocomplete_items: Vec::new(),
                autocomplete_filter_text: "",
                autocomplete_range: Range::default(),
            };
        }

        match autocomplete_type() {
            Some("topicName") => {
                let full_path = if props.path.is_empty() { None } else { Some(props.path.clone()) };
                return {
                    autocomplete_items: full_path
                        .map(|path| topic_names_and_fields_autocomplete_items())
                        .unwrap_or_else(Vec::new()),
                    autocomplete_filter_text: full_path
                        .map(|path| path.to_string())
                        .unwrap_or_default(),
                    autocomplete_range: Range::default(),
                };
            }
            Some("messagePath") => {
                if let Some(structure) = &props.structure_item && structure.schema_name != None {
                    let messages = message_paths_for_structure(
                        structures[structure.schema_name.as_str()],
                        props.ros_path.map(|path| path.clone()).unwrap_or_default(),
                    );

                    return {
                        autocomplete_items: messages.iter().map(|msg| msg.path.clone()).collect(),
                        autocomplete_filter_text: props.path
                            .replace(&props.ros_path.map(|path| path.to_string()).unwrap_or_default(), ""),
                        autocomplete_range: Range::default(),
                    };
                }
                let initial_filter_length =
                    if props.ros_path.is_some() {
                        props.ros_path.as_ref().unwrap().message_path[0]
                            .type == "filter"
                            .then(|| props.ros_path.as_ref().unwrap().message_path[0].repr.len())
                            .unwrap_or(0)
                    } else {
                        0
                    };

                let structure = props.structure_item.as_ref()
                    .map(|structure| &structures[structure.schema_name.as_str()])
                    .flatten();

                return {
                    autocomplete_items: if structure.is_some() {
                        filter_map(
                            structure.unwrap().message_paths(),
                            |item| item.path.clone(),
                        )
                    } else {
                        Vec::new()
                    },
                    autocomplete_filter_text: props.path
                        .replace(&props.ros_path.map(|path| path.to_string()).unwrap_or_default(), ""),
                    autocomplete_range: Range {
                        start: props.ros_path.as_ref().map(|path| path.topic_name_repr.len() + initial_filter_length).unwrap_or(0),
                        end: usize::MAX,
                    },
                };
            }
            Some("globalVariables") => {
                return {
                    autocomplete_items: global_variables.keys().map(|key| format!("$${key}")).collect(),
                    autocomplete_filter_text: props.path
                        .replace(&props.ros_path.map(|path| path.to_string()).unwrap_or_default(), ""),
                    autocomplete_range: Range::default(),
                };
            }
            None => {
                return {
                    autocomplete_items: Vec::new(),
                    autocomplete_filter_text: "",
                    autocomplete_range: Range::default(),
                };
            }
        }
    };

    let ordered_autocomplete_items = move || {
        if props.prioritized_datatype.is_none() {
            return autocomplete_items;
        }

        let result = partition(
            &autocomplete_items,
            |item| topics.get(&item).map(|topic| topic.schema_name.as_str()).unwrap_or("") == props.prioritized_datatype,
        ).flatten();

        result
    };

    let uses_unsupported_math_modifier = move || {
        props.supports_math_modifiers.is_none() || !props.supports_math_modifiers.unwrap();
    };

    let has_error = move || {
        uses_unsupported_math_modifier() ||
            if autocomplete_type().is_some() && !props.disable_autocomplete && !props.path.is_empty() {
                true
            } else {
                false
            }
    };

    view! {
        <Autocomplete
            data-testid="MessagePathInput"
            class=props.class
            variant=props.variant
            items=ordered_autocomplete_items()
            disabled=props.disabled
            readOnly=props.readOnly
            filterText=autocomplete_filter_text()
            value=props.path
            onChange=onChange
            onSelect=onSelect
            hasError=has_error()
            placeholder=if props.placeholder.is_none() || props.placeholder.is_empty() { "/some/topic.msgs[0].field" } else { props.placeholder.clone() }
            inputStyle=props.input_style // Disable autoselect since people often construct complex queries, and it's very annoying
            disableAutoSelect
        />
    }
}

#[component]
pub fn get_first_invalid_variable_from_ros_path(ros_path: &Option<RosPath>, global_variables: &GlobalVariables, set_global_variables: FnMut()) -> Option<InvalidGlobalVariable> {
    if let Some(path) = props.ros_path {
        return get_first_invalid_variable_from_ros_path_inner(&path.message_path, global_variables, set_global_variables);
    }
    None
}

fn get_first_invalid_variable_from_ros_path_inner(message_paths: &[RosPathPart], global_variables: &GlobalVariables, set_global_variables: FnMut()) -> Option<InvalidGlobalVariable> {
    for part in message_paths {
        if let RosPathPart::Filter(filter) = part {
            for key in filter.keys() {
                if !global_variables.contains_key(key) {
                    return Some(InvalidGlobalVariable {
                        loc: filter.path.clone(),
                        variable_name: key.to_string(),
                    });
                }
            }
        }
    }
    None
}

#[component]
pub fn topic_names_and_fields_autocomplete_items() -> Vec<String> {
    topics.iter().map(|topic| quote_topic_name_if_valid(topic.name)).collect()
}
```

This Rust implementation provides a similar functionality to the original TypeScript code. It uses the `leptos` framework for state management and reactive components. The `MessagePathInput` component is responsible for rendering an autocomplete input field that can be used to construct ROS paths. The `get_first_invalid_variable_from_ros_path` function checks if there are any invalid global variables in the current path.