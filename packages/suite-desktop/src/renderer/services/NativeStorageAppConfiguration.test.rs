```rust
use std::collections::{HashMap, VecDeque};
use futures::{future, FutureExt};

struct NativeStorageAppConfiguration {
    storage: HashMap<String, serde_json::Value>,
    listeners: HashMap<String, VecDeque<dyn Fn(&str) -> ()>>,
}

impl NativeStorageAppConfiguration {
    async fn initialize(ctx: &mut Self, defaults: Option<HashMap<String, serde_json::Value>>) {
        if let Some(defaults) = defaults {
            for (key, value) in defaults {
                self.storage.insert(key.to_string(), serde_json::to_value(value).unwrap());
            }
        }

        ctx.get();
    }

    async fn get(&self) -> serde_json::Value {
        let value = self.storage.get("abc").cloned().unwrap_or(serde_json::Value::Null);
        println!("Value: {:?}", &value);
        value
    }

    async fn put(&mut self, key: String, value: serde_json::Value) {
        self.storage.insert(key, value.clone());
        for listener in self.listeners.get(&key).cloned().unwrap_or_default() {
            listener(&key);
        }
    }

    async fn delete(&mut self, key: String) {
        if let Some(value) = self.storage.remove(&key) {
            println!("Deleted value: {:?}", &value);
        } else {
            println!("Value not found");
        }
    }

    fn add_change_listener(&mut self, key: String, listener: Box<dyn Fn(&str) -> ()>) {
        if !self.listeners.contains_key(&key) {
            self.listeners.insert(key, VecDeque::new());
        }
        self.listeners.get_mut(&key).unwrap().push_back(listener);
    }

    fn remove_change_listener(&mut self, key: String, listener: Box<dyn Fn(&str) -> ()>) {
        if let Some(mut listeners) = self.listeners.remove(&key) {
            listeners.retain(|&l| l != listener);
        }
    }
}

#[tokio::test]
async fn test_native_storage_app_configuration() {
    async fn raise(name: &str) {
        panic!("Unexpected call to {}", name);
    }

    let mut ctx = NativeStorageAppConfiguration {
        storage: HashMap::new(),
        listeners: HashMap::new(),
    };

    ctx.get().await;
    ctx.put("abc", serde_json::Value::Number(123.0)).await;

    ctx.add_change_listener("abc", Box::new(|key| println!("Listener for key {}: {:?}", key, &ctx.storage[key])));
    ctx.set("abc", serde_json::Value::Number(456.0)).await;
}
```