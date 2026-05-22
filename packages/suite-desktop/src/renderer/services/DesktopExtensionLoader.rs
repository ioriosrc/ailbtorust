```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExtensionInfo {
    id: String,
    name: String,
    namespace: String,
    qualified_name: String,
    readme: Option<String>,
    changelog: Option<String>,
}

impl ExtensionInfo {
    pub fn new(id: &str, name: &str, namespace: &str) -> Self {
        ExtensionInfo {
            id: id.to_string(),
            name: name.to_string(),
            namespace: namespace.to_string(),
            qualified_name: name.to_string(), // Qualified name is display name for backwards compatibility with existing layouts.
            readme: None,
            changelog: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DesktopExtension {
    id: String,
    package_json: ExtensionInfo,
    readme: Option<String>,
    changelog: Option<String>,
}

impl DesktopExtension {
    pub fn new(id: &str, pkg_info: ExtensionInfo, readme: Option<&str>, changelog: Option<&str>) -> Self {
        DesktopExtension {
            id: id.to_string(),
            package_json: pkg_info,
            readme: readme.map(|s| s.to_string()),
            changelog: changelog.map(|s| s.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoadedExtension {
    extension: ExtensionInfo,
}

impl LoadedExtension {
    pub fn new(extension: ExtensionInfo) -> Self {
        LoadedExtension { extension }
    }
}
```