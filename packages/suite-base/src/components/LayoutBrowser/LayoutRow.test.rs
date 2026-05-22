```rust
// Import necessary modules and types from Rust
use std::sync::Arc;
use test_lib::{render_with, wait_for};

// Define the necessary structs and functions from TypeScript/React
struct LayoutRow {
    layout: Arc<DefaultLayout>,
    any_selected_modified_layouts: bool,
    multi_selected_ids: Vec<String>,
    selected: bool,
    onSelect: fn(),
    onRename: fn(String),
    onDuplicate: fn(Arc<DefaultLayout>),
    onDelete: fn(Arc<DefaultLayout>),
    onShare: fn(Arc<DefaultLayout>),
    onExport: fn(Arc<DefaultLayout>),
    onOverwrite: fn(Arc<DefaultLayout>),
    onRevert: fn(Arc<DefaultLayout>, String),
}

// Implement the necessary methods for LayoutRow
impl LayoutRow {
    fn new(layout: Arc<DefaultLayout>) -> Self {
        LayoutRow {
            layout,
            any_selected_modified_layouts: false,
            multi_selected_ids: Vec::new(),
            selected: false,
            onSelect: |_| {},
            onRename: |_| {},
            onDuplicate: |_| {},
            onDelete: |_| {},
            onShare: |_| {},
            onExport: |_| {},
            onOverwrite: |_| {},
            onRevert: |_| {},
        }
    }

    fn select(&mut self) {
        self.selected = true;
    }

    fn unselect(&mut self) {
        self.selected = false;
    }
}

// Define the necessary structs and functions from TypeScript/React
struct DefaultLayout {
    id: String,
    name: String,
    working: Option<HashMap<String, serde_json::Value>>,
    syncInfo: Option<SyncInfo>,
    permission: LayoutPermission,
}

// Implement the necessary methods for DefaultLayout
impl DefaultLayout {
    fn new(id: String, name: String) -> Self {
        DefaultLayout {
            id,
            name,
            working: None,
            syncInfo: None,
            permission: LayoutPermission::CREATOR_WRITE,
        }
    }

    fn has_modifications(&self) -> bool {
        // Implement logic to check if the layout has modifications
        false
    }
}

// Define the necessary structs and functions from TypeScript/React
struct SyncInfo {
    status: String,
    error: Option<String>,
}

// Define the necessary enums and constants from TypeScript/React
enum LayoutPermission {
    ORG_READ,
    CREATOR_WRITE,
}

// Implement the necessary mock functions for LayoutRow
fn mock_layout() -> Arc<DefaultLayout> {
    let layout = DefaultLayout::new(BasicBuilder.string(), BasicBuilder.string());
    Arc::new(layout)
}

fn mock_confirm_mocked(resolved_value: &str) -> Box<dyn FnMut(String) -> String + Send + Sync> {
    Box::new(move |value| if value == "ok" { resolved_value.to_string() } else { "cancel".to_string() })
}

// Define the necessary test cases for LayoutRow
fn test_layout_row() {
    // Test cases for different scenarios and conditions
    // ...
}
```