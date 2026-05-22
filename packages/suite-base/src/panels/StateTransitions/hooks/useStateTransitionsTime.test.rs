```rust
use std::time::{Duration, Instant};

// Define the Time struct
struct Time {
    sec: u64,
    nsec: u32,
}

impl Time {
    fn new(sec: u64, nsec: u32) -> Self {
        Self { sec, nsec }
    }

    fn to_duration(&self) -> Duration {
        Duration::from_secs_f64(self.sec as f64 + self.nsec as f64 / 1_000_000_000.0)
    }
}

// Define the subtract_times function
fn subtract_times(a: &Time, b: &Time) -> Time {
    let diff_sec = a.sec.checked_sub(b.sec).unwrap_or(0);
    let diff_nsec = a.nsec.checked_sub(b.nsec).unwrap_or(0);

    if diff_nsec < 0 {
        diff_nsec += 1_000_000_000;
        diff_sec -= 1;
    }

    Time::new(diff_sec, diff_nsec)
}

// Define the to_sec function
fn to_sec(time: &Time) -> f64 {
    time.sec as f64 + time.nsec as f64 / 1_000_000_000.0
}

// Implement useStateTransitionsTime in Rust
pub fn use_state_transitions_time() -> (Option<Time>, Option<f64>, Option<f64>) {
    let now = Instant::now();
    let start_time = Time::new(now.elapsed().as_secs(), now.elapsed().subsec_nanos());
    let current_time = Time::new(0, 0); // Placeholder for currentTimeSinceStart and endTimeSinceStart
    let end_time = Time::new(0, 0);

    (Some(start_time), None, None)
}

// Test the useStateTransitionsTime function in Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_state_transitions_time_with_no_data() {
        let (_, _, _) = use_state_transitions_time();
        assert!(None.is_some());
        assert!(None.is_some());
        assert!(None.is_some());
    }

    #[test]
    fn test_use_state_transitions_time_current_time_since_start() {
        let now = Instant::now();
        let start_time = Time::new(now.elapsed().as_secs(), now.elapsed().subsec_nanos());

        let current_time = Time::new(0, 0); // Placeholder for currentTimeSinceStart and endTimeSinceStart
        let end_time = Time::new(0, 0);

        let (_, _, _) = use_state_transitions_time();
        assert_eq!(start_time, _);
        assert_eq!(current_time.to_duration(), Duration::from_secs_f64(current_time.sec as f64 + current_time.nsec as f64 / 1_000_000_000.0));
        assert!(None.is_some());
    }

    #[test]
    fn test_use_state_transitions_time_end_time_since_start() {
        let now = Instant::now();
        let start_time = Time::new(now.elapsed().as_secs(), now.elapsed().subsec_nanos());

        let current_time = Time::new(0, 0); // Placeholder for currentTimeSinceStart and endTimeSinceStart
        let end_time = Time::new(0, 0);

        let (_, _, _) = use_state_transitions_time();
        assert_eq!(start_time, _);
        assert!(None.is_some());
        assert_eq!(end_time.to_duration(), Duration::from_secs_f64(end_time.sec as f64 + end_time.nsec as f64 / 1_000_000_000.0));
    }

    #[test]
    fn test_use_state_transitions_time_current_time_since_start_and_end_time_since_start() {
        let now = Instant::now();
        let start_time = Time::new(now.elapsed().as_secs(), now.elapsed().subsec_nanos());

        let current_time = Time::new(0, 0); // Placeholder for currentTimeSinceStart and endTimeSinceStart
        let end_time = Time::new(0, 0);

        let (_, _, _) = use_state_transitions_time();
        assert_eq!(start_time, _);
        assert_eq!(current_time.to_duration(), Duration::from_secs_f64(current_time.sec as f64 + current_time.nsec as f64 / 1_000_000_000.0));
        assert_eq!(end_time.to_duration(), Duration::from_secs_f64(end_time.sec as f64 + end_time.nsec as f64 / 1_000_000_000.0));
    }
}
```