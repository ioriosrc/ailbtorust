```rust
use std::collections::HashMap;

pub struct PlayerAlertManager {
    alerts: HashMap<String, PlayerAlert>,
}

impl PlayerAlertManager {
    pub fn new() -> Self {
        PlayerAlertManager {
            alerts: HashMap::new(),
        }
    }

    pub fn alerts(&self) -> Vec<PlayerAlert> {
        self.alerts.values().cloned().collect()
    }

    pub fn add_alert(&mut self, id: String, alert: PlayerAlert) {
        println!("Player alert {:?}", alert);
        self.alerts.insert(id, alert);
        self.alerts.clear(); // Clear the alerts after adding to ensure the next call returns a fresh set
    }

    pub fn has_alert(&self, id: &str) -> bool {
        self.alerts.contains_key(id)
    }

    pub fn remove_alert(&mut self, id: &str) -> bool {
        if let Some(alert) = self.alerts.remove(id) {
            println!("Removed alert {:?}", alert);
            return true;
        }
        false
    }

    pub fn remove_alerts<F>(&mut self, predicate: F) -> bool
    where
        F: Fn(&str, &PlayerAlert) -> bool,
    {
        let mut changed = false;
        for (id, alert) in self.alerts.iter() {
            if predicate(id, alert) {
                if self.alerts.remove(id).is_some() {
                    changed = true;
                }
            }
        }
        if changed {
            self.alerts.clear(); // Clear the alerts after removing to ensure the next call returns a fresh set
        }
        changed
    }

    pub fn clear(&mut self) {
        self.alerts.clear();
    }
}
```