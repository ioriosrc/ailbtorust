```rust
use std::collections::HashMap;

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

/// Generates virtual ts files for each type exported by the @foxglove/schemas package.
pub fn generate_foxglove_schema_declarations() -> Vec<HashMap<String, String>> {
    let schemas = HashMap::from_iter(
        export_type_script_schemas()
            .into_iter()
            .map(|(name, source_code)| (name.clone(), source_code))
            .collect::<Vec<(String, String)>>(),
    );

    let files = schemas
        .iter()
        .map(|(name, source_code)| {
            HashMap::from([
                ("fileName".to_string(), format!("@foxglove/schemas/{name}.ts")),
                ("filePath".to_string(), format!("@foxglove/schemas/{name}.ts")),
                (
                    "sourceCode".to_string(),
                    source_code.replace_all(b"/export enum (\w+) {", b"/const enum $1 {"),
                ),
            ])
        })
        .collect::<Vec<HashMap<String, String>>>();

    files
}

/// Returns a configuration for the user script project.
pub fn get_user_script_project_config() -> UserScriptProjectConfig {
    let declarations: Vec<UserScriptProjectConfigItem> = vec![
        generate_foxglove_schema_declarations().into_iter().flatten().collect(),
        raw_user_utils.iter().map(|utility| utility.to_config()).collect(),
    ]
    .concat();

    UserScriptProjectConfig {
        default_lib_filename: lib_filename.clone(),
        ros_lib: Some(UserScriptProjectConfigItem {
            fileName: ros_lib_filename.clone(),
            filePath: "/node_modules/{ros_lib_filename}",
            source_code: ros_lib_dts, // Default value that is overridden.
        }),
        declarations,
        utility_files: declarations
            .iter()
            .map(|item| UserScriptProjectConfigItem {
                fileName: item["fileName"].clone(),
                filePath: format!("{DEFAULT_STUDIO_SCRIPT_PREFIX}{item["fileName"]}.js"),
                source_code: item["sourceCode"].clone(),
            })
            .collect::<Vec<UserScriptProjectConfigItem>>(),
    }
}
```

Note that this code assumes that the `export_type_script_schemas` and `raw_user_utils` functions are defined elsewhere in your Rust project. The `UserScriptProjectConfig` struct is also assumed to be defined elsewhere in your Rust project.