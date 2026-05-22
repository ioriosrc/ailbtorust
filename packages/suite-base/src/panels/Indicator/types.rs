```rust
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

#[derive(Serialize, Deserialize)]
pub struct IndicatorOperator {
    equals: bool,
    less_than: bool,
    less_than_or_equal_to: bool,
    greater_than: bool,
    greater_than_or_equal_to: bool,
}

#[derive(Serialize, Deserialize)]
pub enum IndicatorStyle {
    Bulb,
    Background,
}

#[derive(Serialize, Deserialize)]
pub struct IndicatorRule {
    color: String,
    label: String,
    operator: IndicatorOperator,
    raw_value: RawValueIndicator,
}

#[derive(Serialize, Deserialize)]
pub type RawValueIndicator = Option<bool | i64 | f64 | String>;

#[derive(Serialize, Deserialize)]
pub struct IndicatorConfig {
    fallback_color: String,
    fallback_label: String,
    path: String,
    rules: Vec<IndicatorRule>,
    style: IndicatorStyle,
}

#[derive(Serialize, Deserialize)]
pub struct IndicatorProps {
    context: PanelExtensionContext,
}
```