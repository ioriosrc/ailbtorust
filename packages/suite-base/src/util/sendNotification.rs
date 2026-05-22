```rust
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum NotificationType {
    App,
    User,
}

#[derive(Clone)]
pub enum DetailsType {
    String(String),
    Error(Box<dyn std::error::Error>),
    ReactNode(Box<dyn std::fmt::Display>),
}

#[derive(Debug, Clone)]
pub struct NotificationSeverity {
    level: &'static str,
}

impl NotificationSeverity {
    pub fn new(level: &'static str) -> Self {
        Self { level }
    }
}

pub type NotificationMessage = (String, DetailsType, NotificationType, NotificationSeverity);

struct InWebWorker(bool);
const IN_WEB_WORKER: Arc<Mutex<InWebWorker>> = Arc::new(Mutex::new(InWebWorker(false)));

#[derive(Default)]
struct NotificationHandler {
    handler: fn(String, DetailsType, NotificationType, NotificationSeverity),
}

impl NotificationHandler {
    pub fn new(handler: fn(String, DetailsType, NotificationType, NotificationSeverity)) -> Self {
        Self { handler }
    }

    pub fn notify(&self, message: String, details: DetailsType, type_: NotificationType, severity: NotificationSeverity) {
        (self.handler)(message, details, type_, severity);
    }
}

pub struct NotificationManager {
    handler: Arc<Mutex<NotificationHandler>>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            handler: Arc::new(Mutex::new(NotificationHandler::default())),
        }
    }

    pub fn set_handler(&self, handler: impl Fn(String, DetailsType, NotificationType, NotificationSeverity)) {
        let mut handler = self.handler.lock().unwrap();
        handler.handler = handler;
    }

    pub fn unset_handler(&self) {
        let mut handler = self.handler.lock().unwrap();
        handler.handler = NotificationHandler::default().handler;
    }
}

fn in_web_worker() -> bool {
    IN_WEB_WORKER.lock().unwrap().0
}

pub fn send_notification(
    message: String,
    details: DetailsType,
    type_: NotificationType,
    severity: NotificationSeverity,
) {
    let manager = NotificationManager::new();
    if !in_web_worker() || *IN_WEB_WORKER.lock().unwrap().0 {
        if severity == NotificationSeverity::Warn || severity == NotificationSeverity::Error {
            report_error(new_app_error(details, message));
        }
    }

    let handler = manager.handler.lock().unwrap();
    handler.notify(message, details, type_, severity);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_notification() {
        // Test the send_notification function
    }

    #[test]
    fn test_notification_manager() {
        let manager = NotificationManager::new();
        manager.set_handler(|_| {});
        assert_eq!(manager.handler.lock().unwrap().handler, |message, details, type_, severity| {});
    }
}
```