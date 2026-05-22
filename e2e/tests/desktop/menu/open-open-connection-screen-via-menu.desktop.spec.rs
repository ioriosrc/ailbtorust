```rust
use playwright::{test::TestOptions, context::ContextOptions};

#[test]
async fn display_open_a_new_connection_dialog(
    options: TestOptions,
    context_options: ContextOptions,
) {
    // Given
    let browser = playwright().launch(options);
    let context = browser.new_context(context_options);
    let page = await context.new_page();

    // When
    await page.click("#DataSourceDialog #CloseIcon");
    await page.click("app-menu-file > menu-item-open");
    await page.click("button:has-text('Open connection')");

    // Then
    await page.wait_for_selector(".dialog").await;
}
```