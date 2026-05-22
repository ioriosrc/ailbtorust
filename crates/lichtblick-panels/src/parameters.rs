// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Configuration for the Parameters panel.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParametersConfig {
    pub filter: Option<String>,
}
