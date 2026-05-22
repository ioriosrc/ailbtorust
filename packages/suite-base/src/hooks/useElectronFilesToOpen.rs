```rust
use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::EventTarget;

#[wasm_bindgen]
pub fn use_electron_files_to_open() -> Rc<FileList> {
    let input: EventTarget = document
        .get_element_by_id("electron-open-file-input")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();

    let file_list: Option<Rc<FileList>> = Some(Rc::new(input.files()));

    let event_target = Rc::from(input);
    let update = move || {
        if let Some(files) = input.files() {
            file_list.replace_with(|| Rc::new(files));
        }
    };

    document.add_event_listener_with_callback("change", update).unwrap();

    file_list
}
```