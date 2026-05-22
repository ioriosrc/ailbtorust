```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// individual app configuration values are basic primitive types
pub type AppConfigurationValue = String | i32 | bool | Option<String>;

pub struct ChangeHandler {
    cb: Box<dyn Fn(AppConfigurationValue)>,
}

pub struct IAppConfiguration {
    storage: Arc<Mutex<HashMap<String, AppConfigurationValue>>>,
    change_handlers: Arc<Mutex<HashMap<String, Vec<ChangeHandler>>>>,
}

impl IAppConfiguration {
    fn new() -> Self {
        let storage = Arc::new(Mutex::new(HashMap::new()));
        let change_handlers = Arc::new(Mutex::new(HashMap::new()));

        let context = IAppConfiguration {
            storage,
            change_handlers,
        };

        let context_weak = Arc::downgrade(&context);

        for (_, handlers) in &mut *change_handlers.lock().unwrap() {
            for handler in handlers.iter() {
                handler.cb(Box::new(move |value| {
                    if !context_weak.upgrade().unwrap().storage.lock().unwrap().contains_key(handler.key.as_str()) {
                        context_weak.upgrade().unwrap().storage.lock().unwrap().insert(handler.key.clone(), value);
                    }
                    context_weak.upgrade().unwrap().notify_change_handlers(handler.key.clone(), value);
                }));
            }
        }

        context
    }

    fn get(&self, key: &str) -> AppConfigurationValue {
        self.storage.lock().unwrap().get(key).cloned().unwrap_or_default()
    }

    async fn set(&self, key: &str, value: AppConfigurationValue) -> Result<(), String> {
        let mut storage = self.storage.lock().unwrap();
        if !storage.contains_key(key) {
            return Err("Key not found".to_string());
        }
        storage.insert(key.to_string(), value.clone());

        // Notify all change handlers
        for handler in &mut *self.change_handlers.lock().unwrap() {
            if handler.key.as_str() == key {
                for cb in handler.cb.iter() {
                    cb(value.clone());
                }
            }
        }

        Ok(())
    }

    fn addChangeListener(&self, key: &str, cb: ChangeHandler) {
        let mut handlers = self.change_handlers.lock().unwrap();
        if !handlers.contains_key(key) {
            handlers.insert(key.to_string(), vec![cb]);
        } else {
            handlers.get_mut(key).unwrap().push(cb);
        }
    }

    fn removeChangeListener(&self, key: &str, cb: ChangeHandler) {
        let mut handlers = self.change_handlers.lock().unwrap();
        if let Some(handlers) = handlers.get_mut(key) {
            let index = handlers.iter().position(|h| h.cb.as_ref() == cb).unwrap_or_default();
            handlers.remove(index);
        }
    }

    fn notify_change_handlers(&self, key: String, value: AppConfigurationValue) {
        for handler in &mut *self.change_handlers.lock().unwrap()[&key] {
            handler.cb(value.clone());
        }
    }
}

// Exposes an interface for reading and writing user-configurable options and other persistent application state.
pub struct AppConfigurationContext;

impl Context<IAppConfiguration> for AppConfigurationContext {
    fn get_current(&self) -> Option<&IAppConfiguration> {
        let context = Arc::downgrade(self);
        if let Some(context) = context.upgrade() {
            return Some(context);
        }
        None
    }

    fn provide<P: IAppConfiguration>(&self, provider: P) -> Self {
        let storage = Arc::new(Mutex::new(provider.storage.lock().unwrap().clone()));
        let change_handlers = Arc::new(Mutex::new(provider.change_handlers.lock().unwrap().clone()));

        let context = IAppConfiguration {
            storage,
            change_handlers,
        };

        let context_weak = Arc::downgrade(&context);

        for (_, handlers) in &mut *change_handlers.lock().unwrap() {
            for handler in handlers.iter() {
                handler.cb(Box::new(move |value| {
                    if !context_weak.upgrade().unwrap().storage.lock().unwrap().contains_key(handler.key.as_str()) {
                        context_weak.upgrade().unwrap().storage.lock().unwrap().insert(handler.key.clone(), value);
                    }
                    context_weak.upgrade().unwrap().notify_change_handlers(handler.key.clone(), value);
                }));
            }
        }

        context
    }
}

pub fn useAppConfiguration() -> IAppConfiguration {
    AppConfigurationContext::get_current().unwrap()
}
```