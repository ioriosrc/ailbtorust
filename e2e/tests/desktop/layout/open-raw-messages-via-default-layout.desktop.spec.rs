```rust
use playwright::{api::Page, browser::BrowserContext, browser_context::ViewportSize};

#[tokio::test]
async fn open_raw_messages_panel_on_layout_selection() {
    let browser = BrowserContext::new().await.unwrap();
    let context = browser.new_context().await.unwrap();

    // Given: Open the main window
    let page = context.new_page().await.unwrap();
    await page.goto("http://localhost:3000").await;

    // GIVEN: Default layout is open
    await page.click("#DataSourceDialog > #CloseIcon").await;
    await page.click("text=Layouts > layout").await;
    await page.click(".layout-list-item[data-id='Default']").await;

    // WHEN: User clicks on the Raw Messages panel settings
    await page.click("#panel-settings-left").await;

    // THEN: No topic selected should be displayed
    let no_topic_selected = await page.text_content("#no-topic-selected > .topic-name").await;
    assert_eq!(no_topic_selected, "No topic selected");

    // Close the browser context and browser
    context.close().await.unwrap();
    browser.close().await.unwrap();
}
```

This Rust test case uses the Playwright library to simulate user interactions with a web application. It opens the main window, navigates to the Layouts page, selects the "Default" layout, clicks on the Raw Messages panel settings, and then checks that "No topic selected" is displayed.