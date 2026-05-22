```rust
use std::str::FromStr;

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

// No time functions that require `moment` should live in this file.
use log::{error, trace};
use rostime::{from_rfc3339_string, from_string};

/**
 * Formats a Time object as a string.
 *
 * @param stamp - The Time object to format.
 * @returns A formatted string representing the time.
 */
fn format_time_raw(stamp: &rostime::Time) -> String {
    if stamp.sec < 0 || stamp.nsec < 0 {
        error!("Times are not allowed to be negative");
        return "(invalid negative time)";
    }
    format!("{:}-{:09}", stamp.sec, stamp.nsec)
}

const DURATION_20_YEARS_SEC = 20 * 365 * 24 * 60 * 60;

// Values "too small" to be absolute epoch-based times are probably relative durations.
fn is_absolute_time(time: &rostime::Time) -> bool {
    time.sec > DURATION_20_YEARS_SEC
}

/**
 * Formats a Time object as a string.
 *
 * @param stamp - The Time object to format.
 * @returns A formatted string representing the time.
 */
fn format_frame(stamp: &rostime::Time) -> String {
    format!("{:}-{:09}", stamp.sec, stamp.nsec)
}

/**
 * Retrieves the timestamp for a given message event.
 *
 * @param messageEvent - The message event containing the timestamp.
 * @param timestamp_method - The method to use for retrieving the timestamp. Can be "headerStamp" or "receiveTime".
 * @returns The timestamp if available, or None otherwise.
 */
fn get_timestamp_for_message_event(
    message_event: &rostime::MessageEvent,
    timestamp_method: &str,
) -> Option<rostime::Time> {
    match timestamp_method {
        "headerStamp" => {
            if let Some(header_stamp) = &message_event.message.header.as_ref() {
                if header_stamp.stamp.is_some() {
                    return header_stamp.stamp.clone();
                }
            }
            None
        }
        "receiveTime" => message_event.receive_time.clone(),
        _ => unreachable!("Unsupported timestamp method: {}", timestamp_method),
    }
}

/**
 * Retrieves the timestamp for a given message.
 *
 * @param message - The message containing the timestamp.
 * @returns The timestamp if available, or None otherwise.
 */
fn get_timestamp_for_message(message: &rostime::Message) -> Option<rostime::Time> {
    if let Some(header_stamp) = &message.header.as_ref() {
        if header_stamp.stamp.is_some() {
            return header_stamp.stamp.clone();
        }
    } else if let Some(marker_array) = message as &rostime::MarkerArray {
        for marker in marker_array.markers.iter() {
            if let Some(header_stamp) = &marker.header.as_ref() {
                if header_stamp.stamp.is_some() {
                    return header_stamp.stamp.clone();
                }
            }
        }
    }

    None
}

/**
 * Parses a time string into a Time object.
 *
 * @param time_string - The time string to parse.
 * @returns A Time object if parsing succeeds, or None otherwise.
 */
fn parse_time_url_string(time_string: &str) -> Option<rostime::Time> {
    if time_string.is_empty() {
        return None;
    }

    let parsed_time = from_rfc3339_string(time_string).ok();
    if let Some(parsed_time) = parsed_time {
        return Some(parsed_time);
    }

    // Fallback to parsing the raw sec.nsec format
    match str::parse::<(u64, u32)>(&time_string) {
        Ok((sec, nsec)) => Some(rostime::Time::new(sec, nsec)),
        Err(_) => None,
    }
}
```