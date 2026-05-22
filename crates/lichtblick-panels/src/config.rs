// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// Base panel configuration trait.
pub trait PanelConfigTrait: Serialize + for<'de> Deserialize<'de> + Default + Clone {
    /// Panel type identifier.
    fn panel_type() -> &'static str;
}

/// Common configuration for panels that subscribe to topics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopicConfig {
    /// Selected topic path.
    pub topic_path: Option<String>,
}
