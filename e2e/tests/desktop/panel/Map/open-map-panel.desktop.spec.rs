```rust
use async_test::prelude::*;

#[tokio::test]
async fn open_map_panel_after_loading_a_bag_file() {
    let filename = "example.bag";
    let window = load_files(&filename).await.unwrap();

    await window.get_by_text("AddPanelButton").click();
    await window.get_by_text("panel-menu-item Map").click();
    await window.get_by_text("panel-settings-left").click();
    await window.wait_for_element_with_text("Waiting for first GPS point...").nth(0);

    assert_eq!(window.get_elements_with_text("Map panel", true).len(), 1);
}
```