```rust
// Import necessary Rust dependencies
use wasm_bindgen::prelude::*;

// Define the HighlightedValue component as a Rust struct
#[wasm_bindgen]
struct HighlightedValue {
    item_label: String,
}

// Implement methods for the HighlightedValue struct to handle rendering logic
impl HighlightedValue {
    // Constructor to initialize the HighlightedValue with an item label
    pub fn new(item_label: &str) -> Self {
        HighlightedValue { item_label: item_label.to_string() }
    }

    // Render method to create the HTML element based on the item label
    #[wasm_bindgen(js_name = render)]
    pub fn render(&self) -> WebAssemblyResult<()> {
        let mut document = web_sys::window().expect("Failed to access window");
        let body = document.get_element_by_id("body")?.unwrap();

        // Create a span element for the item label
        let span = document.create_element("span").ok_or(web_sys::Error::new("Failed to create span element"))?;

        // Set the text content of the span with the item label
        span.set_text_content(&self.item_label);

        // Append the span element to the body of the HTML document
        body.append_child(&span)?;
        Ok(())
    }
}

// Define a custom attribute for the HighlightedValue component
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(getter, js_name = maybeCollapsed)]
    fn get_maybe_collapsed(element: &HighlightedValue) -> WebAssemblyResult<JsValue>;
}

// Main function to run the tests using wasm-bindgen-test
#[wasm_bindgen_test]
async fn test_highlighted_value() {
    // Create an instance of HighlightedValue
    let highlighted_value = HighlightedValue::new("example");

    // Render the component
    highlighted_value.render().unwrap();

    // Get the span element from the document
    let span = get_maybe_collapsed(&highlighted_value).unwrap().dyn_ref::<web_sys::HtmlSpanElement>().expect("Failed to get span element");

    // Check if the text content is correct
    assert_eq!(span.text_content(), "example");
}
```