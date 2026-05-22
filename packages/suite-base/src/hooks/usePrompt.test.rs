```rust
use wasm_bindgen::prelude::*;
use web_sys::{document, HtmlInputElement};

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console::log!("App running!");
    Ok(())
}

async fn prompt(title: &str, placeholder: &str, initial_value: &str) -> Option<String> {
    let mut input = InputElement::new().await?;
    input.set_title(title);
    input.set_placeholder(placeholder);
    input.set_value(initial_value);

    document().body().unwrap().append_child(&input).unwrap();

    while true {
        if let Some(value) = input.value().await? {
            return Some(value.trim().to_string());
        }
    }

    unimplemented!()
}

#[wasm_bindgen]
pub async fn use_prompt(title: &str, placeholder: &str, initial_value: &str) -> Option<String> {
    prompt(title, placeholder, initial_value).await
}
```