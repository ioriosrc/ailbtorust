```rust
use std::sync::{Arc, Mutex};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use chart_js::dataset::ChartDataset;
use message_path::{MessagePath, Time};
use suite::datum::{Datum, Immutable};

type DatumWithReceiveTime = Datum & {
  receive_time: Time;
};

/**
 * Series item type shared by builders that work only with the current frame (no accumulation).
 * Used by IndexDatasetsBuilder and CurrentCustomDatasetsBuilder.
 */
pub struct CurrentFrameSeriesItem {
  config_index: usize,
  enabled: bool,
  message_path: String,
  parsed: Arc<Mutex<MessagePath>>,
  dataset: ChartDataset<"scatter", DatumWithReceiveTime>,
}
```