```rust
use serde_json::Value;

pub fn setup_receive_report_error_handler(rpc: &mut Rpc) {
    rpc.receive(
        "sendNotification",
        |details| {
            send_notification(details.message, details.details, details.type, details.severity);
        },
    );
}

pub fn setup_main_thread_rpc(rpc: &mut Rpc) {
    setup_receive_report_error_handler(rpc);
}
```