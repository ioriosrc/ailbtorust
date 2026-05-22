```rust
use std::collections::{HashMap, HashSet};
use std::ops::IndexMut;

type MessageEvent = Box<dyn std::any::Any>;
type DiagnosticStatusArrayMsg = Box<dyn std::any::Any>;
type UseAvailableDiagnosticResult = HashMap<String, HashSet<String>>;

fn add_messages(
  previous_available_diagnostics: &mut HashMap<String, HashSet<String>>,
  messages: &[MessageEvent],
): bool {
  // If we detect new hardware ids or names we need to create a new instance of available diagnostics
  // so downstream consumers know it changed by observing the object reference changing
  let mut modified = false;

  for message in messages.iter() as &[MessageEvent<DiagnosticStatusArrayMsg>] {
    if let Some(message) = message.downcast_ref::<DiagnosticStatusArrayMsg>() {
      let status_array: &Vec<DiagnosticStatusArrayMsg::Target> = message.as_ref();
      if status_array.is_empty() {
        continue;
      }

      for status in status_array.iter() {
        let hardware_id = status.hardware_id().unwrap_or_default();
        let name = status.name();

        if let Some(name_set) = previous_available_diagnostics.get_mut(hardware_id) {
          if !name_set.contains(name.unwrap_or_default()) && name.is_some() {
            modified = true;
            name_set.insert(name.unwrap());
          }
        } else {
          modified = true;
          previous_available_diagnostics.insert(
            hardware_id.to_string(),
            HashSet::from([name.unwrap_or_default()]),
          );
        }
      }
    }
  }

  modified
}

fn empty_map() -> HashMap<String, HashSet<String>> {
  HashMap::new()
}

pub fn use_available_diagnostics(topic: Option<&str>) -> UseAvailableDiagnosticResult {
  let topics = useMemo(
    || {
      if let Some(topic) = topic {
        vec![topic]
      } else {
        Vec::new()
      }
    },
    [topic],
  );

  use_message_reducer(UseAvailableDiagnosticResult::default(), {
    topics,
    restore: empty_map,
    add_messages,
  });
}
```