```rust
use std::collections::HashMap;

pub struct LocalStorageAppConfiguration {
    defaults: HashMap<String, serde_json::Value>,
    change_handlers: HashMap<String, Vec<fn(serde_json::Value)>>,
}

impl LocalStorageAppConfiguration {
    pub fn new(defaults: Option<HashMap<String, serde_json::Value>>) -> Self {
        let mut defaults = HashMap::new();
        if let Some(ref def) = defaults {
            for (k, v) in def {
                defaults.insert(k.to_string(), serde_json::from_str(&v).unwrap());
            }
        }
        LocalStorageAppConfiguration {
            defaults,
            change_handlers: HashMap::new(),
        }
    }

    pub fn get<V: serde::Serialize + serde::Deserialize>(&self, key: &str) -> V {
        let value = match localStorage.get(key) {
            Some(val) => serde_json::from_str(&val).unwrap(),
            None => self.defaults.get(key).map(|v| serde_json::from_value(v.clone()).unwrap()),
        };
        serde_json::to_value(value).unwrap()
    }

    pub async fn set<V: serde::Serialize + serde::Deserialize>(&mut self, key: &str, value: V) -> Result<(), serde_json::Error> {
        if let Ok(value_json) = serde_json::to_string(&value) {
            localStorage.setItem(key, value_json);
        }
        let listeners = self.change_handlers.get_mut(key).unwrap();
        for listener in listeners.iter() {
            listener(value.clone());
        }
        Ok(())
    }

    pub fn add_change_listener<V: serde::Serialize + serde::Deserialize>(
        &mut self,
        key: &str,
        cb: fn(V),
    ) -> Result<(), serde_json::Error> {
        let mut handlers = self.change_handlers.entry(key.to_string()).or_default();
        if !handlers.contains(&cb) {
            handlers.push(cb);
        }
        Ok(())
    }

    pub fn remove_change_listener<V: serde::Serialize + serde::Deserialize>(
        &mut self,
        key: &str,
        cb: fn(V),
    ) -> Result<(), serde_json::Error> {
        let mut handlers = self.change_handlers.entry(key.to_string()).or_default();
        if handlers.contains(&cb) {
            handlers.remove(&cb);
        }
        Ok(())
    }
}
```