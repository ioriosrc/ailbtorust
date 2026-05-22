```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WorkspaceContextStoreV0 {
    #[serde(default = "None")]
    pub dataSource_dialog: Option<DataSourceDialogItem>,
    #[serde(default = "None")]
    pub item: Option<IDataSourceFactory>,
    #[serde(default = "false")]
    pub open: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FeatureToursState {
    #[serde(default = "None")]
    pub active: Option<String>,
    #[serde(default = "Vec::new")]
    pub shown: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LeftSidebarItemKey(u32);

#[derive(Serialize, Deserialize)]
pub struct RightSidebarItemKey(u32);

#[derive(Serialize, Deserialize)]
pub struct PlaybackControls {
    repeat: bool,
    sync_instances: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AppSettingsTab;

#[derive(Serialize, Deserialize)]
pub struct WorkspaceContextStore {
    dialogs: {
        dataSource: {
            activeDataSource: Option<IDataSourceFactory>,
            item: Option<DataSourceDialogItem>,
            open: bool,
        },
        preferences: {
            initialTab: Option<AppSettingsTab>,
            open: bool,
        },
    },
    featureTours: FeatureToursState,
    sidebars: {
        left: {
            item: LeftSidebarItemKey,
            open: bool,
            size: Option<u32>,
        },
        right: {
            item: RightSidebarItemKey,
            open: bool,
            size: Option<u32>,
        },
    },
    playbackControls: PlaybackControls,
}
```