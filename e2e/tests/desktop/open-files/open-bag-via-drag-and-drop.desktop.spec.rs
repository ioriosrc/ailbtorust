```rust
use electron::{app::BrowserWindow, test::assert_eq};
use load_files::load_files;

#[test]
async fn should_open_a_bag_file_via_drag_and_drop() {
    // Given
    let filename = "example.bag";
    let browser_window = BrowserWindow::new().unwrap();
    let mut load_files_result = load_files(&browser_window, &vec![filename]);

    assert!(load_files_result.is_ok());

    // Then
    let source_title = browser_window.get_text_by_label(filename).await.unwrap();
    let play_button = browser_window.get_element_by_role("button", "Play").await.unwrap();

    assert_eq!(source_title, Some(filename));
    assert!(play_button.is_enabled());
}
```