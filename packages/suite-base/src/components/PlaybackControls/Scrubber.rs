```rust
use serde::{Deserialize, Serialize};
use std::ops::{Sub, Add, Div};

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    seconds: f64,
}

impl Time {
    pub fn from_secs(seconds: f64) -> Self {
        Self { seconds }
    }

    pub fn to_secs(self) -> f64 {
        self.seconds
    }
}

impl Sub<Time> for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Self::Output {
        Time {
            seconds: self.seconds - other.seconds,
        }
    }
}

impl Add<Time> for Time {
    type Output = Time;

    fn add(self, other: Time) -> Self::Output {
        Time {
            seconds: self.seconds + other.seconds,
        }
    }
}

impl Div<f64> for Time {
    type Output = Time;

    fn div(self, divisor: f64) -> Self::Output {
        Time {
            seconds: self.seconds / divisor,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerPresence {
    value: String,
}

impl PlayerPresence {
    pub const INITIALIZING: &str = "INITIALIZING";
    pub const BUFFERING: &str = "BUFFERING";
}

type HoverInfo = (Time, f64, f64);

fn subtract_times(a: Time, b: Time) -> Time {
    Time::from_secs(a.seconds - b.seconds)
}

fn add_times(a: Time, b: Time) -> Time {
    Time::from_secs(a.seconds + b.seconds)
}

fn to_sec(t: Time) -> f64 {
    t.seconds
}

fn from_sec(seconds: f64) -> Time {
    Time::from_secs(seconds)
}

fn main() {
    // Your implementation here
}
```