```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use chrono::{NaiveDateTime};
use serde_json::Value;

use crate::suite_base::players::{Topic};

#[derive(Debug)]
struct MessageDefinition {
    name: String,
    is_complex: bool,
}

#[derive(Debug)]
struct Immutable<MessageDefinition> {
    name: String,
    is_complex: bool,
}

fn quote_topic_name_if_needed(topic_name: &str) -> String {
    // Implementation for quoting topic names
    format!("\"{}\"", topic_name)
}

fn generate_message_path_suffixes_for_schema(
    schema: &MessageDefinition,
    schemas_by_name: HashMap<&str, Immutable<MessageDefinition>>,
    prefix: &str,
    seen_schema_names: &[&str],
) -> Vec<(String, String, bool)> {
    let mut result = vec![];
    for field in schema.definitions.iter() {
        if field.is_constant {
            continue;
        }

        let field_name = quote_topic_name_if_needed(&field.name);
        let type_str = &field.type;

        let path_suffix = format!("{prefix}.{field_name}");
        let is_leaf = !field.is_complex;

        result.push((path_suffix, type_str, is_leaf));

        if field.is_complex {
            let field_schema = schemas_by_name.get(field.name).unwrap();
            result.extend(generate_message_path_suffixes_for_schema(
                field_schema,
                schemas_by_name,
                &format!("{prefix}[:]", path_suffix),
                [&schema.name],
            ));
        }
    }

    result
}

pub type MessagePathSearchItem = (
    Topic,
    (String, String, bool),
    Option<&str>,
);

fn get_message_path_search_items(
    all_topics: &[Topic],
    schemas_by_name: HashMap<&str, Immutable<MessageDefinition>>,
) -> (Vec<MessagePathSearchItem>, HashMap<String, Vec<MessagePathSearchItem>>) {
    let mut items: Vec<MessagePathSearchItem> = vec![];
    let mut items_by_topic_name = HashMap::new();

    for topic in all_topics.iter() {
        if topic.schema_name.is_none() {
            continue;
        }

        let schema_name = &topic.schema_name;
        let schema = schemas_by_name.get(schema_name).unwrap();
        for field in schema.definitions.iter() {
            if field.is_constant {
                continue;
            }

            let field_name = quote_topic_name_if_needed(&field.name);
            let type_str = &field.type;

            let path_suffix = format!("{schema_name}.{field_name}");
            let is_leaf = !field.is_complex;

            items.push((
                topic.clone(),
                (path_suffix, type_str, is_leaf),
                topic.schema_name.as_deref(),
            ));

            if field.is_complex {
                let field_schema = schemas_by_name.get(field.name).unwrap();
                items.extend(generate_message_path_suffixes_for_schema(
                    field_schema,
                    schemas_by_name,
                    &format!("{schema_name}[:]", path_suffix),
                    &[schema_name],
                ));
            }
        }
    }

    (items, items_by_topic_name)
}
```