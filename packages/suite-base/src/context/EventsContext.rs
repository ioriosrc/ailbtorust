```rust
use std::sync::{Arc, Mutex};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use chrono::NaiveTime;

/**
 * DataSourceEvent representsing a single event within a data source.
 */
#[derive(Debug)]
pub struct DataSourceEvent {
    pub id: String;
    pub created_at: NaiveTime;
    pub device_id: String;
    pub duration_nanos: u64,
    pub end_time: NaiveTime,
    pub end_time_seconds: f64,
    pub metadata: std::collections::HashMap<String, String>,
    pub start_time: NaiveTime,
    pub start_time_seconds: f64,
    pub timestamp_nanos: u64,
    pub updated_at: NaiveTime,
}

/**
 * Represents an event including its fractional position on the timeline.
 */
#[derive(Debug)]
pub struct TimelinePositionedEvent {
    /** The event. */
    pub event: DataSourceEvent;

    /** The end position of the event, as a value 0-1 relative to the timeline. */
    pub end_position: f64,

    /** The start position of the event, as a value 0-1 relative to the timeline. */
    pub start_position: f64,

    /** The time, in seconds, relative to the start of the timeline. */
    pub seconds_since_start: f64,
}

pub type EventsStore = Arc<Mutex<crate::store::EventsStore>>;
```