```rust
use std::collections::{HashMap, HashSet};

struct ExtensionInfo {
    // Define the fields of ExtensionInfo here
}

pub struct StoredExtension {
    pub info: ExtensionInfo,
    pub content: Vec<u8>,
    pub workspace: Option<String>,
    pub file_id: Option<String>,
    pub external_id: Option<String>,
}

impl StoredExtension {
    fn from(info: ExtensionInfo, content: Vec<u8>, workspace: Option<&str>, file_id: Option<&str>, external_id: Option<&str>) -> Self {
        Self {
            info,
            content,
            workspace: workspace.map(|s| s.to_string()),
            file_id: file_id.map(|s| s.to_string()),
            external_id: external_id.map(|s| s.to_string()),
        }
    }
}

pub struct IExtensionStorage {
    namespace: String,
    extensions: HashMap<String, StoredExtension>,
}

impl IExtensionStorage {
    fn new(namespace: &str) -> Self {
        IExtensionStorage {
            namespace: namespace.to_string(),
            extensions: HashMap::new(),
        }
    }

    async fn list(&self) -> Vec<ExtensionInfo> {
        // Implement the logic to retrieve all stored extensions
        unimplemented!()
    }

    async fn get(&self, id: &str) -> Option<StoredExtension> {
        self.extensions.get(id).cloned().map(|stored| stored.clone())
    }

    async fn put(&mut self, extension: StoredExtension) -> Result<StoredExtension, Box<dyn std::error::Error>> {
        self.extensions.insert(extension.info.name.to_string(), extension);
        Ok(extension)
    }

    async fn delete(&mut self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.extensions.remove(id).is_some() {
            Ok(())
        } else {
            Err("Extension not found".into())
        }
    }
}
```