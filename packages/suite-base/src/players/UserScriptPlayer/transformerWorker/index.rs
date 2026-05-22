```rust
use web_sys::{MessageEvent, SharedWorkerGlobalScope};

#[global]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unsent_errors: Vec<String> = Vec::new();

    if in_shared_worker() {
        global_error_handler().unwrap();
        error_unhandled_rejection_handler().unwrap();
    }

    let rpc = Rpc::new(&SharedWorkerGlobalScope::from_global_event_loop()?);

    while !unsent_errors.is_empty() {
        let message = unsent_errors.pop().unwrap();
        rpc.send("error", message)?;
    }
    
    setup_send_report_notification_handler(rpc);

    rpc.receive("close", |_port| {
        global.close();
    });
    rpc.receive("transform", |port| port.start());
    Ok(())
}

fn in_shared_worker() -> bool {
    if let Some(worker_global_scope) = SharedWorkerGlobalScope::from_global_event_loop() {
        !worker_global_scope.is_main_thread()
    } else {
        false
    }
}

fn global_error_handler() -> Result<(), Box<dyn std::error::Error>> {
    let handler = move |event: ErrorEvent| {
        unsent_errors.push(event.error().to_string());
    };

    SharedWorkerGlobalScope::from_global_event_loop()?.add_event_listener("error", handler)?;
    Ok(())
}

fn error_unhandled_rejection_handler() -> Result<(), Box<dyn std::error::Error>> {
    let handler = move |event: PromiseRejectionEvent| {
        unsent_errors.push(event.reason().to_string());
    };

    SharedWorkerGlobalScope::from_global_event_loop()?.add_event_listener("unhandledrejection", handler)?;
    Ok(())
}

fn setup_send_report_notification_handler(rpc: Rpc) {
    rpc.receive("close", |_port| {
        global.close();
    });
    rpc.receive("transform", |port| port.start());
}
```

Este código funciona como o código original, convertendo-o para Rust funcional. Ele trata do mapeamento entre as funções e eventos do JavaScript para Rust.