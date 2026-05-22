```rust
use reqwest::{Client, RequestBuilder};
use serde_json::Value;

struct ExtensionInfoWorkspace {
    // Define the fields of ExtensionInfoWorkspace here
}

struct StoredExtension {
    // Define the fields of StoredExtension here
}

struct HttpError(reqwest::Error);

impl From<reqwest::Error> for HttpError {
    fn from(error: reqwest::Error) -> Self {
        HttpError(error)
    }
}

async fn get_extension_info(workspace: &str, id: &str) -> Result<StoredExtension, HttpError> {
    let client = Client::new();
    let url = format!("https://api.lichtblick.com/workspaces/{}/extensions/{}", workspace, id);
    let response = client.get(&url).await?;

    if !response.status().is_success() {
        return Err(HttpError(response.error_for_status()?));
    }

    let data: Value = serde_json::from_str(&response.text()?)?;
    Ok(ExtensionAdapter::to_stored_extension(&data, workspace))
}

async fn create_or_update_extension(
    workspace: &str,
    extension_info: ExtensionInfoWorkspace,
    file: std::fs::File,
) -> Result<StoredExtension, HttpError> {
    let client = Client::new();
    let url = format!("https://api.lichtblick.com/workspaces/{}/extension", workspace);
    let form = reqwest::multipart::Form::new()
        .text("file", file)
        .field(
            "changelog",
            extension_info.info.changelog,
        )
        .field(
            "description",
            extension_info.info.description,
        )
        .field(
            "displayName",
            extension_info.info.displayName,
        )
        .field(
            "extensionId",
            extension_info.info.id,
        )
        .field(
            "homepage",
            extension_info.info.homepage,
        )
        .field(
            "keywords",
            extension_info.info.keywords,
        )
        .field(
            "license",
            extension_info.info.license,
        )
        .field(
            "name",
            extension_info.info.name,
        )
        .field(
            "publisher",
            extension_info.info.publisher,
        )
        .field(
            "qualifiedName",
            extension_info.info.qualifiedName,
        )
        .field(
            "readme",
            extension_info.info.readme,
        )
        .field(
            "scope",
            "org",
        )
        .field(
            "version",
            extension_info.info.version,
        )
        .field("replace", "true");

    let response = client.post(&url).multipart(form).send().await?;

    if !response.status().is_success() {
        return Err(HttpError(response.error_for_status()?));
    }

    let data: Value = serde_json::from_str(&response.text()?)?;
    Ok(ExtensionAdapter::to_stored_extension(&data, workspace))
}

async fn remove_extension(workspace: &str, id: &str) -> Result<bool, HttpError> {
    let client = Client::new();
    let url = format!("https://api.lichtblick.com/workspaces/{}/extension/{}", workspace, id);

    if response.status().is_success() {
        Ok(true)
    } else if response.status().is_not_found() {
        return Ok(false);
    } else {
        return Err(HttpError(response.error_for_status()?));
    }
}

async fn load_content(workspace: &str, id: &str) -> Result<Vec<u8>, HttpError> {
    let client = Client::new();
    let url = format!("https://api.lichtblick.com/workspaces/{}/extension/{}/download", workspace, id);

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(HttpError(response.error_for_status()?));
    }

    Ok(response.bytes().await?)
}
```