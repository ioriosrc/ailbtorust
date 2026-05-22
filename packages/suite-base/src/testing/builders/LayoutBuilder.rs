```rust
use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LayoutPermission {
    pub creator_write: bool,
    pub org_read: bool,
    pub org_write: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutSyncStatus {
    pub new: bool,
    pub updated: bool,
    pub tracked: bool,
    pub locally_deleted: bool,
    pub remotely_deleted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlaybackConfig {
    speed: f64,
}

#[derive(Serialize, Deserialize)]
pub struct UserScript {
    name: String,
    source_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserScripts(Vec<UserScript>);

#[derive(Serialize, Deserialize)]
pub struct LayoutData {
    config_by_id: serde_json::Value,
    global_variables: Vec<GlobalVariable>,
    user_nodes: UserScripts,
    playback_config: PlaybackConfig,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutBaseline {
    data: LayoutData,
    saved_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutSyncInfo {
    status: LayoutSyncStatus,
    last_remote_saved_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Layout {
    id: LayoutID,
    external_id: String,
    name: String,
    from: String,
    permission: LayoutPermission,
    baseline: LayoutBaseline,
    working: LayoutBaseline,
    sync_info: LayoutSyncInfo,
}

#[derive(Serialize, Deserialize)]
pub struct RemoteLayout {
    id: String,
    external_id: String,
    name: String,
    permission: LayoutPermission,
    data: LayoutData,
    saved_at: String,
}

fn layout_id(default_id: Option<&str>) -> LayoutID {
    (default_id.unwrap_or_default()).to_string()
}
```