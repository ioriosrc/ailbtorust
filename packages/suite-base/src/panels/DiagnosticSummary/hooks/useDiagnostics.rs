```rust
use std::collections::{HashMap, VecDeque};

type DiagnosticInfo = ...; // Define the type of diagnostic info

#[derive(Default)]
struct DiagnosticsById {
    map: HashMap<String, DiagnosticInfo>,
}

pub fn computeDiagnosticInfo(status: &...) -> DiagnosticInfo { ... } // Implement the function to compute diagnostic info

fn addMessages(prev: &mut HashMap<String, DiagnosticsById>, message_events: VecDeque<MessageEvent<DiagnosticStatusArrayMsg>>): HashMap<String, DiagnosticsById> {
    let mut modified = false;
    let next = prev.clone();

    for event in message_events.iter() {
        let { header, status } = event.message;

        for &status in status.iter() {
            let info = compute_diagnostic_info(status, header.stamp);
            let diagnostics_by_name = next.get_mut(&status.hardware_id);

            if diagnostics_by_name.is_none() {
                next.insert(status.hardware_id.clone(), DiagnosticsById { map: HashMap::from([(status.name.clone(), info)]) });
                modified = true;
            } else {
                diagnostics_by_name.as_mut().unwrap().map.entry(status.name).or_insert(info);
                modified = true;
            }
        }
    }

    // We shallow-copy the buffer when it changes to help users know when to rerender.
    return if modified { next } else { prev.clone() };
}

fn main() {
    let mut diagnostics: HashMap<String, DiagnosticsById> = Default::default();

    // Example usage of addMessages function
    let message_events = VecDeque::from(vec![MessageEvent{message: DiagnosticStatusArrayMsg{header: ...}}]); // Initialize with some diagnostic messages
    add_messages(&mut diagnostics, message_events);
}
```