```rust
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

// Mock implementation of BroadcastChannel
pub struct MockBroadcastChannel {
    name: String,
    on_message: Option<fn(BroadcastMessageEvent)>,
    posted_messages: Vec<BroadcastMessageEvent>,
    is_closed: bool,
}

impl MockBroadcastChannel {
    pub fn new() -> Self {
        Self {
            name: BROADCAST_CHANNEL_NAME.to_string(),
            on_message: None,
            posted_messages: Vec::new(),
            is_closed: false,
        }
    }

    pub fn post_message(&mut self, message: BroadcastMessageEvent) {
        self.posted_messages.push(message);
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    // Helper to simulate receiving a message
    pub fn simulate_incoming_message(&self, message: BroadcastMessageEvent) {
        if let Some(on_message) = &self.on_message {
            on_message(message);
        }
    }
}

fn main() {
    let (broadcast_tx, broadcast_rx) = mpsc::channel();
    let mock_channel = MockBroadcastChannel::new();

    // Simulate the receiving of a message
    let receiver_thread = thread::spawn(move || {
        let received_msg: BroadcastMessageEvent = broadcast_rx.recv().unwrap();
        println!("Received message: {:?}", received_msg);
    });

    // Post a message to the mock channel
    mock_channel.post_message(BroadcastMessageEvent { data: "Hello".to_string() });
    mock_channel.simulate_incoming_message(BroadcastMessageEvent { data: "Hello".to_string() });

    // Wait for the receiver thread to finish
    receiver_thread.join().unwrap();
}
```