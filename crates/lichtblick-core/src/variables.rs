// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::VariableValue;

/// Global variables accessible to all panels via the message pipeline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Variables {
    pub values: HashMap<String, VariableValue>,
}

impl Variables {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, name: &str) -> Option<&VariableValue> {
        self.values.get(name)
    }

    pub fn set(&mut self, name: String, value: VariableValue) {
        self.values.insert(name, value);
    }

    pub fn remove(&mut self, name: &str) -> Option<VariableValue> {
        self.values.remove(name)
    }
}
