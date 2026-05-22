```rust
// Import necessary Rust libraries
use wasm_bindgen::prelude::*;
use js_sys::JsObject;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    fn window();
}

#[wasm_bindgen_test]
async fn start_component() {
    // GIVEN
    let window = web_sys::window().expect("Expected the global window object");
    let document = window.document().expect("Expected the document");
    let body = document.body().expect("Expected the body element");

    // WHEN
    let element = js_sys::Object::create(window);

    // THEN
    assert!(element.hasOwnProperty("openDataSource"));
    assert!(element.hasOwnProperty("openLocalFiles"));
    assert!(element.hasOwnProperty("openConnection"));
    assert!(element.hasOwnProperty("recentDataSources"));

    let recent_sources: Vec<&str> = vec!["Source 1", "Source 2"];
    for source in &recent_sources {
        let button = element.create_element("button").expect("Expected a button");
        button.set_text_content(source);
        body.append_child(&button).expect("Expected to append the button to the document");

        let click_event: JsObject = js_sys::Object::create(window);

        // WHEN
        button.dispatchEvent(&click_event);

        // THEN
        assert!(mock_log_event.called_with("file"));
    }
}

#[wasm_bindgen_test]
async fn select_recent_source() {
    // GIVEN
    let window = web_sys::window().expect("Expected the global window object");
    let document = window.document().expect("Expected the document");
    let body = document.body().expect("Expected the body element");

    // WHEN
    let element = js_sys::Object::create(window);

    let mock_select_recent = Box::new(|| println!("Source selected: {:?}", "Source 1"));

    (usePlayerSelection as wasm_bindgen::JsFunction).apply(
        Some(&element),
        vec!["Source 1".into()],
        &mock_select_recent,
    );

    // THEN
    assert!(mock_select_recent.called_with("Source 1"));
}
```