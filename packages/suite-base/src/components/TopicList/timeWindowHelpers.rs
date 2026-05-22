```rust
use std::cmp;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

/// Time conversion constants
const NANOSECONDS_PER_MILLISECOND: u64 = 1_000_000;
const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;

/// Window size constants (in milliseconds)
const DEFAULT_WINDOW_MS: u64 = 500;
const MIN_WINDOW_MS: u64 = 100;
const MAX_WINDOW_MS: u64 = 30 * 1000; // 30 seconds
const WINDOW_CAP_MS: u64 = 60 * 1000; // 1 minute

/// Adaptive window calculation constants
const DEFAULT_TARGET_MESSAGES: u64 = 10;
const DEFAULT_MAX_WINDOWS: u64 = 4;
const WINDOW_GROWTH_FACTOR: f64 = 5.0;

/**
 * Subtracts milliseconds from a Time, handling nanosecond overflow correctly.
 */
pub fn subtract_ms(time: Time, milliseconds: u64) -> Time {
  // Round to ensure we get integer values (milliseconds may be floating point from calculations)
  let nanoseconds = (milliseconds as f64 * NANOSECONDS_PER_MILLISECOND) as u64;
  let nsec = time.nsec - nanoseconds;
  let sec = time.sec;

  // Handle nanosecond underflow
  while nsec < 0 {
    nsec += NANOSECONDS_PER_SECOND;
    sec -= 1;
  }

  // Ensure nsec is an integer (avoid floating point precision issues)
  Time { sec, nsec }
}

/**
 * Converts Time to seconds (decimal).
 */
pub fn time_to_sec(time: Time) -> f64 {
  time.sec as f64 + time.nsec / NANOSECONDS_PER_SECOND as f64
}

/**
 * Calculates an optimal time window size for finding messages based on topic statistics.
 * Returns the window size in milliseconds.
 */
pub fn calculate_optimal_window_ms(params: {
  num_messages: u64;
  first_message_time: Time;
  last_message_time: Time;
  target_messages_in_window: Option<u64>;
}) -> u64 {
  let {
    num_messages,
    first_message_time,
    last_message_time,
    target_messages_in_window = Some(DEFAULT_TARGET_MESSAGES),
  } = params;

  let total_duration_sec = time_to_sec(last_message_time) - time_to_sec(first_message_time);

  if total_duration_sec <= 0 || num_messages <= 0 {
    return DEFAULT_WINDOW_MS;
  }

  let messages_per_second = num_messages as f64 / total_duration_sec;

  if messages_per_second == 0.0 {
    return MAX_WINDOW_MS;
  }

  let estimated_window_sec = target_messages_in_window / messages_per_second;

  // Clamp between min and max window sizes
  let window_ms = cmp::max(MIN_WINDOW_MS, cmp::min(MAX_WINDOW_MS, estimated_window_sec * 1000.0));

  return window_ms as u64;
}

/**
 * Creates a sequence of progressively larger time windows for adaptive search.
 */
pub fn create_window_sizes(
  initial_window_ms: u64,
  max_windows: u64 = DEFAULT_MAX_WINDOWS,
) -> Vec<u64> {
  let mut windows = vec![initial_window_ms];

  for i in 1..max_windows {
    let next_window = initial_window_ms * (WINDOW_GROWTH_FACTOR as f64).pow(i as f64);
    windows.push(cmp::min(next_window, WINDOW_CAP_MS));
  }

  windows
}

/**
 * Checks if a time window would reach before a boundary time.
 */
pub fn would_reach_boundary(
  current_time: Time,
  window_ms: u64,
  boundary_time: Option<Time>,
) -> bool {
  if let Some(boundary_time) = boundary_time {
    let window_start = subtract_ms(current_time, window_ms);
    return compare(window_start, boundary_time) <= 0;
  }
  false
}
```