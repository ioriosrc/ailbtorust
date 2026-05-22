```rust
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct Diagnostic {
    severity: &'static str,
    message: String,
    source: &'static str,
    start_line_number: Option<u32>,
    start_column: Option<u32>,
    end_line_number: Option<u32>,
    end_column: Option<u32>,
    code: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptData {
    name: String,
    source_code: String,
    transpiled_code: String,
    project_code: Option<Map<String, String>>,
    diagnostics: Vec<Diagnostic>,
    input_topics: Vec<String>,
    output_topic: String,
    output_datatype: String,
    datatypes: RosDatatypes,
    source_file: Option<&'static str>,
    type_checker: Option<&'static str>,
    ros_lib: &'static str,
    types_lib: &'static str,
    global_variables: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptRegistration {
    script_id: String,
    script_data: ScriptData,
    inputs: Vec<String>,
    output: Topic,
    process_block_message: fn(&'static str, &GlobalVariables) -> Option<MessageEvent>,
    process_message: fn(&'static str, &GlobalVariables) -> Option<MessageEvent>,
    terminate: fn(),
}

#[derive(Serialize, Deserialize)]
pub struct ScriptDataTransformer {
    script_data: ScriptData,
    topics: Vec<Topic>,
}

#[derive(Serialize, Deserialize)]
pub struct UserScriptLog {
    source: &'static str,
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RegistrationOutput {
    error: Option<String>,
    user_script_logs: Vec<UserScriptLog>,
    user_script_diagnostics: Vec<Diagnostic>,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessMessageOutput {
    message: serde_json::Value,
    error: Option<String>,
    user_script_logs: Vec<UserScriptLog>,
    user_script_diagnostics: Vec<Diagnostic>,
}
```