```rust
use leptos::{html, IntoView};

async fn fake_panel() -> impl IntoView {
    html! {
        <div>
            <PanelToolbar />
            <div>I'm a fake panel</div>
        </div>
    }
}

async fn droppable_panel() -> impl IntoView {
    html! {
        <div>
            <PanelToolbar />
            <div>Drop here!</div>
            <pre>{JSON.stringify([], 2)}</pre>
        </div>
    }
}

pub struct MockPanelCatalog {
    fake_panel: PanelInfo,
    droppable_panel: PanelInfo,
}

impl MockPanelCatalog {
    pub fn new() -> Self {
        Self {
            fake_panel: PanelInfo::new("Fake Panel", "Fake", async () -> Result<_, Error> {
                let module = || html! {
                    <div>
                        <PanelToolbar />
                        <div>I'm a fake panel</div>
                    </div>
                };
                Ok(Panel { default_config: {}, module })
            }),
            droppable_panel: PanelInfo::new("Droppable Panel", "Droppable", async () -> Result<_, Error> {
                let module = || html! {
                    <div>
                        <PanelToolbar />
                        <div>Drop here!</div>
                        <pre>{JSON.stringify([], 2)}</pre>
                    </div>
                };
                Ok(Panel { default_config: {}, module })
            }),
        }
    }

    pub fn get_panels(&self) -> Vec<PanelInfo> {
        vec![self.fake_panel.clone(), self.droppable_panel.clone()]
    }

    pub fn get_panel_by_type(&self, type: &str) -> Option<&PanelInfo> {
        self.get_panels().find(|panel| panel.type == type)
    }
}

#[derive(Serialize, Deserialize)]
struct LayoutData {
    layout: String,
    configById: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Fixture {
    topics: Vec<Topic>,
    datatypes: Map<String, Datatype>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    name: String,
    schema_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datatype {
    definitions: Vec<DatatypeDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatatypeDefinition {
    name: String,
    type: String,
}

async fn workspace(initial_layout_state: LayoutData) -> impl IntoView {
    html! {
        <Workspace initial_layout_state={initial_layout_state} />
    }
}
```