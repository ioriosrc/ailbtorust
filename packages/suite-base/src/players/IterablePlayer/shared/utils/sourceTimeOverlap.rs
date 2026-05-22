```rust
use chrono::NaiveDateTime; // Assuming chrono crate is used for Time

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

fn source_overlaps_range<T>(source: &[T], start: Option<NaiveDateTime>, end: Option<NaiveDateTime>) -> bool {
    // If no query range is specified, all sources are relevant
    if start.is_none() && end.is_none() {
        return true;
    }

    let source_start = source.get(0).map(|s| s.start().unwrap());
    let source_end = source.get(0).map(|s| s.end().unwrap());

    // If the source doesn't report its time range, assume it's relevant
    if source_start.is_none() || source_end.is_none() {
        return true;
    }

    // Two ranges [A, B] and [C, D] overlap iff A <= D && C <= B
    let start_time = source_start.unwrap_or(NaiveDateTime::from_utc(&0, &NaiveTime::now()));
    let end_time = source_end.unwrap_or(NaiveDateTime::from_utc(&0, &NaiveTime::now()));

    return compare(start_time, end_time) <= 0 && compare(start_time, start.unwrap_or(NaiveDateTime::from_utc(&0, &NaiveTime::now()))) <= 0;
}

fn filter_sources_by_time_range<T>(sources: &[T], start: Option<NaiveDateTime>, end: Option<NaiveDateTime>) -> Vec<&T> {
    sources.iter().filter(|source| source_overlaps_range(source, start, end)).collect()
}

fn filter_sources_for_backfill<T>(sources: &[T], time: NaiveDateTime) -> Vec<&T> {
    sources.iter().filter(|source| source.start() <= &time).collect()
}
```