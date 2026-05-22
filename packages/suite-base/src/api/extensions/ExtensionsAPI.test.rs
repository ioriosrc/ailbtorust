```rust
use std::fs::File;
use std::io::{self, BufReader};

use crate::common::{
    extensions_api::CreateOrUpdateResponse,
    models::ExtensionInfoWorkspace,
};
use crate::service::IExtensionStorage;
use crate::util::HttpError;

pub struct ExtensionsAPI {
    workspace: String,
}

impl ExtensionsAPI {
    pub fn new(workspace: String) -> Self {
        Self { workspace }
    }

    async fn list(&self) -> Vec<StoredExtension> {
        // Simulate fetching extensions from the API
        let extensions = ExtensionBuilder.extensions_info();
        extensions.into_iter().collect()
    }

    async fn get(&self, extension_id: &str) -> Option<StoredExtension> {
        let mock_content = BasicBuilder.string().as_bytes();
        let mock_file = File::open("mockfile.zip").expect("Failed to open file");
        let reader = BufReader::new(mock_file);
        let mut content = Vec::with_capacity(reader.get_ref().len());
        io::copy(&mut reader, &mut content).unwrap();

        Some(StoredExtension {
            info: ExtensionInfoWorkspace {
                extension_id: extension_id.to_string(),
                workspace: self.workspace.clone(),
                ...mock_content,
            },
            content: content,
            file_id: "file-123".to_string(),
            external_id: extension_id.to_string(),
        })
    }

    async fn create_or_update(
        &self,
        extension: ExtensionInfoWorkspace,
        mock_file: File,
    ) -> Result<StoredExtension, HttpError> {
        // Simulate creating or updating the extension
        let file_content = io::read_to_string(&mock_file)?;
        let content = file_content.as_bytes();

        Ok(StoredExtension {
            info: extension,
            content: content.into(),
            file_id: "file-123".to_string(),
            external_id: extension.extension_id.clone(),
        })
    }

    async fn remove(&self, extension_id: &str) -> bool {
        // Simulate removing the extension
        true
    }

    async fn load_content(&self, id: &str) -> Option<io::Cursor<Vec<u8>>> {
        let mock_file = File::open("mockfile.zip").expect("Failed to open file");
        let reader = BufReader::new(mock_file);
        let mut content = Vec::with_capacity(reader.get_ref().len());
        io::copy(&mut reader, &mut content).unwrap();

        Some(io::Cursor::from(content))
    }

    async fn error_handling(&self) -> Result<(), HttpError> {
        // Simulate an HTTP error
        Err(HttpError::new("Network error", 500, "Network error"))
    }
}
```