```rust
use testbed::{app::App, window::Window};
use playwright::{api::ElementHandle, browser::BrowserContext};

#[tokio::test]
async fn create_new_layout_by_accessing_layouts_create_new_layout() {
    let app = App::launch().await;
    let context = app.new_context().await;
    let main_window = context.main_window().await;

    // Given
    await main_window.get_element(&Selector::by_id("DataSourceDialog")).get_element(&Selector::by_id("CloseIcon")).click();
    await main_window.get_element(&Selector::by_text("layouts-left", true)).click();

    // When
    let layout_list_item = context.wait_until_selector(&Selector::by_text("Default", true), None).await;
    await layout_list_item.click();
    await main_window.get_element(&Selector::by_text("Create new layout")).click();
    await main_window.get_element(&Selector::by_text("panel-grid-card Diagnostics – Detail (ROS)", true)).click();

    // Then
    let unnamed_layout = context.wait_until_selector(&Selector::by_text("Unnamed layout", true), None).await;
    assert!(unnamed_layout.text_content().await.contains("Unnamed layout"));
}
```