```rust
use serde_json::{self, Deserialize};

#[derive(Deserialize)]
struct ExtensionInfo {
    id: String,
    external_id: String,
    namespace: String,
}

// Assuming IExtensionApiResponse and StoredExtension are defined elsewhere
#[derive(Debug)]
pub struct IExtensionApiResponse {
    extension_id: String,
    // Other fields of IExtensionApiResponse
}

#[derive(Debug)]
pub struct StoredExtension {
    info: ExtensionInfo,
    content: Vec<u8>,
    workspace: String,
    fileId: Option<String>,
    external_id: String,
}

impl From<IExtensionApiResponse> for StoredExtension {
    fn from(api_response: IExtensionApiResponse) -> Self {
        StoredExtension {
            info: api_response.into(),
            content: Vec::new(), // Default to an empty vector
            workspace: "default".to_string(), // Default to a default workspace name
            fileId: None,
            external_id: api_response.external_id,
        }
    }
}

// Example usage:
fn main() {
    let api_response = IExtensionApiResponse {
        extension_id: "example-extension".to_string(),
        // Initialize other fields of IExtensionApiResponse
    };

    let stored_extension = StoredExtension::from(api_response);

    println!("{:#?}", stored_extension);
}
```

Note: The provided Rust code is a simplified example to illustrate the conversion from TypeScript/React to Rust. In actual applications, you would need to handle more complex scenarios such as deserialization of JSON responses, error handling, and database interactions.