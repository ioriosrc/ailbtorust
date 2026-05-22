```rust
use std::collections::HashMap;

// Define the DesktopExtensionInfo struct based on your requirements
#[derive(Debug)]
struct DesktopExtensionInfo {
    // Define fields as needed
}

// Define the LoadedExtension struct based on your requirements
#[derive(Debug)]
struct LoadedExtension {
    raw: String,
}

// Define the ExtensionLoader struct with mock implementations
pub struct DesktopExtensionLoader {
    pub namespace: String,
}

impl DesktopExtensionLoader {
    pub fn new(namespace: String) -> Self {
        Self { namespace }
    }

    // Mock implementation for getExtensions
    pub async fn get_extensions(&self) -> Vec<DesktopExtensionInfo> {
        let extensions: Vec<DesktopExtensionInfo> = vec![
            DesktopExtensionInfo {
                name: self.namespace.to_string(),
            },
            DesktopExtensionInfo {
                name: self.namespace.to_string(),
            },
        ];
        extensions
    }

    // Mock implementation for load_extension
    pub async fn load_extension(&self, extension_id: String) -> LoadedExtension {
        LoadedExtension { raw: format!("loaded; id={}", extension_id) }
    }

    // Mock implementation for install_extension
    pub async fn install_extension(
        &self,
        foxe_file_data: &[u8],
    ) -> Result<DesktopExtensionInfo, String> {
        Ok(DesktopExtensionInfo {
            name: self.namespace.to_string(),
        })
    }

    // Mock implementation for uninstall_extension
    pub async fn uninstall_extension(&self, extension_id: String) {
        println!("Uninstalling extension {}", extension_id);
    }
}
```