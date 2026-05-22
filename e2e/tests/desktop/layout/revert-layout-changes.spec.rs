```rust
use playwright::{test, expect};
use crate::fixtures::{load_files};

const LAYOUT_FILE = "imported-layout.json";

async fn split_panel(main_window: &playwright::Page, panel_id: &str) -> Result<(), playwright::Error> {
    main_window
        .get_by_test_id(format!("panel-mouseenter-container {panel_id}"))
        .get_by_test_id("panel-menu")
        .click();
    await main_window.wait_for_timeout(1000);
    main_window.get_by_role("menuitem", |i| i.name() == "Split down").click();
    await main_window.wait_for_timeout(1000);
}

#[test]
async fn makes_changes_to_layout_and_then_reverts_them(
    mut browser_context: playwright::Context,
    test_page: &mut playwright::Page
) -> Result<(), Box<dyn std::error::Error>> {
    // Given
    load_files(&browser_context, &[LAYOUT_FILE]).await?;

    // When
    test_page.get_by_test_id("DataSourceDialog").get_by_test_id("CloseIcon").click();
    await test_page.wait_for_timeout(1000);
    test_page.get_by_test_id("layouts-left").click();

    // Then
    let imported_layout = test_page.get_role("button", |i| i.name() == "imported-layout");
    expect(imported_layout).to_have_count(1);

    // When
    await split_panel(&test_page, "3D!18i6zy7").await?;

    // Then
    let unsaved_changes_icon = test_page
        .get_role("listitem")
        .filter(|i| i.has_text("imported-layout"))
        .get_by_test_id("unsaved-changes-icon");
    expect(unsaved_changes_icon).to_be_visible();

    // When
    await unsaved_changes_icon.click();
    await test_page.get_by_role("menuitem", |i| i.name() == "Revert").click();

    // Then
    expect(unsaved_changes_icon).not_to_be_visible();
    Ok(())
}
```