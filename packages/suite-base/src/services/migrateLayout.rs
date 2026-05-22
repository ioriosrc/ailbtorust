```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LayoutData {
    config_by_id: Option<HashMap<String, Layout>>,
}

#[derive(Serialize, Deserialize)]
pub struct ISO8601Timestamp(String);

#[derive(Serialize, Deserialize)]
pub struct Layout {
    id: String,
    external_id: Option<String>,
    from: Option<String>,
    name: Option<String>,
    permission: String,
    working: Option<WorkingLayout>,
    baseline: BaselineLayout,
    sync_info: Option<SyncInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkingLayout {
    data: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct BaselineLayout {
    data: HashMap<String, serde_json::Value>,
    saved_at: ISO8601Timestamp,
}

#[derive(Serialize, Deserialize)]
pub struct SyncInfo {
    // Define any sync-related information if needed
}

// Migrate legacy to new 3D panels format
fn migrate_legacy_to_new_3dp_panels(layout_data: &LayoutData) -> LayoutData {
    let mut result = layout_data.clone();

    if result.config_by_id.is_none() {
        result.config_by_id = Some(HashMap::new());
    }

    // Perform migration logic here

    result
}

// Import a layout from storage, transferring old properties to the current expected format
fn migrate_layout(layout: &Layout) -> Layout {
    let mut baseline = layout.baseline.clone();
    if baseline.data.is_none() {
        if let Some(working) = &layout.working {
            baseline.data = Some(migrate_panels_state(&working.data));
        } else {
            baseline.data = Some(serde_json::Value::Null);
        }
    }

    Layout {
        id: layout.id.clone(),
        external_id: layout.external_id.clone(),
        from: layout.from.clone(),
        name: layout.name.clone().unwrap_or_else(|| format!("Unnamed ({})", std::time::SystemTime::now().to_string())),
        permission: layout.permission.clone().unwrap_or_default().uppercase(),
        working: migrate_panels_state(&layout.working),
        baseline,
        sync_info: layout.sync_info.clone(),
    }
}
```