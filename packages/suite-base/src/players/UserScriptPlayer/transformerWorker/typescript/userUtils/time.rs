```rust
use std::cmp;

// Define the Time struct
#[derive(Debug)]
struct Time {
    sec: i64,
    nsec: i32,
}

// Checks ROS-time equality
pub fn are_same(t1: &Time, t2: &Time) -> bool {
    t1.sec == t2.sec && t1.nsec == t2.nsec
}

// Compare two times and return a negative value if the right is greater or a positive value if the left is greater or 0 if the times are equal
pub fn compare(t1: &Time, t2: &Time) -> i32 {
    let sec_diff = t1.sec.cmp(&t2.sec);
    match sec_diff {
        std::cmp::Ordering::Equal => t1.nsec.cmp(&t2.nsec),
        _ => sec_diff,
    }
}

// Fix time to be non-negative and less than 1e9
pub fn fix_time(t: &Time) -> Time {
    let mut sec = t.sec;
    let mut nsec = t.nsec;

    while nsec > 1_000_000_000 {
        nsec -= 1_000_000_000;
        sec += 1;
    }

    while nsec < 0 {
        nsec += 1_000_000_000;
        sec -= 1;
    }

    Time { sec, nsec }
}

// Subtract two times
pub fn subtract_times(t1: &Time, t2: &Time) -> Time {
    let diff = Time {
        sec: t1.sec - t2.sec,
        nsec: t1.nsec - t2.nsec,
    };

    fix_time(&diff)
}
```