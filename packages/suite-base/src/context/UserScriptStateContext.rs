```rust
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
struct Diagnostic;

#[derive(Clone, Debug)]
struct UserScriptLog;

type UserScriptStates = std::collections::HashMap<String, (Vec<Diagnostic>, Vec<UserScriptLog>)>;

#[derive(Clone, Debug)]
struct UserScriptStore {
    state: UserScriptStates,
    ros_lib: String,
    types_lib: String,
}

pub fn create() -> Arc<Mutex<UserScriptStore>> {
    let store = Mutex::new(UserScriptStore {
        state: Default::default(),
        ros_lib: "ros_lib_dts".to_string(),
        types_lib: generate_empty_types_lib().0,
    });

    Arc::new(store)
}

pub struct UserScriptStateProvider {
    store: Arc<Mutex<UserScriptStore>>,
}

impl UserScriptStateProvider {
    pub fn new() -> Self {
        Self { store: create() }
    }

    pub fn get_store(&self) -> &Arc<Mutex<UserScriptStore>> {
        &self.store
    }
}

fn generate_empty_types_lib() -> (String, Vec<Diagnostic>) {
    // Implement this function to generate an empty types library
    (String::from("empty_types_lib"), vec![])
}

pub fn use_user_script_state<T>(selector: impl Fn(&UserScriptStore) -> T) -> T {
    let store = UserScriptStateProvider::new().get_store();

    std::thread::spawn(move || {
        let mut store = store.lock().unwrap();
        selector(&store)
    })
}
```