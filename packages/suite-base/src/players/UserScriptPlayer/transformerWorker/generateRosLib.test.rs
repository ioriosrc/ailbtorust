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
//   Copyright 2019-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use std::collections::HashMap;

type RosTypeDef = String;
type InterfaceMap = HashMap<&str, RosTypeDef>;

fn format_def(definitions: &[(&str, bool)], name: &str) -> RosTypeDef {
    let mut output = format!("export interface {} {{", name);
    for (type_name, is_complex) in definitions {
        if is_complex {
            output.push_str("  ");
        }
        output.push_str(type_name);
        if !is_complex && type_name != "string" {
            output.push('[]');
        }
        output.push('; ');
    }
    output.pop(); // Remove trailing semicolon
    output.push(' }');
    output
}

fn generate_type_defs(datatypes: &HashMap<&str, &str>) -> HashMap<&str, RosTypeDef> {
    let mut definitions = HashMap::new();
    for (schema_name, data_type) in datatypes.iter() {
        let definitions = match schema_name {
            "std_msgs/ColorRGBA" => vec![
                ("r", false),
                ("g", false),
                ("b", false),
                ("a", false),
            ],
            "std_msgs/Time" | "std_msgs/Duration" => vec![("t", false)],
            _ => Vec::new(),
        };
        definitions.insert(schema_name, format_def(&definitions, schema_name));
    }
    definitions
}

fn generate_ros_lib(topics: &[Topic], datatypes: &HashMap<&str, &str>) -> String {
    let mut ros_lib = format!("export const ROS_LIB = `");
    for topic in topics {
        let definitions = generate_type_defs(datatypes);
        let schema_name = topic.schema_name;
        let interface_map = definitions.get(schema_name).unwrap();
        let declaration = format_def(interface_map, schema_name);
        if !declaration.is_empty() {
            ros_lib.push_str(&declaration);
            ros_lib.push('\n');
        }
    }
    ros_lib.pop(); // Remove trailing newline
    ros_lib.push('`;');
    ros_lib
}

// Example usage:
fn main() {
    let topics = vec![
        Topic {
            name: "/my_topic",
            schema_name: "std_msgs/ColorRGBA",
        },
        Topic {
            name: "/empty_topic",
            schema_name: "std_msgs/NoDef",
        },
    ];

    let datatypes = stress_test_datatypes.clone();

    let ros_lib = generate_ros_lib(topics, &datatypes);

    println!("{}", ros_lib);
}
```