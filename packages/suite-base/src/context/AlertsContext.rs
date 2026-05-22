```rust
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerAlert {
    // Define the fields of PlayerAlert
}

#[derive(Clone)]
pub struct SessionAlert {
    tag: String,
    alert_data: PlayerAlert,
}

impl SessionAlert {
    pub fn new(tag: String, alert_data: PlayerAlert) -> Self {
        SessionAlert { tag, alert_data }
    }

    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    pub fn set_alert_data(&mut self, alert_data: PlayerAlert) {
        self.alert_data = alert_data;
    }
}

#[derive(Clone)]
pub struct AlertsContextStore {
    alerts: Mutex<Vec<SessionAlert>>,
    actions: Actions,
}

impl AlertsContextStore {
    pub fn new() -> Self {
        AlertsContextStore {
            alerts: Mutex::new(vec![]),
            actions: Actions { ..Default::default() },
        }
    }

    pub fn clear_alert(&self, tag: &str) {
        let mut alerts = self.alerts.lock().unwrap();
        alerts.retain(|alert| alert.get_tag() != tag);
    }

    pub fn clear_alerts(&self) {
        let mut alerts = self.alerts.lock().unwrap();
        alerts.clear();
    }

    pub fn set_alert(&mut self, tag: &str, alert_data: SessionAlert) {
        let mut alerts = self.alerts.lock().unwrap();
        *alerts.iter_mut()
            .find(|alert| alert.get_tag() == tag)
            .expect("Alert not found") = alert_data;
    }
}

#[derive(Clone)]
pub struct Actions {
    clear_alert: fn(&self, tag: &str),
    clear_alerts: fn(&self),
    set_alert: fn(&self, tag: &str, alert_data: SessionAlert),
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            clear_alert: |this, tag| this.clear_alert(tag),
            clear_alerts: |this| this.clear_alerts(),
            set_alert: |this, tag, alert_data| this.set_alert(tag, alert_data),
        }
    }

    pub fn clear_alert(&self, tag: &str) {
        self.clear_alert(tag);
    }

    pub fn clear_alerts(&self) {
        self.clear_alerts();
    }

    pub fn set_alert(&self, tag: &str, alert_data: SessionAlert) {
        self.set_alert(tag, alert_data);
    }
}

pub type AlertsContext = Arc<Mutex<AlertsContextStore>>;

pub fn useAlertsStore<T>(selector: impl FnOnce(&AlertsContextStore) -> T) -> T {
    let context = Arc::clone(&AlertsContext);
    use_store(context, selector)
}

fn select_actions(store: &AlertsContextStore) -> Actions {
    Actions {
        clear_alert: |this| this.clear_alert(),
        clear_alerts: |this| this.clear_alerts(),
        set_alert: |this, tag, alert_data| this.set_alert(tag, alert_data),
    }
}
```