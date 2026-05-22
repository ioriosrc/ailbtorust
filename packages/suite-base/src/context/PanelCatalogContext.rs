```rust
use std::sync::{Arc, Mutex};
use std::future::{self, AsyncResult};

pub struct PanelComponent;

pub trait PanelStatics {
    fn new(config: PanelConfig) -> Self;
    fn update(&mut self, config: PanelConfig);
}

pub type PanelInfo = (String, String, Option<String>, Option<String>, Option<String>);

pub type PanelCatalog {
    panels: Arc<Mutex<Vec<PanelInfo>>>,
}

impl PanelCatalog {
    pub fn new() -> Self {
        PanelCatalog {
            panels: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn get_panels(&self) -> Vec<PanelInfo> {
        self.panels.lock().unwrap().clone()
    }

    pub fn get_panel_by_type(&self, type_: &str) -> Option<&PanelInfo> {
        for panel in self.get_panels().await.iter() {
            if panel.1 == type_ {
                return Some(panel);
            }
        }
        None
    }

    pub async fn register_panel(&mut self, name: &str, title: &str, type_: &str, description: Option<&str>, thumbnail: Option<&str>, help: Option<AsyncResult<impl 'static + Fn() -> String>>) {
        let mut panels = self.panels.lock().unwrap();
        panels.push((
            name.to_string(),
            title.to_string(),
            description.map(String::from),
            thumbnail.map(String::from),
            help.clone(),
        ));
    }

    pub fn unregister_panel(&mut self, name: &str) {
        let mut panels = self.panels.lock().unwrap();
        panels.retain(|panel| panel.0 != name);
    }
}

pub struct PanelCatalogContext(Arc<Mutex<PanelCatalog>>);

impl PanelCatalogContext {
    pub fn new(panels: Arc<Mutex<PanelCatalog>>) -> Self {
        PanelCatalogContext(panels)
    }

    pub fn get_panels(&self) -> Arc<Mutex<Vec<PanelInfo>>> {
        self.0.clone()
    }
}

pub fn use_panel_catalog() -> Arc<Mutex<Vec<PanelInfo>>> {
    let context = &*PanelCatalogContext::default();
    context.get_panels().clone()
}
```