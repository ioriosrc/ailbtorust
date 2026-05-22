```rust
use std::sync::{Arc, RwLock};

pub enum ForwardedMenuEvent {
    Open,
    OpenFile,
    OpenConnection,
    OpenDemo,
    OpenHelpAbout,
    OpenHelpDocs,
    OpenHelpGeneral,
}

pub type UnregisterFn = fn();

#[derive(Clone)]
pub struct NativeMenuBridge {
    listeners: Arc<RwLock<Vec<fn() -> ()>>>,
}

impl NativeMenuBridge {
    pub fn add_ipc_event_listener(
        &self,
        event_name: ForwardedMenuEvent,
        handler: fn() -> (),
    ) -> UnregisterFn {
        let listeners = self.listeners.clone();
        move || {
            listeners.write().unwrap().push(handler);
        }
    }
}

pub type StorageContent = String | Vec<u8>;

#[derive(Clone)]
pub struct Storage {
    list: Arc<RwLock<Vec<String>>>,
    all: Arc<RwLock<Vec<StorageContent>>>,
    get: Arc<RwLock<HashMap<&str, Option<StorageContent>>>>,
    put: Arc<RwLock<HashMap<&str, Vec<u8>>>>,
    delete: Arc<RwLock<HashMap<&str, ()>>>,

    directory: String,
}

impl Storage {
    pub fn list(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(self.list.read().unwrap().clone())
    }

    pub fn all(&self) -> Result<Vec<StorageContent>, Box<dyn std::error::Error>> {
        Ok(self.all.read().unwrap().clone())
    }

    pub fn get(
        &self,
        key: &str,
        options: Option<&struct { encoding: &'static str }>,
    ) -> Result<Option<StorageContent>, Box<dyn std::error::Error>> {
        match options {
            Some(options) if options.encoding == "utf8" => Ok(self.get.read().unwrap().get(key)?.map(|b| String::from_utf8_lossy(b).into())),
            _ => Ok(self.get.read().unwrap().get(key)?.map(|b| b.clone())),
        }
    }

    pub fn put(
        &self,
        key: &str,
        value: StorageContent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.put.write().unwrap().insert(key, value);
        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.delete.write().unwrap().remove(key);
        Ok(())
    }
}

pub type DesktopExtension = struct {
    id: String,
    package_json: serde_json::Value,
    directory: String,
    readme: String,
    changelog: String,
};

pub type DesktopLayout = struct {
    layout_json: serde_json::Value,
    from: String,
};

#[derive(Debug, Clone)]
pub struct CLIFlags(serde_json::Value);

pub type LoadedExtension = struct {
    buffer: Option<Vec<u8>>,
    raw: String,
};

pub trait Desktop {
    fn set_represented_filename(&self, path: Option<&str>) -> Result<(), Box<dyn std::error::Error>>;
    fn add_ipc_event_listener(
        &self,
        event_name: ForwardedWindowEvent,
        handler: fn() -> (),
    ) -> UnregisterFn;

    fn update_native_color_scheme(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn get_deep_links(&self) -> Vec<String>;

    fn reset_deep_links(&self);

    fn get_extensions(&self) -> Result<Vec<DesktopExtension>, Box<dyn std::error::Error>>;

    fn load_extension(&self, id: &str) -> Result<LoadedExtension, Box<dyn std::error::Error>>;

    fn fetch_layouts(&self) -> Result<Vec<DesktopLayout>, Box<dyn std::error::Error>>;

    fn install_extension(&self, foxe_file_data: Vec<u8>) -> Result<DesktopExtension, Box<dyn std::error::Error>>;

    fn uninstall_extension(&self, id: &str) -> bool;

    fn get_cli_flags(&self) -> Result<CLIFlags, Box<dyn std::error::Error>>;

    fn handle_title_bar_double_click(&self);

    fn is_maximized(&self) -> bool;
    fn minimize_window(&self);
    fn maximize_window(&self);
    fn unmaximize_window(&self);
    fn close_window(&self);
    fn reload_window(&self);

    fn update_language(&self);
}
```