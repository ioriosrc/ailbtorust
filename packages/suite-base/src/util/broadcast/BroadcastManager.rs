```rust
use std::sync::{Arc, RwLock};
use std::thread;

const BROADCAST_CHANNEL_NAME: &str = "bmw_broadcast_channel";

#[derive(Clone)]
pub struct BroadcastMessageEvent {
    // Define the structure of your message here
}

type ChannelListeners = Arc<RwLock<Vec<Box<dyn Fn(BroadcastMessageEvent) + Send>>>>;

struct BroadcastManager {
    channel: crossbeam_channel::Sender<BroadcastMessageEvent>,
    listeners: ChannelListeners,
}

impl Default for BroadcastManager {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let listeners = Arc::new(RwLock::new(Vec::new()));
        let manager = BroadcastManager { channel, listeners };
        thread::spawn(move || loop {
            if let Ok(msg) = receiver.recv() {
                for listener in &*listeners.read() {
                    listener(msg);
                }
            }
        });
        manager
    }
}

impl.BroadcastManager {
    pub fn post_message(&self, message: BroadcastMessageEvent) {
        self.channel.send(message).unwrap();
    }

    pub fn add_listener(&self, listener: impl Fn(BroadcastMessageEvent) + Send + 'static) {
        let cloned_listeners = Arc::clone(&self.listeners);
        self.listeners.write().unwrap().push(Box::new(move |msg| {
            if msg.channel_name == BROADCAST_CHANNEL_NAME {
                listener(msg.clone());
            }
        }));
    }

    pub fn remove_listener(&self, listener: impl Fn(BroadcastMessageEvent) + Send + 'static) {
        let cloned_listeners = Arc::clone(&self.listeners);
        self.listeners.write().unwrap().retain(|&x| x != Box::new(move |msg| {
            if msg.channel_name == BROADCAST_CHANNEL_NAME {
                listener(msg.clone());
            }
        }));
    }

    pub fn close(&mut self) {
        let mut listeners = Arc::clone(&self.listeners);
        for _ in 0..listeners.read().unwrap().len() {
            let cloned_listeners = Arc::clone(&listeners);
            thread::spawn(move || loop {
                if let Ok(msg) = cloned_listeners.write().unwrap().pop() {
                    msg(BroadcastMessageEvent { channel_name: BROADCAST_CHANNEL_NAME.to_string(), data: None });
                }
            });
        }
        self.channel.close();
    }

    pub fn set_should_sync(&self, should_sync: bool) {
        if should_sync {
            let mut listeners = Arc::clone(&self.listeners);
            for _ in 0..listeners.read().unwrap().len() {
                let cloned_listeners = Arc::clone(&listeners);
                thread::spawn(move || loop {
                    if let Ok(msg) = cloned_listeners.write().unwrap().pop() {
                        msg(BroadcastMessageEvent { channel_name: BROADCAST_CHANNEL_NAME.to_string(), data: None });
                    }
                });
            }
        } else {
            self.close();
        }
    }

    pub fn get_instance() -> Arc<Self> {
        if BroadcastManager::should_sync {
            let (sender, receiver) = crossbeam_channel::unbounded();
            let listeners = Arc::new(RwLock::new(Vec::new()));
            let manager = BroadcastManager { channel: sender, listeners };
            thread::spawn(move || loop {
                if let Ok(msg) = receiver.recv() {
                    for listener in &*listeners.read() {
                        listener(msg);
                    }
                }
            });
            Arc::new(manager)
        } else {
            Arc::new(BroadcastManager::default())
        }
    }
}
```