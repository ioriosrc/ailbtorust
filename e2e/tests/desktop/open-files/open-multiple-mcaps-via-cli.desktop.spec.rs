```rust
use test::{assert_eq, assert_true};
use web_sys::Element;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
async fn should_open_a_file_passed_with_flag_source_via_cli() {
    // Then
    let source_title: Element = document().get_element_by_text(`${mcap_one}, ${mcap_two}`).expect("Source title not found");
    assert_eq!(source_title.inner_text(), `${mcap_one}, ${mcap_two}`);

    // And the "Play" button enabled
    let play_button: Element = document().get_element_by_role("button", { name: "Play", exact: true }).expect("Play button not found");
    assert_true(play_button.is_enabled());
}
```