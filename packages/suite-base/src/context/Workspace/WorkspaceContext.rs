```rust
use std::rc::Rc;

pub struct WorkspaceItemKey(String);

impl From<&str> for WorkspaceItemKey {
    fn from(value: &str) -> Self {
        WorkspaceItemKey(value.to_string())
    }
}

#[derive(Default)]
pub struct LeftSidebarItemKey(String);

impl From<&str> for LeftSidebarItemKey {
    fn from(value: &str) -> Self {
        LeftSidebarItemKey(value.to_string())
    }
}

#[derive(Default)]
pub struct RightSidebarItemKey(String);

impl From<&str> for RightSidebarItemKey {
    fn from(value: &str) -> Self {
        RightSidebarItemKey(value.to_string())
    }
}

#[derive(Default)]
pub struct WorkspaceContextStore {
    dialogs: Option<Dialogs>,
    feature_tours: Option<FeatureTours>,
    playback_controls: PlaybackControls,
    sidebars: Sidebars,
}

pub struct Dialogs {
    active_data_source: Option<DataSourceFactory>,
    item: Option<DataSourceDialogItem>,
    open: bool,
}

pub struct Preferences {
    initial_tab: Option<AppSettingsTab>,
    open: bool,
}

pub struct FeatureTours {
    active: Option<String>,
    shown: Vec<String>,
}

#[derive(Default)]
pub struct PlaybackControls {
    repeat: bool,
    sync_instances: bool,
}

#[derive(Default)]
pub struct Sidebars {
    left: Sidebar,
    right: Sidebar,
}

pub struct Sidebar {
    item: Option<LeftSidebarItemKey>,
    open: bool,
    size: Option<usize>,
}

impl Default for WorkspaceContext {
    fn default() -> Self {
        WorkspaceContext {
            dialogs: None,
            feature_tours: None,
            playback_controls: PlaybackControls::default(),
            sidebars: Sidebars::default(),
        }
    }
}

pub struct AppSettingsTab(String);

impl From<&str> for AppSettingsTab {
    fn from(value: &str) -> Self {
        AppSettingsTab(value.to_string())
    }
}

#[derive(Default)]
pub struct DataSourceFactory;

pub struct DataSourceDialogItem {}

// ... (rest of the code remains the same)
```