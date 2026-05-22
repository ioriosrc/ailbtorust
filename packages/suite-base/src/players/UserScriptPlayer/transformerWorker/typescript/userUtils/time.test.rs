```rust
use chrono::{Duration, NaiveDateTime};

fn are_same(t1: NaiveDateTime, t2: NaiveDateTime) -> bool {
    t1.eq(&t2)
}

fn subtract_times(t1: NaiveDateTime, t2: NaiveDateTime) -> Duration {
    t1 - t2
}

fn compare(t1: NaiveDateTime, t2: NaiveDateTime) -> i64 {
    t1.cmp(&t2).signum()
}
```