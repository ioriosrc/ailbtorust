```rust
pub type Time = u64; // Assuming bigint is represented as u64 for simplicity in Rust
pub type Duration = u64;

/// Compares two timestamps and returns a negative value if `a` is before `b`,
/// zero if they are equal, and a positive value if `a` is after `b`.
pub fn compare_time(a: Time, b: Time) -> i32 {
  (a - b).cmp(&0)
}

/// Calculates the percentage of `target` relative to `start` and `end`.
/// It returns a floating-point number between 0.0 and 1.0.
pub fn percent_of(start: Time, end: Time, target: Time) -> f64 {
  let total_duration = (end - start).into(); // Casting u64 to i64 for division
  let target_duration = (target - start).into();
  target_duration as f64 / total_duration as f64
}

/// Interpolates between `start` and `end` based on a given fraction.
/// It returns the interpolated time value as a u64.
pub fn interpolate(start: Time, end: Time, fraction: f64) -> Time {
  let duration = (end - start).into(); // Casting u64 to i64 for division
  start + ((duration * fraction).round() as u64)
}
```