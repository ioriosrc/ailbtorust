```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn console_log(str: *const u8, len: usize);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use crate::{DontShowThisAgainCheckbox, set_config_value};

    set_config_value(true);

    render_to_root(
        <DontShowThisAgainCheckbox />,
        document.body(),
    );

    Ok(())
}

#[wasm_bindgen(module = "src/js/DontShowThisAgainCheckbox.js")]
extern "C" {
    fn initialize_component();
}

#[wasm_bindgen(module = "src/js/config.js")]
extern "C" {
    fn set_config_value(config: bool);
}

pub struct DontShowThisAgainCheckbox;

impl Component for DontShowThisAgainCheckbox {
    type Props = ();

    fn render(&self, props: Self::Props) -> HtmlElement {
        html! {
            <div class="flex items-center justify-between p-4">
                <p>Dont show this again on startup</p>
                <input
                    type="checkbox"
                    checked={config_value()}
                    onchange={|event: Event| set_config_value(event.target.checked)}
                />
            </div>
        }
    }
}
```

Note:
- The code has been refactored to use Rust's web assembly bindings (`wasm_bindgen`) for interacting with the DOM and web API.
- The `render_to_root` function is assumed to be a custom function that renders a React component into the specified DOM element.
- The `config_value` and `set_config_value` functions are assumed to be defined elsewhere in the codebase.