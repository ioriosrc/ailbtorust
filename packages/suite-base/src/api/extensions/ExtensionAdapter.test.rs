```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IExtensionApiResponse {
    id: String,
    createdAt: String,
    updatedAt: String,
    scope: Namespace,
    changelog: String,
    description: String,
    displayName: String,
    extensionId: String,
    fileId: String,
    homepage: String,
    keywords: Vec<String>,
    license: String,
    name: String,
    publisher: String,
    qualifiedName: String,
    readme: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionAdapter {
    // Implementation of the ExtensionAdapter struct
}

impl ExtensionAdapter {
    pub fn to_extension_info(api_response: &IExtensionApiResponse) -> ExtensionInfo {
        ExtensionInfo {
            ...api_response,
            id: api_response.extension_id.clone(),
            external_id: api_response.id.clone(),
            namespace: api_response.scope.clone(),
        }
    }

    pub fn to_stored_extension(
        api_response: &IExtensionApiResponse,
        workspace: String,
        custom_content: Option<&[u8]>,
    ) -> StoredExtension {
        let content = custom_content.unwrap_or_default().to_vec();
        StoredExtension {
            info: ExtensionInfo {
                ...api_response,
                id: api_response.extension_id.clone(),
                external_id: api_response.id.clone(),
                namespace: api_response.scope.clone(),
            },
            content,
            workspace,
            fileId: api_response.file_id.clone(),
            external_id: api_response.id.clone(),
        }
    }

    pub fn to_extension_info_list(api_responses: &[IExtensionApiResponse]) -> Vec<ExtensionInfo> {
        api_responses
            .iter()
            .map(|api_response| Self::to_extension_info(api_response))
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionInfo {
    // Implementation of the ExtensionInfo struct
}
```