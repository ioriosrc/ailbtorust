```rust
use chrono::{DateTime, Utc};
use chrono_duration_format::{parse_duration_str, DurationFormatter};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2018-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use chrono::{TimeZone, Utc};
use chrono_duration_format::{parse_duration_str, DurationFormatter};

use crate::rostime::Time;

pub fn format(stamp: Time, timezone: Option<&str>) -> String {
    let date = stamp.to_date();
    let time = stamp.to_time();
    let mut formatter = DurationFormatter::new("h:mm:ss.SSS A z");
    if timezone.is_some() {
        formatter.set_format("{0}");
    }
    let formatted_time = time.format(&formatter).to_string();
    format!("{} {}", date.format("%Y-%m-%d"), formatted_time)
}

pub fn parse_time_str(str: &str, timezone: Option<&str>) -> Time {
    if !is_valid_time_str(str) {
        return Time::default();
    }
    let parsed_date = str.split_whitespace().next().unwrap_or("");
    let parsed_time = str.split_whitespace().last().unwrap_or("");
    let date = DateTime::<Utc>::from_str(&format!("{} {}", parsed_date, parsed_time), "%Y-%m-%d %H:%M:%S.%f").unwrap();
    Time::new(date)
}

fn is_valid_time_str(str: &str) -> bool {
    str.matches(r"^\d+-\d+-\d+\s+\d+:\d+:\d+.\d+\s[PpAa][Mm]\s[A-Za-z$]+$")
}

const TOD_DATE_TIME_REGEX = r"^\d+-\d+-\d+\s+\d+:\d+:\d+.\d+\s[PpAa][Mm]\s[A-Za-z$]+";

pub fn get_validated_time_and_method_from_string(text: Option<&str>, timezone: Option<&str>) -> Option<(Time, TimeDisplayMethod)> {
    if text.is_none() || text.unwrap_or("") == "" {
        return None;
    }
    let is_invalid_raw_time = !text.is_numeric();
    let is_invalid_tod_time = tod_date_time_regex.is_match(text) && parse_fuzzy_ros_time(text).is_none();

    if is_invalid_raw_time && is_invalid_tod_time {
        return None;
    }

    Some((
        parse_fuzzy_ros_time(text)
            .unwrap_or_else(|_| Time::new(DateTime::<Utc>::from_str("0001-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())),
        if is_invalid_raw_time {
            "TOD".to_string()
        } else {
            "SEC".to_string()
        },
    ))
}

fn main() {
    let timestamp = Time::new(DateTime::<Utc>::from_str("2023-10-05 14:30:45.678", "%Y-%m-%d %H:%M:%S").unwrap());
    println!("{}", format(timestamp, Some("UTC")));

    let time_str = "2023-10-05 14:30:45.678";
    let parsed_time = parse_time_str(time_str, None);
    println!("{}", parsed_time);

    let validated_time_and_method = get_validated_time_and_method_from_string(Some("2023-10-05 14:30:45.678"), Some("UTC"));
    if let Some((time, method)) = validated_time_and_method {
        println!("Time: {}, Method: {}", time, method);
    }
}
```