```rust
use std::fs::File;
use std::io::{BufReader, BufWrite};
use std::path::Path;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use lightrabbit::players::IterablePlayer;
use lightrabbit::sources::iterable_source::{IterableSourceConfig, IterableSource};
use lightrabbit::utils::metrics::MetricsCollector;

struct SampleNuscenesDataSourceFactory {
    id: &'static str,
    type_: &'static str,
    display_name: &'static str,
    icon_name: &'static str,
    hidden: bool,
    sample_layout: &'static dyn std::any::Any,
}

impl SampleNuscenesDataSourceFactory {
    fn new() -> Self {
        Self {
            id: SAMPLE_NUSCENES_DATA_SOURCE_ID,
            type_: SAMPLE_NUSCENES_DATA_SOURCE_TYPE,
            display_name: SAMPLE_NUSCENES_DATA_SOURCE_DISPLAY_NAME,
            icon_name: SAMPLE_NUSCENES_DATA_SOURCE_ICON_NAME,
            hidden: true,
            sample_layout: SampleNuscenesLayout as &'static dyn std::any::Any,
        }
    }

    fn initialize(&self, args: MetricsCollector) -> Result<IterablePlayer, Box<dyn std::error::Error>> {
        let bag_url = SAMPLE_NUSCENES_DATA_SOURCE_URL;
        let mut file = File::open(bag_url)?;
        let reader = BufReader::new(file);
        let config = IterableSourceConfig::from_reader(reader)?;

        Ok(IterablePlayer::new(
            Self::create_worker(config.clone()),
            is_sample_data_source: true,
            name: SAMPLE_NUSCENES_DATA_SOURCE_NAME,
            metrics_collector: args,
            url_params: {},
            source_id: self.id,
            read-ahead_duration: SAMPLE_NUSCENES_DATA_SOURCE_READ_AHEAD_DURATION,
        ))
    }

    fn create_worker(config: IterableSourceConfig) -> Box<dyn IterableSource> {
        let worker_path = Path::new("@lichtblick/suite-base/players/IterablePlayer/Mcap/McapIterableSourceWorker.worker");
        Box::new(McapIterableSourceWorker::new(worker_path, config))
    }
}

struct McapIterableSourceWorker {}

impl McapIterableSourceWorker {
    fn new(worker_path: &Path, config: IterableSourceConfig) -> Self {
        McapIterableSourceWorker {}
    }

    // Implement the logic to create and manage the worker here
}
```