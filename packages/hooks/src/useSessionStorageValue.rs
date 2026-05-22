```rust
use std::sync::{RwLock};
use std::thread;

type SessionStorageValue = (
    RwLock<Option<String>>,
    fn(&mut Self, &str) -> Result<(), ()>,
);

/// This provides a convenience wrapper around sessionStorage and triggers
/// a react state change when values change.
///
/// @param key sessionStorage key to manage.
/// @returns [value, setValue] tuple for that key.
pub fn use_session_storage_value(key: &str) -> SessionStorageValue {
    let prefixed_key = format!("{}.{}", env!("DEV_WORKSPACE"), key);
    let value = RwLock::new(None);

    let set_value = move |new_val: Option<String>| -> Result<(), ()> {
        if new_val.is_some() {
            storage.set(&prefixed_key, &new_val)?;
        } else {
            storage.remove(&prefixed_key)?;
        }
        Ok(())
    };

    let change_listener = move |event| {
        if event.key == prefixed_key {
            *value.write().unwrap() = Some(event.new_val.unwrap());
        }
    };

    thread::spawn(move || {
        // Listen to sessionStorage events
        while let Ok(event) = storage.listen(&prefixed_key) {
            change_listener(event);
        }
    });

    (value, set_value)
}
```