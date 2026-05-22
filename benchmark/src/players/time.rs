```rust
use rust_time::Time; // Assuming you have a crate that provides Time and fromNanoSec functions

const ONE_MS_IN_NS: u64 = 1_000_000;

pub fn now() -> Time {
    let stamp_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    let stamp_ns = (stamp_ms.as_secs_f64() * ONE_MS_IN_NS) as u64;
    Time::from_nanosec(stamp_ns)
}
```