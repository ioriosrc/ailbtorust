```rust
use chrono::{NaiveTime, Utc};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

mod source_time_overlap {
    // ... existing code
}

fn make_source(start: Option<NaiveTime>, end: Option<NaiveTime>) -> Box<dyn Iterator<Item = Time>> {
    // ... existing code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_overlaps_range() {
        let source = make_source(None, None);
        assert_eq!(source_overlaps_range(&source, None, None), true);

        let source = make_source(None, None);
        assert_eq!(source_overlaps_range(&source, NaiveTime::from_hms(20, 0, 0)), true);

        let source = make_source(NaiveTime::from_hms(10, 0, 0), NaiveTime::from_hms(30, 0, 0));
        assert_eq!(source_overlaps_range(&source, NaiveTime::from_hms(20, 0, 0)), true);

        let source = make_source(NaiveTime::from_hms(15, 0, 0), NaiveTime::from_hms(25, 0, 0));
        assert_eq!(source_overlaps_range(&source, NaiveTime::from_hms(10, 0, 0)), true);

        let source = make_source(NaiveTime::from_hms(0, 0, 0), NaiveTime::from_hms(100, 0));
        assert_eq!(source_overlaps_range(&source, NaiveTime::from_hms(10, 0, 0)), true);

        let source = make_source(NaiveTime::from_hms(0, 0, 0), NaiveTime::from_hms(5, 0, 0));
        assert_eq!(source_overlaps_range(&source, NaiveTime::from_hms(10, 0, 0)), false);

        let source = make_source(NaiveTime::from_hms(30, 0, 0), NaiveTime::from_hms(40, 0));
        assert_eq!(source_overlaps_range(&source, NaiveTime::from_hms(10, 0, 0)), false);

        let source = make_source(NaiveTime::from_hms(20, 0, 0), NaiveTime::from_hms(30, 0, 0));
        assert_eq!(source_overlaps_range(&source, NaiveTime::from_hms(15, 0, 0)), true);
    }

    #[test]
    fn test_filter_sources_by_time_range() {
        let sources = vec![
            make_source(None, None),
            make_source(None, None),
        ];

        assert_eq!(filter_sources_by_time_range(&sources), vec![&sources[0], &sources[1]]);

        let source1 = make_source(None, None);
        let source2 = make_source(NaiveTime::from_hms(50, 0, 0), NaiveTime::from_hms(60, 0));
        let sources = vec![source1, source2];

        assert_eq!(filter_sources_by_time_range(&sources, NaiveTime::from_hms(20, 0, 0)), vec![&sources[1]]);

        let source = make_source(None, None);
        assert_eq!(filter_sources_by_time_range(&vec![source]), Vec::<Box<dyn Iterator<Item = Time>>>::new());
    }

    #[test]
    fn test_filter_sources_for_backfill() {
        let sources = vec![
            make_source(None, None),
            make_source(NaiveTime::from_hms(15, 0, 0), NaiveTime::from_hms(25, 0)),
        ];

        assert_eq!(filter_sources_for_backfill(&sources, NaiveTime::from_hms(20, 0, 0)), vec![&sources[0]]);

        let source = make_source(None, None);
        assert_eq!(filter_sources_for_backfill(&vec![source]), Vec::<Box<dyn Iterator<Item = Time>>>::new());
    }
}
```