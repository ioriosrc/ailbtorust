```rust
use std::collections::HashMap;

pub struct Alert {
    tag: String,
    // Add other fields as needed
}

#[derive(Default)]
pub struct AlertsContext {
    alerts: Vec<Alert>,
}

impl AlertsContext {
    pub fn clear_alert(&mut self, tag: &str) {
        self.alerts.retain(|alert| alert.tag != tag);
    }

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }

    pub fn set_alert(&mut self, tag: &str, alert: Alert) {
        if let Some(existing) = self.alerts.iter().find(|alert| alert.tag == tag) {
            if existing != &alert {
                *existing = alert;
            }
        } else {
            self.alerts.push(alert);
        }
    }
}

pub fn create_alerts_store() -> AlertsContext {
    AlertsContext::default()
}
```