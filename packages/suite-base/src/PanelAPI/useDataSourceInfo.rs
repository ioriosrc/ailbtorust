```rust
use std::time::{Duration};

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

use chrono::{DateTime};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct RosDatatypes {
    // Define the structure of RosDatatypes here
}

#[derive(Debug, Clone)]
pub struct Topic {
    // Define the structure of Topic here
}

// Metadata about the source of data currently being displayed.
// This is not expected to change often, usually when changing data sources.
pub type DataSourceInfo = (
    Vec<Topic>,
    HashSet<String>,
    RosDatatypes,
    HashSet<String>,
    Option<DateTime<Duration>>,
    String,
);

/**
 * Data source info" encapsulates **rarely-changing** metadata about the source from which
 * Lichtblick Suite is loading data.
 *
 * A data source might be a local file, a remote file, or a streaming source.
 */
pub fn use_datasource_info() -> DataSourceInfo {
    let datatypes = use_message_pipeline(select_datatypes);
    let topics = use_message_pipeline(select_topics);
    let services = use_message_pipeline(select_services);
    let startTime = use_message_pipeline(select_start_time);
    let capabilities = use_message_pipeline(select_capabilities);
    let playerId = use_message_pipeline(select_player_id);

    // we want the returned object to have a stable identity
    (
        topics,
        services.into_iter().collect(),
        datatypes,
        capabilities.into_iter().collect(),
        startTime.map(|time| time.to_rfc3339()),
        playerId,
    )
}
```

Note: The `Topic` and `RosDatatypes` structs are assumed to be defined elsewhere in the Rust codebase.