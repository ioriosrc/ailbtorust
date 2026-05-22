```rust
use chrono::NaiveTime;
use chrono::{Duration, Timelike};
use itertools::Itertools;

fn main() {
    // Function to check if two chunks overlap
    fn chunk_overlap(chunk1: (NaiveTime, NaiveTime), chunk2: (NaiveTime, NaiveTime)) -> bool {
        let start_max = chunk1.0.max(chunk2.0);
        let end_min = chunk1.1.min(chunk2.1);

        start_max <= end_min
    }

    // Helper function to generate chunks from a time array
    fn generate_chunks(time_array: &[(f64, f64)]) -> Vec<(NaiveTime, NaiveTime)> {
        let mut chunks = vec![];
        for i in 0..time_array.len() - 1 {
            let start_time = NaiveTime::from_hms(
                time_array[i].0 as u32,
                time_array[i].1 as u32,
                time_array[i].0 as u32 % 60,
            );
            let end_time = NaiveTime::from_hms(
                time_array[i + 1].0 as u32,
                time_array[i + 1].1 as u32,
                time_array[i + 1].0 as u32 % 60,
            );

            chunks.push((start_time, end_time));
        }

        chunks
    }

    // Test cases
    let test_cases = vec![
        ((1.0, 2.0), (2.0, 3.0), 0),
        ((0.0, 1.0), (1.5, 2.5), 1),
        ((0.0, 1.0), (4.0, 5.0), 2),
        ((0.0, 1.0), (0.9, 1.9), 3),
    ];

    for (chunk1_start, chunk1_end, expected_overlap) in test_cases {
        let chunk1 = NaiveTime::from_hms(
            chunk1_start as u32,
            chunk1_end as u32,
            chunk1_start as u32 % 60,
        );
        let chunk2 = NaiveTime::from_hms(
            chunk1_end as u32,
            chunk2_end as u32,
            chunk2_end as u32 % 60,
        );

        assert_eq!(chunk_overlap(chunk1, chunk2), expected_overlap);
    }
}
```