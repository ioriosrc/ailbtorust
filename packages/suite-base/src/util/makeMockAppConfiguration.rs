```rust
use std::collections::{HashMap, HashSet};

pub struct AppConfiguration {
    map: HashMap<String, String>,
    listeners: HashMap<String, HashSet<fn(String) -> ()>>,
}

impl AppConfiguration {
    pub fn new() -> Self {
        AppConfiguration {
            map: HashMap::new(),
            listeners: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub async fn set(&mut self, key: &str, value: String) {
        self.map.insert(key.to_string(), value);
        self.notify_listeners(key, value.clone());
    }

    pub fn add_change_listener(&mut self, key: &str, cb: fn(String) -> ()) {
        let listeners = self.listeners.entry(key.to_string()).or_insert_with(HashSet::new);
        listeners.insert(cb);
    }

    pub fn remove_change_listener(&mut self, key: &str, cb: fn(String) -> ()) {
        if let Some(listeners) = self.listeners.get_mut(key) {
            listeners.remove(&cb);
        }
    }

    fn notify_listeners(&self, key: &str, value: String) {
        if let Some(listeners) = self.listeners.get(key) {
            for listener in listeners.iter() {
                listener(value.clone());
            }
        }
    }
}
```