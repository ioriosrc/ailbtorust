```rust
use web_sys::{Element, EventTarget};

// Assuming these types are defined elsewhere in your Rust codebase
struct Layout;
struct LayoutID(String);
struct LayoutRow {
    layout: Layout,
    selected: bool,
}
type Layouts = Vec<Layout>;

#[derive(Default)]
struct Props<'a> {
    title: &'a str,
    empty_text: &'a str,
    items: &'a [Layout],
    any_selected_modified_layouts: bool,
    multi_selected_ids: Vec<&'a str>,
    selected_id: Option<&'a str>,
    onSelect: fn(&'a Layout),
    onRename: fn(&'a LayoutID, &'a str),
    onDuplicate: fn(&'a LayoutID),
    onDelete: fn(&'a LayoutID),
    onShare: fn(&'a LayoutID),
    onExport: fn(&'a LayoutID),
    onOverwrite: fn(&'a LayoutID),
    onRevert: fn(&'a LayoutID),
    onMakePersonalCopy: fn(&'a LayoutID),
}

#[wasm_bindgen]
extern "C" {
    #[js_function]
    fn render_element(element: Element, props: Props<'static>) -> *mut WebAssembly.Memory;
}

#[wasm_bindgen]
extern "C" {
    #[js_function]
    fn screen_get_by_text(element: Element, text: &str) -> Option<*mut WebAssembly.Memory>;
}

#[wasm_bindgen]
extern "C" {
    #[js_function]
    fn screen_query_by_test_id(element: Element, test_id: &str) -> Option<*mut WebAssembly.Memory>;
}

// Usage example in Rust
fn main() {
    let title = "Sample Title";
    let empty_text = "Add a new layout to get started with Lichtblick!";
    let sample_layouts: [Layout; 3] = [
        Layout { id: LayoutID(String::from("1")), name: String::from("Layout One") },
        Layout { id: LayoutID(String::from("2")), name: String::from("Layout Two") },
        Layout { id: LayoutID(String::from("3")), name: String::from("Layout Three") },
    ];

    let props = Props {
        title,
        empty_text,
        items: &sample_layouts,
        any_selected_modified_layouts: false,
        multi_selected_ids: vec!["2".to_string()],
        selected_id: Some("2".to_string()),
        onSelect: |_| {},
        onRename: |_| {},
        onDuplicate: |_| {},
        onDelete: |_| {},
        onShare: |_| {},
        onExport: |_| {},
        onOverwrite: |_| {},
        onRevert: |_| {},
        onMakePersonalCopy: |_| {},
    };

    let element = Element::new().unwrap(); // Replace with actual DOM element
    render_element(element, props);
}
```