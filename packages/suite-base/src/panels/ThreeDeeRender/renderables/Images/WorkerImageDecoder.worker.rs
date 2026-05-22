```rust
use web_sys::ImageData;
use wasm_bindgen::prelude::*;
use futures::TryStreamExt;

#[wasm_bindgen]
pub fn decode(image: JsValue, options: JsValue) -> Result<ImageData, JsValue> {
    let image = image.into_serde::<RosImage>().unwrap();
    let options: RawImageOptions = options.into_serde::<RawImageOptions>().unwrap();

    let data = image.decode(&options)?;
    Ok(ImageData::from_rgb8(
        data.width as u32,
        data.height as u32,
        &data.bytes,
    ))
}
```