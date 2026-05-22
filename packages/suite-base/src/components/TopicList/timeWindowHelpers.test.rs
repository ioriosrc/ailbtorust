```rust
use chrono::{DateTime, NaiveDateTime, Utc};

fn subtract_milliseconds(time: DateTime<Utc>, ms: u32) -> DateTime<Utc> {
    let mut datetime = time.clone();
    let nanos = datetime.nanosecond() as i64 - ms * 1000 * 1000;
    if nanos < 0 {
        datetime = datetime.checked_sub(Duration::nanoseconds(-nanos)).unwrap();
    }
    datetime
}

fn time_to_sec(time: DateTime<Utc>) -> f64 {
    let secs = time.timestamp() as f64 / 1_000.0;
    let nanosecs = time.nanosecond() as f64;
    secs + nanosecs * 1e-9
}

fn calculate_optimal_window_ms(
    num_messages: usize,
    first_message_time: DateTime<Utc>,
    last_message_time: DateTime<Utc>,
    target_messages_in_window: usize,
) -> u32 {
    let time_diff = (last_message_time - first_message_time).num_milliseconds() as f64;
    if time_diff <= 0.0 {
        return 500; // Default window size
    }

    let num_windows = ((time_diff / target_messages_in_window as f64) + 1.0).ceil() as usize;

    let window_size = (time_diff as f64 / num_windows as f64).round() as u32;
    if window_size < 100 {
        100 // Clamp to minimum
    } else if window_size > 30_000 {
        30_000 // Clamp to maximum
    } else {
        window_size
    }
}

fn create_window_sizes(num_messages: usize, num_windows: usize) -> Vec<u32> {
    let mut windows = Vec::with_capacity(num_windows);
    let step = (1000.0 * 1000 as f64 / num_messages as f64).ceil() as u32;

    for i in 0..num_windows {
        let window_size = step * (i + 1);
        windows.push(window_size);
    }

    // Clamp to maximum
    if windows.len() > 1 && windows[windows.len() - 1] > 30_000 {
        windows.pop();
        windows.push(30_000);
    }

    windows
}

fn would_reach_boundary(current_time: DateTime<Utc>, window_size: u32, boundary_time: Option<DateTime<Utc>>) -> bool {
    if let Some(boundary_time) = boundary_time {
        // Calculate the time when the window reaches or exceeds the boundary
        let end_time = boundary_time - Duration::milliseconds(window_size as i64);
        
        // Check if the current time is before the end time
        return current_time < end_time;
    }

    false
}
```