```rust
use chrono::{DateTime, Utc};

// =============================================================================
// API Data Transfer Objects (DTOs)
// =============================================================================

#[derive(Debug, Clone)]
pub struct LayoutApiData {
    pub id: String,
    pub layout_id: LayoutID,
    pub name: String,
    pub data: LayoutData,
    pub workspace: String,
    pub permission: LayoutPermission,
    pub from: String,
    pub created_by: String,
    pub updated_by: String,
}

// =============================================================================
// API Request Types
// =============================================================================

#[derive(Debug, Clone)]
pub struct CreateLayoutRequest {
    pub layout_id: LayoutID,
    pub name: String,
    pub data: LayoutData,
    pub permission: LayoutPermission,
}

#[derive(Debug, Clone)]
pub struct UpdateLayoutRequest {
    pub id: LayoutID,
    pub external_id: String,
    pub name: Option<String>,
    pub data: Option<LayoutData>,
    pub permission: Option<LayoutPermission>,
    pub saved_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UpdateLayoutRequestBody {
    pub name: Option<String>;
    pub data: Option<LayoutData>;
    pub permission: Option<LayoutPermission>;
}

// =============================================================================
// API Response Types
// =============================================================================

#[derive(Debug, Clone)]
pub enum UpdateLayoutResponse {
    Success { new_layout: RemoteLayout },
    Conflict,
}

#[derive(Debug, Clone)]
pub type LayoutApiResponse = LayoutApiData;

pub type WorkspaceLayoutResponse = {
    layout: LayoutApiData;
};

// =============================================================================
// Service Layer Types
// =============================================================================

#[derive(Debug, Clone)]
pub struct SaveNewLayoutParams {
    pub id: Option<String>,
    pub name: String,
    pub permission: LayoutPermission,
    pub data: LayoutData,
}
```