```rust
use std::error::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn initialize(args: JsValue) -> Result<JsValue, Box<dyn Error>> {
    let args = js_sys::Object::try_from(&args).map_err(|e| e.into())?;

    if args.has_property("file") {
        let file_path = args.get("file").expect("Expected file property");
        let source = BagIterableSource::new_with_file(file_path);
        Ok(source.to_js_value())
    } else if args.has_property("url") {
        let url = args.get("url").expect("Expected url property");
        let source = BagIterableSource::new_with_url(url);
        Ok(source.to_js_value())
    } else {
        Err("file or url required".into())
    }
}

#[wasm_bindgen]
pub fn to_string(value: JsValue) -> String {
    value.as_str().unwrap_or_default()
}
```