```rust
use async_std::task;
use electron::{ipc, webview::WebView};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UninstallExtensionRequest {
    filename: String,
}

#[derive(Serialize, Deserialize)]
struct UninstallResponse {
    status: String,
}

async fn handle_uninstall_request(mut request: UninstallExtensionRequest, sender: ipc::Sender<UninstallResponse>) {
    // Simulate uninstalling the extension
    if request.filename == "lichtblick.suite-extension-turtlesim-0.0.1.foxe" {
        sender.send(UninstallResponse { status: "success".to_string() }).await;
    } else {
        sender.send(UninstallResponse { status: "error".to_string() }).await;
    }
}

async fn main() {
    let mut webview = WebView::new().unwrap();

    // Simulate loading the extension file
    task::spawn(async move {
        let response = ipc::send_ipc("load_files", UninstallExtensionRequest {
            filename: "lichtblick.suite-extension-turtlesim-0.0.1.foxe".to_string(),
        });
        println!("Load files response: {:?}", response);
    });

    // Simulate navigating to the extensions menu and selecting "turtlesim"
    task::spawn(async move {
        let response = ipc::send_ipc("navigate_to_extensions", ());
        println!("Navigate to extensions response: {:?}", response);

        let response = ipc::send_ipc("search_extensions", "turtlesim");
        println!("Search extensions response: {:?}", response);

        let response = ipc::send_ipc("select_extension", "turtlesim 0.0.1");
        println!("Select extension response: {:?}", response);
    });

    // Simulate clicking the uninstall button
    task::spawn(async move {
        let response = ipc::send_ipc("click_uninstall_button", ());
        println!("Click uninstall button response: {:?}", response);

        let response = ipc::send_ipc("wait_for_uninstalling_toast", ());
        println!("Wait for uninstalling toast response: {:?}", response);
    });

    // Simulate confirming the uninstall
    task::spawn(async move {
        let response = ipc::send_ipc("confirm_uninstall", ());
        println!("Confirm uninstall response: {:?}", response);

        let response = ipc::send_ipc("wait_for_uninstall_toast", ());
        println!("Wait for uninstall toast response: {:?}", response);
    });
}
```