```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use crate::remote_layout_storage::RemoteLayoutStorage;
use crate::layout_manager_service::LayoutManager;

pub struct LayoutManagerProvider {
    layout_manager: Arc<Mutex<LayoutManager>>,
}

impl LayoutManagerProvider {
    pub fn new(remote_layout_storage: &RemoteLayoutStorage) -> Self {
        let layout_manager = Arc::new(Mutex::new(LayoutManager::new(remote_layout_storage)));
        Self { layout_manager }
    }

    pub fn get_layout_manager(&self) -> Arc<Mutex<LayoutManager>> {
        self.layout_manager.clone()
    }
}
```