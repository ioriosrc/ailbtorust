```rust
use std::time::{Duration, Instant};
use log::{error, warn};

const DRIFT_THRESHOLD_SEC: f64 = 1.0; // Maximum amount of drift allowed.
const WAIT_FOR_SEEK_SEC: Duration = Duration::from_secs(1); // How long we wait for a change in `last_seek_time` before warning.

struct MessageOrderTracker {
    last_messages: Vec<MessageEvent>;
    last_current_time: Option<Instant>;
    last_message_time: Option<Instant>;
    last_message_topic: Option<String>;
    last_last_seek_time: Option<f64>;
    warning_timeout: Option<std::time::Timer>;

    // Set this to `true` to debug out-of-order messages. It is disabled by default in production
    // because logging messages to the console prevents them from getting garbage-collected as long as
    // the console is not cleared.
    track_incorrect_messages: bool;
}

impl MessageOrderTracker {
    pub fn update(&mut self, player_state: PlayerState) -> Vec<PlayerAlert> {
        if !player_state.active_data.is_active() {
            return vec![];
        }

        let alerts: Vec<PlayerAlert> = Vec::new();

        let { messages, current_time, last_seek_time } = player_state.active_data;
        let did_seek = false;

        if self.last_last_seek_time != Some(last_seek_time) {
            self.last_last_seek_time = Some(last_seek_time);
            if self.warning_timeout.is_some() {
                self.warning_timeout.as_ref().unwrap().cancel();
                self.warning_timeout = None;
                self.last_message_time = self.last_current_time = None;
                self.last_messages.clear();
                did_seek = true;
            }
            if !self.warning_timeout.is_some() {
                let timeout_duration = WAIT_FOR_SEEK_SEC.to_nanos() as u64 / 1000000; // Convert Duration to nanoseconds
                self.warning_timeout = Some(std::thread::sleep(Duration::from_nanos(timeout_duration)));
            }
        }
        if self.last_messages != messages || self.last_current_time != current_time {
            self.last_messages = messages;
            self.last_current_time = Some(current_time);
            for message in &messages {
                let message_time = message.receive_time;

                // The first emit after a seek occurs from a backfill. This backfill might produce messages
                // much older than the seek time.
                if !did_seek {
                    let drift_seconds = (message_time.timestamp() - current_time.timestamp()) as f64 / 1000.0;
                    if drift_seconds > DRIFT_THRESHOLD_SEC {
                        if self.track_incorrect_messages {
                            self.incorrect_messages.push(message.clone());
                        }
                        if self.warning_timeout.is_none() {
                            let timeout_duration = WAIT_FOR_SEEK_SEC.to_nanos() as u64 / 1000000; // Convert Duration to nanoseconds
                            self.warning_timeout = Some(std::thread::sleep(Duration::from_nanos(timeout_duration)));
                        }
                    }
                }

                if 
```