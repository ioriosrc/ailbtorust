```rust
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Worker;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    // foxglove-depcheck-used: babel-plugin-transform-import-meta
    let worker = Worker::new("./worker.js")?;
    
    worker.onopen().unwrap().set_callback(move |event| {
        console.log("WebSocket connection opened");
        event.target.ok().unwrap().post_message("open".into());
    });

    worker.onmessage().unwrap().set_callback(move |event| {
        let msg = event.data.into();
        match msg.as_object()?["type"].as_string() {
            Some(type_) => match type_.as_str() {
                "open" => console.log(&msg),
                "close" => {
                    console.log("WebSocket connection closed");
                    worker.terminate();
                },
                "error" => console.log(&msg),
                "message" => {
                    console.log(&msg);
                },
                _ => {}
            },
            None => {}
        }
    });

    Ok(())
}
```