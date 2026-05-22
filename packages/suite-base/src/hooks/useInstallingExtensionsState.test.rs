```rust
use actix_web::{App, HttpServer};
use serde_json::Value;
use std::sync::Mutex;

// Define the types as per your requirements
type ExtensionData = serde_json::Value;
type InstallExtensionsResult = serde_json::Value;

async fn handle_install_extensions_request(
    req: HttpRequest,
    extensions_data: Vec<ExtensionData>,
) -> HttpResponse {
    // Simulate the installation process
    let success_count = match extensions_data.len() {
        0 => 0,
        _ => 1, // For simplicity, assume only one extension is installed successfully
    };

    let result = if success_count == extensions_data.len() {
        Ok(InstallExtensionsResult::try_from(json!({
            "success": true,
            "extensionName": "Foxe Extensions",
            "loaderResults": [
                { "loaderType": "browser", "success": true },
                { "loaderType": "server", "success": true }
            ]
        }))
    } else {
        Err(InstallExtensionsResult::try_from(json!({
            "success": false,
            "extensionName": "Foxe Extensions",
            "error": new Error("One or more extensions failed to install."),
            "loaderResults": [
                { "loaderType": "browser", "success": false },
                { "loaderType": "server", "success": false }
            ]
        }))
    };

    HttpResponse::Ok()
        .json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/install-foxe-extensions", web::post().to(handle_install_extensions_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```