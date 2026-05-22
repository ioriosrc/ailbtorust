```rust
use web_sys::Blob;
use web_sys::HtmlCanvasElement;
use web_sys::ImageData;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    fn decode_image(image: JsValue, options: Object) -> JsValue;
}

struct WorkerImageDecoder;

impl WorkerImageDecoder {
    pub async fn decode(&self, image: ImageData, options: &JsObject) -> JsValue {
        let blob = blob_from_data(&image);
        decode_image(blob, options)
    }

    fn blob_from_data(image: &ImageData) -> Blob {
        let canvas = create_canvas_from_data(image);
        let ctx = canvas.get_context("2d").expect("Failed to get 2D context");
        let image_data = ctx
            .save()
            .expect("Failed to save canvas state")
            .draw_image_with_html_element(&image, 0.0, 0.0)
            .expect("Failed to draw image on canvas")
            .get imageData(0, 0)
            .expect("Failed to get image data");
        ctx.restore().expect("Failed to restore canvas state");

        let blob = js_sys::Blob::new_with_u8array_and_options(image_data.data(), Some(&[image_data.width() as u32, image_data.height() as u32]));
        blob
    }

    fn create_canvas_from_data(image: &ImageData) -> HtmlCanvasElement {
        let canvas = document().create_element("canvas").expect("Failed to create canvas element");
        canvas.set_width(image.width());
        canvas.set_height(image.height());
        canvas
    }
}

#[wasm_bindgen(start)]
pub async fn main() {
    let decoder = WorkerImageDecoder;
    let image_data = ImageData::new_with_u8array_and_options(vec![255, 0, 0, 255], 10, 10);
    let options = js_sys::JsValue::undefined();
    let result = decoder.decode(image_data, &options).await.expect("Failed to decode image");
    // Handle the decoded data here
}
```