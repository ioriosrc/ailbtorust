```rust
use std::sync::{Arc, Mutex};
use crossbeam_channel as cbc;

#[derive(Debug)]
pub struct EventListenerHandler {
    channel: Arc<Mutex<Option<cbc::Sender<()>>>>,
}

impl EventListenerHandler {
    pub fn new(channel: Arc<Mutex<Option<cbc::Sender<()>>>>) -> Self {
        Self { channel }
    }

    pub fn add_listener(&self, event_name: &str, callback: Box<dyn Fn() + Send>) {
        let mut sender = self.channel.lock().unwrap();
        if let Some(sender) = *sender {
            sender.send(()).unwrap();
        } else {
            // Log error or handle situation where the channel is not available
        }
    }

    pub fn remove_listener(&self, event_name: &str, callback: Box<dyn Fn() + Send>) {
        let mut sender = self.channel.lock().unwrap();
        if let Some(sender) = *sender {
            sender.send(()).unwrap();
        } else {
            // Log error or handle situation where the channel is not available
        }
    }
}
```