```rust
use std::time::{Duration, Nanoseconds};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub const SAMPLE_NUSCENES_DATA_SOURCE_URL: &str = "https://mcap-proxy.lichtblick.workers.dev/NuScenes-v1.0-mini-scene-sample.mcap";
pub const SAMPLE_NUSCENES_DATA_SOURCE_ID: &str = "sample-nuscenes";
pub const SAMPLE_NUSCENES_DATA_SOURCE_TYPE: &str = "sample";
pub const SAMPLE_NUSCENES_DATA_SOURCE_DISPLAY_NAME: &str = "Sample: Nuscenes";
pub const SAMPLE_NUSCENES_DATA_SOURCE_ICON_NAME: &str = "FileASPX";
pub const SAMPLE_NUSCENES_DATA_SOURCE_NAME: &str = "Adapted from nuScenes dataset. Copyright © 2020 nuScenes. https://www.nuscenes.org/terms-of-use";
pub const SAMPLE_NUSCENES_DATA_SOURCE_READ_AHEAD_DURATION: Duration = Duration::new(10, 0);
```