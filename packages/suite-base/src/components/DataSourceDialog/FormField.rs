```rust
use wasm_bindgen::prelude::*;
use crate::components::FormFieldProps;

#[wasm_bindgen]
pub fn form_field(props: FormFieldProps) -> JsValue {
    let error = None;
    let field = props.field;

    let mut set_error = move |message: &str| {
        error = Some(message.to_string());
        props.on_error(message.to_string());
    };

    let on_change = move |event: web_sys::EventTarget| {
        let target = event.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
        if let Ok(value) = target.value() {
            if field.validate(&value).is_err() {
                set_error("Invalid input");
                return;
            }
            props.on_change(value);
        }
    };

    JsValue::from_serde(&serde_json!({
        "full_width": true,
        "disabled": props.disabled,
        "key": field.label.to_string(),
        "label": field.label.to_string(),
        "error": error.is_some(),
        "helper_text": error,
        "slot_props": {
            "form_helper_text": {
                "variant": "standard",
            },
            "input": {
                "notched": false,
            },
            "input_label": {
                "shrink": true,
            },
        },
        "placeholder": field.placeholder.to_string(),
        "default_value": field.default_value.to_string(),
        "on_change": on_change,
        "variant": "outlined",
    }))
}
```