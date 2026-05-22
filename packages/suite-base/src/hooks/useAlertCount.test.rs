```rust
use mockall::mock;
use std::cell::{RefCell, RefMut};

// Define the necessary types for testing
type PlayerAlert = Vec<(String, &str)>;

#[derive(Default)]
struct AlertsStore {
    player_alerts: RefCell<Vec<PlayerAlert>>,
    session_alerts: RefCell<Vec<PlayerAlert>>,
}

impl AlertsStore {
    fn get_player_alerts(&self) -> RefMut<Vec<PlayerAlert>> {
        self.player_alerts.borrow_mut()
    }

    fn get_session_alerts(&self) -> RefMut<Vec<PlayerAlert>> {
        self.session_alerts.borrow_mut()
    }
}

#[test]
fn test_use_alert_count() {
    // Arrange
    mock! { AlertsStore => pub fn get_player_alerts() -> _ }
    mock! { AlertsStore => pub fn get_session_alerts() -> _ }

    let alerts_store = RefCell::new(AlertsStore::default());
    AlertsStore::get_player_alerts().borrow_mut().push(vec![("Hello", "World")]);
    AlertsStore::get_session_alerts().borrow_mut().push(vec![("Info", "Alert")]);

    // Act
    use super::use_alert_count;
    let hook = Box::new(use_alert_count());
    let alerts_store_ref = RefMut::from(&alerts_store);

    // Assert
    assert_eq!(hook.player_alerts(), &["Hello".to_string(), "World".to_string()]);
    assert_eq!(hook.session_alerts(), &["Info".to_string(), "Alert".to_string()]);
    assert_eq!(hook.alert_count(), 2);
}
```