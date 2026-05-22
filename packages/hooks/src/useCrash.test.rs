```rust
use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    use_crash();
    Ok(())
}

extern "C" {
    fn use_crash() -> *const u8;
}

#[derive(Debug, PartialEq)]
struct Error {
    message: String,
}

impl From<JsValue> for Error {
    fn from(error: JsValue) -> Self {
        let string = error.as_string().unwrap();
        Error { message }
    }
}

fn use_crash() -> Result<(), Box<dyn std::error::Error>> {
    act!(async move {
        let result = Promise::new(|_, reject| {
            reject.rejected_with(JsValue::from("my error"));
        })
        .await?;
        Err(Error::from(result))
    })
}
```