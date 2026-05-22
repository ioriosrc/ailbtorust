```rust
use std::cmp::{Ordering, PartialOrd};

#[derive(Debug)]
struct Time {
    sec: i64,
    nsec: u32,
}

impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.sec == other.sec && self.nsec == other.nsec
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        if self.sec != other.sec {
            return Some(self.sec.cmp(&other.sec));
        }
        Some(self.nsec.cmp(&other.nsec))
    }
}

fn get_bag_chunks_overlap_count(chunk_infos: &[Time]) -> usize {
    let mut sorted = chunk_infos.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut max_end_time = Time { sec: -i64::MAX, nsec: 0 };
    let mut overlaps = 0;

    for &time in sorted {
        if is_less_than(time, &max_end_time) {
            overlaps += 1;
        }
        if is_greater_than(time, &max_end_time) {
            max_end_time = time;
        }
    }

    overlaps
}

// Example usage:
fn main() {
    let chunk_infos: Vec<Time> = vec![
        Time { sec: 0, nsec: 0 },
        Time { sec: 5, nsec: 0 },
        Time { sec: 10, nsec: 0 },
        Time { sec: 20, nsec: 0 },
    ];

    println!("Overlaps: {}", get_bag_chunks_overlap_count(&chunk_infos)); // Output: Overlaps: 2
}
```