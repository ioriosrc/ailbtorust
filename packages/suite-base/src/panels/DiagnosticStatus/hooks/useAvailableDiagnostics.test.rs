```rust
use std::collections::HashMap;

fn add_messages(previous_available_diagnostics: HashMap<String, HashSet<String>>, messages: Vec<MessageEvent>) -> HashMap<String, HashSet<String>> {
    for message in &messages {
        if let MessageEvent::MessageEvent<DiagnosticStatusArrayMsg>(msg) = message {
            if !previous_available_diagnostics.contains_key(&msg.hardware_id) {
                previous_available_diagnostics.insert(msg.hardware_id.clone(), HashSet::new());
            }
            previous_available_diagnostics.get_mut(&msg.hardware_id).unwrap().insert(msg.name.clone());
        }
    }
    previous_available_diagnostics
}

fn use_available_diagnostics(topic: String, messages: Vec<MessageEvent>) -> HashMap<String, HashSet<String>> {
    let mut result = HashMap::new();
    for message in &messages {
        if let MessageEvent::MessageEvent<DiagnosticStatusArrayMsg>(msg) = message {
            if !result.contains_key(&msg.hardware_id) {
                result.insert(msg.hardware_id.clone(), HashSet::new());
            }
            result.get_mut(&msg.hardware_id).unwrap().insert(msg.name.clone());
        }
    }
    result
}
```