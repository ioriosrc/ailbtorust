```rust
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
struct Time {
    seconds: u64,
}

impl From<&str> for Time {
    fn from(s: &str) -> Self {
        let parts = s.split('.').collect::<Vec<_>>();
        if parts.len() != 2 {
            panic!("Invalid time format");
        }
        let seconds_str = parts[0];
        let fraction_str = parts[1];

        let mut seconds = 0;
        for (i, digit) in seconds_str.chars().enumerate() {
            seconds += digit.to_digit(10).unwrap() * 10u64.pow(i as u32);
        }

        let fraction = fraction_str.parse::<f64>().unwrap();
        seconds += fraction;

        Time { seconds }
    }
}

#[derive(Clone, Copy)]
struct AppTimeFormat {
    time_format: TimeDisplayMethod,
}

impl AppTimeFormat {
    fn set_time_format(&mut self, method: TimeDisplayMethod) {
        self.time_format = method;
    }

    fn get_current_time_display_method(&self) -> &TimeDisplayMethod {
        &self.time_format
    }
}

enum TimeDisplayMethod {
    TOD,
    SEC,
}

impl Default for AppTimeFormat {
    fn default() -> Self {
        Self {
            time_format: TimeDisplayMethod::TOD,
        }
    }
}

fn main() {
    let mut app_time_format = AppTimeFormat::default();

    // Example usage
    println!("{:?}", app_time_format.get_current_time_display_method());
    app_time_format.set_time_format(TimeDisplayMethod::SEC);
    println!("{:?}", app_time_format.get_current_time_display_method());

    // In a real-world scenario, you would interact with this struct using state and actions
}
```