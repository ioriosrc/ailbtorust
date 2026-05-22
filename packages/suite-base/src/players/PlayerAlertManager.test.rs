```rust
use std::collections::HashMap;
use console_log;

struct PlayerAlertManager {
    alerts: HashMap<String, String>,
}

impl PlayerAlertManager {
    fn new() -> Self {
        Self {
            alerts: HashMap::new(),
        }
    }

    fn add_alert(&mut self, key: &str, alert: String) {
        self.alerts.insert(key.to_string(), alert);
        match key.parse::<i32>() {
            Ok(id) => console_log::warn!("Added alert {}", id),
            Err(_) => console_log::error!("Invalid key format for alert {}", key),
        }
    }

    fn remove_alert(&mut self, key: &str) -> bool {
        if self.alerts.contains_key(key) {
            let _ = self.alerts.remove(key);
            match key.parse::<i32>() {
                Ok(id) => console_log::warn!("Removed alert {}", id),
                Err(_) => console_log::error!("Invalid key format for removed alert {}", key),
            }
            true
        } else {
            false
        }
    }

    fn remove_alerts(&mut self, predicate: impl Fn(&str, &String) -> bool) -> bool {
        let mut alerts_to_remove = Vec::new();
        for (key, alert) in self.alerts.iter() {
            if predicate(key, alert) {
                alerts_to_remove.push((key.to_string(), alert.clone()));
            }
        }

        for (key, alert) in alerts_to_remove {
            self.remove_alert(&key);
        }

        true
    }

    fn alerts(&self) -> Vec<String> {
        self.alerts.values().cloned().collect()
    }
}
```