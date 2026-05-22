```rust
use std::cmp::{self, Ordering};
use wasm_bindgen::JsValue;

pub struct RepeatAdapter {
    props:RepeatAdapterProps,
}

impl RepeatAdapter {
    pub fn new(props: RepeatAdapterProps) -> Self {
        RepeatAdapter { props }
    }

    pub fn play(&mut self) {
        // Implementation to play the music
        println!("Playing");
    }

    pub fn seek(&mut self, time: Time) {
        // Implementation to seek to a specific time in the music
        println!("Seeking to {}", time);
    }

    pub fn update(&mut self, active_data: Option<ActiveData>) {
        if let Some(data) = &active_data {
            if data.current_time > data.end_time && self.props.repeat_enabled {
                self.seek(data.start_time);
                self.play();
            }
        }
    }
}

struct ActiveData {
    current_time: Time,
    end_time: Time,
    start_time: Time,
}

type Time = f64; // Assuming a basic type for time

fn compare(a: Time, b: Time) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    if a < b {
        return Ordering::Less;
    }
    Ordering::Greater
}
```