```rust
use playwright::error::Error;
use playwright::test::{TestInfo, TestOptions};
use playwright::{expect, BrowserContext, Page};

async fn run_test(context: &BrowserContext, page: &Page) -> Result<(), Error> {
    // Given
    context.navigate("http://localhost:3000").await?;
    await page.click("#DataSourceDialog > #CloseIcon").await?;
    await page.click("#layouts-left").await?;
    await page.click("#layout-list-item").with_text("Default").click().await?;

    // When
    await page.click("#panel-settings-left").await?;
    await page.click(".3D").nth(0).click().await?;

    // Then
    expect(page.locator("#3D panel").count()).to_eq(1);
}

#[test]
async fn test_open_3d_panel_on_layout_click(TestInfo: TestInfo) -> Result<(), Error> {
    BrowserContext::launch_async("path/to/chromium").await?;
    let context = BrowserContext::new().await?;
    let page = context.new_page().await?;
    run_test(&context, &page).await?;
    Ok(())
}
```