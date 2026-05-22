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

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::message_pipeline::{MessagePipelineContext, MessagePipelineProvider};
use crate::model::{Capability, ParameterValue};
use crate::ui::components::{CopyButton, JsonInput};
use crate::ui::models::{EditableValue, DisplayableValue};
use crate::ui::styles::makeStyles;
use crate::utils::debounce;

const ANIMATION_RESET_DELAY_MS: u32 = 3000;

fn isActiveElementEditable() -> bool {
    let active_el = document.active_element();
    return (
        active_el.is_not_none()
            && ((active_el as &HTMLElement).is_content_editable()
                || active_el.tag_name() == "INPUT"
                || active_el.tag_name() == "TEXTAREA")
    );
}

fn editable_value(value: &ParameterValue) -> String {
    match value {
        ParameterValue::Uint8Array(bytes) => bytes.iter().map(|byte| byte.to_string()).collect::<String>(),
        _ => format!("{:?}", value),
    }
}

fn displayable_value(value: &ParameterValue) -> String {
    match value {
        ParameterValue::Uint8Array(bytes) => bytes.iter().map(|byte| byte.to_string()).collect::<String>(),
        _ => format!("{}", value),
    }
}

struct SubmittableJsonInput {
    value: EditableValue,
    on_submit: Box<dyn Fn(EditableValue)>,
}

impl SubmittableJsonInput {
    fn new(value: EditableValue, on_submit: impl Fn(EditableValue) + 'static) -> Self {
        Self {
            value,
            on_submit: Box::new(on_submit),
        }
    }

    fn render(&self) -> Html {
        html! {
            <Stack direction="row">
                <JsonInput
                    value={self.value}
                    onChange={|value| self.set_value(value)}
                />
                {!self.is_equivalent(self.value.clone()) && [
                    <Tooltip key="submit" title="Submit change">
                        <IconButton
                            onClick={|| {
                                (self.on_submit)(self.value);
                            }}
                        >
                            <CheckIcon />
                        </IconButton>
                    </Tooltip>,
                    <Tooltip key="reset" title="Reset">
                        <IconButton
                            key="reset"
                            onClick={|| {
                                self.set_value(EditableValue::default());
                            }}
                        >
                            <ClearIcon />
                        </IconButton>
                    </Tooltip>,
                ]}
            </Stack>
        }
    }

    fn set_value(&mut self, new_value: EditableValue) {
        if !self.is_equivalent(new_value.clone()) {
            self.value = new_value;
            self.update_event();
        }
    }

    fn is_equivalent(&self, other: EditableValue) -> bool {
        self.value == other
    }

    fn update_event(&self) {
        let updated_parameters = self.get_updated_parameters().collect::<HashMap<_, _>>();
        for (name, value) in &updated_parameters {
            if name != &self.value.name() {
                // Notify the component that a parameter has changed
                notify_parameter_changed(name, value);
            }
        }
    }

    fn get_updated_parameters(&self) -> HashMap<String, ParameterValue> {
        let mut updated_parameters = HashMap::new();
        for (name, value) in &self.value.data() {
            if name != &self.value.name() {
                updated_parameters.insert(name.clone(), value.clone());
            }
        }
        updated_parameters
    }
}

struct Parameters {
    capabilities: Arc<RwLock<HashMap<Capability, bool>>>,
    set_parameter_unbounced: Box<dyn Fn(String, ParameterValue)>,
    parameters: RwLock<HashMap<String, ParameterValue>>,
    changed_parameters: Arc<RwLock<Vec<String>>>,
}

impl Parameters {
    fn new(capabilities: Arc<RwLock<HashMap<Capability, bool>>>, set_parameter_unbounced: Box<dyn Fn(String, ParameterValue)>)
        -> Self
    {
        Self {
            capabilities,
            set_parameter_unbounced,
            parameters: RwLock::new(HashMap::new()),
            changed_parameters: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn get_capabilities(&self) -> Arc<RwLock<HashMap<Capability, bool>>> {
        self.capabilities.clone()
    }

    fn set_parameter_unbounced(&self, name: String, value: ParameterValue) {
        (self.set_parameter_unbounced)(name, value);
    }

    fn get_parameters(&self) -> Arc<RwLock<HashMap<String, ParameterValue>>> {
        self.parameters.clone()
    }

    fn get_changed_parameters(&self) -> Arc<RwLock<Vec<String>>> {
        self.changed_parameters.clone()
    }
}

fn notify_parameter_changed(name: &str, value: &ParameterValue) {
    // Notify the component that a parameter has changed
    // This could involve updating the UI or triggering an event
}

impl MessagePipelineProvider for Parameters {
    fn get_capabilities(&self) -> Arc<RwLock<HashMap<Capability, bool>>> {
        self.get_capabilities()
    }

    fn set_parameter_unbounced(&self, name: String, value: ParameterValue) {
        self.set_parameter_unbounced(name, value);
    }

    fn get_parameters(&self) -> Arc<RwLock<HashMap<String, ParameterValue>>> {
        self.get_parameters()
    }
}

fn parameters_panel_type() -> &'static str {
    "Parameters"
}

fn default_config() -> HashMap<&'static str, serde_json::Value> {
    serde_json::json!({
        "title": "Parameters",
    })
}

fn main() {
    // Main function logic here
}
```