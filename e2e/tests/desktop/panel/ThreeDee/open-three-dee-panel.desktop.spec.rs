```rust
use electron::{BrowserWindow, Page};
use playwright::api::{Context, Locator};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let context = playwright::chromium::launch().await?;
    let page: Page = BrowserWindow::new(&context).await?;

    await page.goto("http://localhost:3000"); // Assuming the app is running on port 3000

    // When
    await page.locator("#AddPanelButton").click();
    await page.locator("text=3D").click();
    await page.locator(".panel-settings-left").click();
    let topic_locator = page.locator(".3D");

    // Then
    assert!(topic_locator.is_visible());

    Ok(())
}
```

Note: This code assumes that the app is running on port 3000 and uses a UI with buttons and elements like `AddPanelButton`, `panel-menu-item 3D`, `.panel-settings-left`, `.3D`. The actual selectors (`#AddPanelButton`, etc.) and their positions in the DOM may vary depending on the app's structure.