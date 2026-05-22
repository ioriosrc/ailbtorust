```rust
use playwright::{BrowserContext, BrowserType, ElementHandle, Locator, Page, Response, WaitForTimeoutOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let browser_type = BrowserType::chrome();
    let context = browser_type.new_context().await?;
    let mut page = context.new_page().await?;

    // Load the file
    let filename = "demo-shuffled.bag";
    page.goto(format!("file://{}", filename)).await?;

    // Toggle topic visibility
    page.locator("#panel-settings-left").click().await?;
    page.locator("button:has-text('3D')").click().await?;
    let visibility_buttons = page.locator(".toggle-visibility");

    // Select only visibles
    page.locator("#menu-").get_text("List visible").await?.parse::<i64>()?;
    visibility_buttons.click().await?;

    // Select only invisibles
    page.locator("#menu-").get_text("List invisible").await?.parse::<i64>()?;
    visibility_buttons.click().await?;

    // Select all
    page.locator("#menu-").get_text("List all").await?.parse::<i64>()?;
    visibility_buttons.click().await?;

    Ok(())
}
```

Note: This Rust code uses the `playwright` crate to interact with a Chromium browser. The file loading and visibility toggling operations are similar to those in the TypeScript/React test, but implemented using Rust's async/await syntax and the playwright API.