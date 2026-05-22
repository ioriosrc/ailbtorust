```rust
use std::sync::{Arc, Mutex};
use parking_lot::MutexGuard;

/// Mock implementation of `send_notification` for testing purposes.
///
/// # Examples
/// ```
/// use lightblick_suite_base::util::send_notification::mock_send_notification;
/// mock_send_notification();
/// ```
fn mock_send_notification() {
    let notifier = Arc::new(Mutex::new(vec![]));

    std::thread::spawn(move || {
        let mut notifiers: MutexGuard<Vec<&mut dyn Fn(&str, &DetailsType, &NotificationType, &NotificationSeverity)>> =
            notifier.lock().unwrap();

        for handler in notifiers.iter() {
            handler("test", &DetailsType::default(), &NotificationType::default(), &NotificationSeverity::INFO);
        }
    });
}

/// Set the current notification handler.
///
/// # Examples
/// ```
/// use lightblick_suite_base::util::send_notification::mock_set_notification_handler;
/// mock_set_notification_handler(|msg, details, type_, severity| {
///     println!("Notification: {} - {}", msg, severity);
/// });
/// ```
fn mock_set_notification_handler(handler: Option<Box<dyn Fn(&str, &DetailsType, &NotificationType, &NotificationSeverity)>>) {
    let notifier = Arc::new(Mutex::new(vec![]));

    if let Some(handler) = handler {
        notifiers.lock().unwrap().push(Box::from(handler));
    }
}

/// Setup the mock notification system.
///
/// # Examples
/// ```
/// use lightblick_suite_base::util::send_notification::{mock_send_notification, mock_set_notification_handler};
/// mock_send_notification();
/// mock_set_notification_handler(|msg, details, type_, severity| {
///     println!("Notification: {} - {}", msg, severity);
/// });
/// ```
fn setup_mock_notification() {
    let notifier = Arc::new(Mutex::new(vec![]));

    std::thread::spawn(move || {
        let mut notifiers: MutexGuard<Vec<&mut dyn Fn(&str, &DetailsType, &NotificationType, &NotificationSeverity)>> =
            notifier.lock().unwrap();

        for handler in notifiers.iter() {
            handler("test", &DetailsType::default(), &NotificationType::default(), &NotificationSeverity::INFO);
        }
    });
}
```