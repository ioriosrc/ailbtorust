```rust
use std::collections::{HashSet, VecDeque};

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

use protobuf::descriptor;

#[derive(Debug)]
pub struct MessageDefinition {
    name: String,
    definitions: Vec<descriptor::FieldDescriptorProto>,
}

/// For one datatype in the datatypes, find the MessageDefinition that we can use
/// to either write or parse it. `datatypes` should contain the root datatype and
/// all complex sub-datatypes.
pub fn ros_datatypes_to_message_definition(
    datatypes: &descriptor::FileDescriptor,
    root_datatype_name: &str,
) -> Vec<MessageDefinition> {
    let mut result = vec![];
    let mut seen_datatype_names = HashSet::from([root_datatype_name.to_string()]);
    // It doesn't matter if we use a stack or queue here, but we use a stack.
    let mut datatype_name_stack = VecDeque::from([root_datatype_name]);

    while !datatype_name_stack.is_empty() {
        let current_datatype_name = datatype_name_stack.pop_front().unwrap();
        if let Some(current_datatype) = datatypes.get_by_name(&current_datatype_name.to_string()) {
            // The root datatype has no name field.
            let msg_definition: MessageDefinition = if current_datatype_name == root_datatype_name {
                MessageDefinition {
                    name: current_datatype_name.to_string(),
                    definitions: current_datatype.get_all_fields().to_vec(),
                }
            } else {
                MessageDefinition {
                    name: current_datatype_name.to_string(),
                    definitions: current_datatype.get_all_fields().to_vec(),
                }
            };
            result.push(msg_definition);
            for field in current_datatype.get_all_fields() {
                // Only search subfields if we haven't already seen it and it is "complex", IE it has its own fields and should
                // be contained in `datatypes`.
                let field_name = field.name.to_string();
                if field.is_complex && !seen_datatype_names.contains(&field_name) {
                    datatype_name_stack.push_back(field_name);
                    seen_datatype_names.insert(field_name);
                }
            }
        } else {
            panic!("While searching datatypes for \"{root_datatype_name}\", could not find datatype \"{current_datatype_name}\"");
        }
    }

    result
}
```