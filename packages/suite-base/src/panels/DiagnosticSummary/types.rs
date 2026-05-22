```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

type DiagnosticId = u32;
type DiagnosticInfo = HashMap<String, String>;

pub type DiagnosticSummaryConfig = {
  min_level: i32,
  pinned_ids: Vec<DiagnosticId>,
  topic_to_render: String,
  hardware_id_filter: Option<String>,
  sort_by_level: bool,
  seconds_until_stale: i32,
};

pub type DiagnosticsById = HashMap<DiagnosticId, DiagnosticInfo>;

pub struct NodeRowProps {
  info: DiagnosticInfo,
  is_pinned: bool,
  onClick: fn(info: DiagnosticInfo),
  onClick_pin: fn(info: DiagnosticInfo),
}

pub struct DiagnosticSummaryProps {
  config: DiagnosticSummaryConfig,
  save_config: Box<dyn Fn(DiagnosticSummaryConfig)>,
}
```