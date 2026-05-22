```rust
use std::convert::{From, Into};

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

pub const ARROW_SEEK_BIG_MS: u32 = 500;
pub const ARROW_SEEK_DEFAULT_MS: u32 = 100;
pub const ARROW_SEEK_SMALL_MS: u32 = 10;

#[derive(Debug, PartialEq)]
pub enum Direction {
    FORWARD,
    BACKWARD,
}

fn jump_seek(
    direction_sign: Direction,
    current_time: Time,
    modifier_keys: Option<(bool, bool)>,
    default_step_size: Option<u32>,
): Time {
    let time_ms = to_millis(current_time);

    let correct_seek_value =
        default_step_size.map(|s| s > 0).unwrap_or(default_step_size.is_some()) && correct_step_value;

    let delta_ms: u32;
    if modifier_keys.unwrap_or((false, false)).1 {
        delta_ms = ARROW_SEEK_BIG_MS;
    } else if modifier_keys.unwrap_or((false, false)).0 {
        delta_ms = ARROW_SEEK_SMALL_MS;
    } else {
        delta_ms = correct_seek_value.unwrap_or(ARROW_SEEK_DEFAULT_MS);
    }

    from_millis(time_ms + delta_ms * direction_sign as u32)
}

fn to_millis(time: Time) -> u32 {
    time.into()
}

fn from_millis(ms: u32) -> Time {
    Time::from_microseconds(ms as i64 * 1000)
}
```