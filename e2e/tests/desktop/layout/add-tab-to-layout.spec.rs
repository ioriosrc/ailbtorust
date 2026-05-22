```rust
use test_case::test_case;

#[tokio::test]
async fn create_new_layout_and_add_tab() {
    // Given
    let main_window = get_main_window().await;
    load_from_file_picker(&main_window, LAYOUT_FILE).await.unwrap();
    await_tab_list_view(&main_window, 1).await.unwrap();

    // Then
    assert_eq!(get_tab_count(&main_window, "toolbar-tab").await.unwrap(), 1);

    // When
    let panel_search = main_window.get_element_by_id("panel-list-textfield").await.unwrap().locator("input");
    await panel_search.fill("tab");
    await click_button_with_text(&main_window, "Tab Group panels together", true).await;
    await click_button_with_text(&main_window, "3d", false).await;

    // Then
    assert_eq!(get_tab_count(&main_window, "toolbar-tab").await.unwrap(), 2);
    assert_eq!(get_tab_count(&main_window, "add-tab").await.unwrap(), 2);

    // When
    let add_tab_button = main_window.get_element_by_role("button", Some(&"add-tab")).await.unwrap();
    await add_tab_button.click();

    // Then
    assert_eq!(get_tab_count(&main_window, "toolbar-tab").await.unwrap(), 3);

    // When
    let tab_icon = main_window.get_element_by_id("tab-icon").await.unwrap().nth(0).await;
    await tab_icon.click();

    // Then
    assert_eq!(get_tab_count(&main_window, "toolbar-tab").await.unwrap(), 0);
}
```