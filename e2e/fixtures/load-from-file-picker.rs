```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn load_from_file_picker(main_window: &web_sys::Page, filenames: String) {
    let files = vec![filenames];
    let file_infos = files.iter().map(|filename| {
        let filename = filename.clone();
        file::File::new_with_bytes_and_name(
            web_sys::Blob::from_str(&read_to_string(format!("assets/{}", filename)).await.unwrap()),
            &filename,
        )
    });

    main_window.evaluate_with_callback(async |window| async move {
        let mock_file_handles: Vec<web_sys::FileSystemFileHandle> = file_infos.map(|file_info| {
            web_sys::FileSystemFileHandle::new_with_object_and_type(file_info, "application/json")
                .unwrap()
        }).collect();

        window
            .global_obj::<js_sys::Window>()
            .as_ref()
            .unwrap()
            .get_or_insert(&"showOpenFilePicker", &|| {
                Box::new(move || mock_file_handles) as _ as js_sys::Function
            });
    }, |window| async move {
        window
            .global_obj::<js_sys::Window>()
            .as_ref()
            .unwrap()
            .get("showOpenFilePicker")
            .map(|f| f.call0(&web_sys::JsValue::UNDEFINED))
            .await;
    })
}
```