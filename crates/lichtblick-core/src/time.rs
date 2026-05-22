// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};

/// Represents a point in time with nanosecond precision, matching ROS Time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Time {
    /// Seconds since epoch
    pub sec: u32,
    /// Nanoseconds within the current second (0..999_999_999)
    pub nsec: u32,
}

impl Time {
    pub const ZERO: Time = Time { sec: 0, nsec: 0 };

    pub fn new(sec: u32, nsec: u32) -> Self {
        Self { sec, nsec }
    }

    /// Create Time from nanoseconds since epoch.
    pub fn from_nanos(nanos: u64) -> Self {
        Self {
            sec: (nanos / 1_000_000_000) as u32,
            nsec: (nanos % 1_000_000_000) as u32,
        }
    }

    /// Convert to nanoseconds since epoch.
    pub fn to_nanos(self) -> u64 {
        (self.sec as u64) * 1_000_000_000 + (self.nsec as u64)
    }

    /// Convert to seconds as f64.
    pub fn to_secs_f64(self) -> f64 {
        self.sec as f64 + self.nsec as f64 / 1_000_000_000.0
    }

    /// Create from seconds as f64.
    pub fn from_secs_f64(secs: f64) -> Self {
        let sec = secs.floor() as u32;
        let nsec = ((secs - sec as f64) * 1_000_000_000.0) as u32;
        Self { sec, nsec }
    }

    /// Returns true if this time is zero.
    pub fn is_zero(self) -> bool {
        self.sec == 0 && self.nsec == 0
    }

    /// Duration between two times in nanoseconds.
    pub fn duration_nanos(from: Time, to: Time) -> i64 {
        to.to_nanos() as i64 - from.to_nanos() as i64
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sec
            .cmp(&other.sec)
            .then(self.nsec.cmp(&other.nsec))
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Time {
    type Output = Time;
    fn add(self, rhs: Self) -> Self::Output {
        let total_nsec = self.nsec + rhs.nsec;
        Time {
            sec: self.sec + rhs.sec + total_nsec / 1_000_000_000,
            nsec: total_nsec % 1_000_000_000,
        }
    }
}

impl Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Self) -> Self::Output {
        let total_nanos = self.to_nanos().saturating_sub(rhs.to_nanos());
        Time::from_nanos(total_nanos)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:09}", self.sec, self.nsec)
    }
}

/// Represents a duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Duration {
    pub sec: i32,
    pub nsec: i32,
}

impl Duration {
    pub const ZERO: Duration = Duration { sec: 0, nsec: 0 };

    pub fn new(sec: i32, nsec: i32) -> Self {
        Self { sec, nsec }
    }

    pub fn from_millis(millis: i64) -> Self {
        Self {
            sec: (millis / 1000) as i32,
            nsec: ((millis % 1000) * 1_000_000) as i32,
        }
    }

    pub fn to_nanos(self) -> i64 {
        (self.sec as i64) * 1_000_000_000 + (self.nsec as i64)
    }

    pub fn to_secs_f64(self) -> f64 {
        self.sec as f64 + self.nsec as f64 / 1_000_000_000.0
    }
}
