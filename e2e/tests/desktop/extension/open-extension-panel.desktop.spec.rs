```rust
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

#[wasm_bindgen]
pub async fn open_extension_panel() {
    // Given
    let filename = "lichtblick.suite-extension-turtlesim-0.0.1.foxe";
    load_files(filename).await;

    // When
    document().get_by_id("DataSourceDialog").unwrap().click();
    document().get_element_by_label("Add panel button").unwrap().click();
    document().get_text_content_of_element("#Turtle [local]").unwrap();

    // Then
    let count = document().get_elements_with_text_content("#Turtle", "Turtle");
    assert_eq!(count, 1);
}
```