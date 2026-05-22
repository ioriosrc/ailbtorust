```rust
use std::cmp;

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
use crate::Range;
use crate::{is_overlapping, missing_ranges};

const READ_AHEAD_BUFFER_SIZE: u64 = 50 * 1024 * 1024; // 50 MB

// Based on a number of properties this function determines if a new connection should be opened or
// not. It can be used for any type of ranges, be it bytes, timestamps, or something else.
pub fn get_new_connection(options: {
    current_remaining_range: Option<Range>, // The remaining range that the current connection (if any) is going to download.
    read_request_range: Option<Range>; // The range of the read request that we're trying to satisfy.
    downloaded_ranges: Vec<Range>; // Array of ranges that have been downloaded already.
    last_resolved_callback_end: Option<u64>; // The range.end of the last read request that we resolved. Useful for reading ahead a bit.
    max_request_size: u64; // The cache size. If equal to or larger than `file_size` we will attempt to download the whole file.
    fileSize: u64; // Size of the file.
    continue_downloading_threshold: u64; // Amount we're willing to wait downloading before opening a new connection.
}) -> Option<Range> {
    let read_request_range = options.read_request_range;
    let current_remaining_range = options.current_remaining_range;
    if let Some(read_request_range) = read_request_range {
        return get_new_connection_with_existing_read_request(read_request_range, options);
    } else if let Some(current_remaining_range) = current_remaining_range {
        return get_new_connection_without_existing_connection(current_remaining_range, options);
    }
    None
}

fn get_new_connection_with_existing_read_request(
    read_request_range: Range,
    options: {
        current_remaining_range: Option<Range>; // The remaining range that the current connection (if any) is going to download.
        downloaded_ranges: Vec<Range>; // Array of ranges that have been downloaded already.
        last_resolved_callback_end: Option<u64>; // The range.end of the last read request that we resolved. Useful for reading ahead a bit.
        max_request_size: u64; // The cache size. If equal to or larger than `file_size` we will attempt to download the whole file.
        fileSize: u64; // Size of the file.
        continue_downloading_threshold: u64; // Amount we're willing to wait downloading before opening a new connection.
    },
) -> Option<Range> {
    // We have a requested range that we're trying to download.
    if read_request_range.end - read_request_range.start > options.max_request_size {
        // This should have been caught way earlier, but just as a sanity check.
        return Err("Range exceeds max request size");
    }

    // Get the parts of the requested range that have not been downloaded yet.
    let not_downloaded_ranges = missing_ranges(read_request_range, &options.downloaded_ranges);

    if !not_downloaded_ranges[0].is_some() {
        // If there aren't any, then we should have never passed in `read_request_range`.
        return Err("Range for the first read request is fully downloaded, so it should have been deleted");
    }

    // We want to start a new connection if:
    let start_new_connection = match (options.current_remaining_range, not_downloaded_ranges[0]) {
        (None, _) | (_, None) => false,
        (Some(current_remaining), Some(range)) if !is_overlapping(&range, &current_remaining) => true,
        (Some(_), Some(range)) if range.start + options.continue_downloading_threshold < range.end => true,
        _ => false,
    };

    if !start_new_connection {
        return None;
    }
    if options.max_request_size >= options.file_size {
        // If we're trying to download the whole file, read all the way up to the next range that we have already downloaded.
        let range = match not_downloaded_ranges[0] {
            Some(range) => range,
            None => return None,
        };
        return missing_ranges(range, &options.downloaded_ranges)[0];
    }

    if not_downloaded_ranges[0].end == read_request_range.end {
        // If we're downloading to the end of our range, do some reading ahead while we're at it.
        let potential_range = match options.last_resolved_callback_end {
            Some(last_resolved_callback_end) => Range {
                start: last_resolved_callback_end,
                end: options.file_size,
            },
            None => Range { start: 0, end: options.file_size },
        };
        if !is_range_coveredByRanges(&potential_range, &options.downloaded_ranges) {
            return Some(potential_range);
        } else {
            return Some(Range { start: 0, end: options.file_size });
        }
    }

    // Otherwise, start reading from the first non-downloaded range.
    let range = match not_downloaded_ranges[0] {
        Some(range) => range,
        None => return None,
    };
    return missing_ranges(range, &options.downloaded_ranges)[0];
}

fn get_new_connection_without_existing_connection(
    options: {
        downloaded_ranges: Vec<Range>; // Array of ranges that have been downloaded already.
        last_resolved_callback_end: Option<u64>; // The range.end of the last read request that we resolved. Useful for reading ahead a bit.
        max_request_size: u64; // The cache size. If equal to or larger than `file_size` we will attempt to download the whole file.
        fileSize: u64; // Size of the file.
    },
) -> Option<Range> {
    // If we don't have any read requests, and we also don't have an active connection, then start
    // reading ahead as much data as we can!
    let mut read-ahead_range: Option<Range> = None;
    if options.max_request_size >= options.file_size {
        // If we have an unlimited cache, we want to read the entire file, but still prefer downloading
        // first near where the last request happened.
        let potential_range = match options.last_resolved_callback_end {
            Some(last_resolved_callback_end) => Range {
                start: last_resolved_callback_end,
                end: options.file_size,
            },
            None => Range { start: 0, end: options.file_size },
        };
        if !is_range_coveredByRanges(&potential_range, &options.downloaded_ranges) {
            read-ahead_range = Some(potential_range);
        } else {
            read-ahead_range = Some(Range { start: 0, end: options.file_size });
        }
    } else if let Some(last_resolved_callback_end) = options.last_resolved_callback_end {
        // Otherwise, if we have a limited cache, we want to read the data right after the last
        // read request, because usually read requests are sequential without gaps.
        let potential_range = Range {
            start: last_resolved_callback_end,
            end: cmp::min(last_resolved_callback_end + READ_AHEAD_BUFFER_SIZE, options.file_size),
        };
        if !is_range_coveredByRanges(&potential_range, &options.downloaded_ranges) {
            read-ahead_range = Some(potential_range);
        }
    }
    if let Some(read-ahead_range) = read-ahead_range {
        // If we have a range that we want to read ahead, then create a new connection for the range
        // within it that has not already been downloaded.
        return missing_ranges(read-ahead_range, &options.downloaded_ranges)[0];
    }
    None
}
```