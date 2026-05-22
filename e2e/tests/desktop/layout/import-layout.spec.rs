```rust
use test::{test, expect};
use playwright::page::Page;
use playwright::fixtures::load_from_file_picker;

const LAYOUT_FILE = "imported-layout.json";

#[test]
async fn import_layout_via_layout_tab() {
    // Given
    let page = Page::new().await?;
    await page.navigate("file://./test/fixtures/index.html").await?;

    // When
    await page.click("#DataSourceDialog > #CloseIcon").await?;
    await page.click("#layouts-left").await?;
    await load_from_file_picker(page, LAYOUT_FILE).await?;

    // Then
    let layout_item = page.get_text("layout-list-item", "imported-layout").await?;
    expect(layout_item).to_be("imported-layout");
}

#[test]
async fn import_layout_via_menu() {
    // Given
    let page = Page::new().await?;
    await page.navigate("file://./test/fixtures/index.html").await?;

    // When
    await page.click("#DataSourceDialog > #CloseIcon").await?;
    await page.click("#layouts-left").await?;
    await load_from_file_picker(page, LAYOUT_FILE).await?;

    // Then
    let layout_item = page.get_text("layout-list-item", "imported-layout").await?;
    expect(layout_item).to_be("imported-layout");
}
```