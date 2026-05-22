```rust
use chrono::{DateTime, Duration};
use std::time::Instant;

fn main() {
    // Given
    let mut current_time = Instant.now(); // Mock current time
    let seconds_until_stale: f64 = 5.0; // Example value
    let update_interval_ms: u32 = 1000; // Example value

    // When
    let stale_time = calculate_stale_time(&current_time, seconds_until_stale, update_interval_ms);

    // Then
    println!("Stale time: {:?}", stale_time);
}

fn calculate_stale_time(current_time: &Instant, seconds_until_stale: f64, update_interval_ms: u32) -> DateTime<chrono::Utc> {
    let now = Instant::now();
    if now.duration_since(*current_time).as_secs_f64() < seconds_until_stale as f64 {
        current_time + Duration::milliseconds(update_interval_ms)
    } else {
        *current_time
    }
}
```