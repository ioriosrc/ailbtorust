```rust
use wasm_bindgen::prelude::*;
use web_sys::{Element, ElementRef, HtmlDocument};

#[wasm_bindgen(start)]
fn main() {
    console_log!("Hello from Rust!");
}

#[wasm_bindgen]
pub fn render_slider(node: &ElementRef) -> WebError {
    let document = document().expect("Could not get document");
    let slider = document
        .create_element("div")
        .unwrap()
        .set_class_name("info-track");
    node.set_inner_html(&slider.innerHTML);

    let div = document.create_element("div").unwrap();
    div.set_class_name("custom-range");
    div.set_attribute("style", &format!("width: {}%", 50));
    slider.append_child(&div).expect("Could not append child");

    let marker = document.create_element("div").unwrap();
    marker.set_class_name("custom-marker");
    marker.set_attribute("style", &format!("left: {}%", 25));
    slider.append_child(&marker).expect("Could not append child");

    Ok(())
}
```