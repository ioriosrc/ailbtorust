```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceDialog {
    pub active_data_source: Option<String>,
    pub item: Option<String>,
    pub open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferencesDialog {
    pub initial_tab: Option<String>,
    pub open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureToursState {
    pub active: Option<bool>;
    pub shown: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarState {
    pub left: SidebarSection;
    pub right: SidebarSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarSection {
    pub item: String,
    pub open: bool,
    pub size: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackControlsState {
    pub repeat: bool,
    pub sync_instances: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceContextStore {
    dialogs: HashMap<String, DataSourceDialog | PreferencesDialog>;
    feature_tours: FeatureToursState;
    sidebars: SidebarState;
    playback_controls: PlaybackControlsState;
}

impl Default for WorkspaceContextStore {
    fn default() -> Self {
        WorkspaceContextStore {
            dialogs: HashMap::new(),
            feature_tours: FeatureToursState {
                active: None,
                shown: Vec::new(),
            },
            sidebars: SidebarState {
                left: SidebarSection {
                    item: "panel-settings".to_string(),
                    open: true,
                    size: None,
                },
                right: SidebarSection {
                    item: None,
                    open: false,
                    size: None,
                },
            },
            playback_controls: PlaybackControlsState {
                repeat: false,
                sync_instances: false,
            },
        }
    }
}

fn make_workspace_context_initial_state() -> WorkspaceContextStore {
    WorkspaceContextStore::default()
}

fn create_workspace_context_store(
    initial_state: Option<WorkspaceContextStore>,
    options: Option<HashMap<String, &'static str>>,
) -> StoreApi<WorkspaceContextStore> {
    let state_creator = || {
        let store = initial_state.unwrap_or_else(|| make_workspace_context_initial_state());
        store
    };

    if let Some(options) = options {
        use zustand::persist;

        persist(state_creator, |state| {
            // Note that this is an opt-in list of keys from the store that we
            // include and restore when persisting to and from localStorage.
            state.filter(|k| k.starts_with("featureTours") || k == "playbackControls" || k == "sidebars")
        })
    } else {
        use zustand::create;

        create(state_creator)
    }
}

pub fn WorkspaceContextProvider(props: {
    children: ReactNode;
    disablePersistenceForStorybook: bool;
    initialState: Option<WorkspaceContextStore>;
    workspaceStoreCreator: Option<
        (
            Option<&'static str>,
            Option<HashMap<String, &'static str>>,
        ) -> StoreApi<WorkspaceContextStore>
    >;
}) -> ReactElement {
    let { children, initialState, disablePersistenceForStorybook } = props;

    let store = use_store(create_workspace_context_store(initialState, Some(options)));

    return <WorkspaceContext.Provider value={store}>{children}</WorkspaceContext.Provider>;
}
```