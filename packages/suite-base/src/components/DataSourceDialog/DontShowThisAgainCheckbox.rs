```rust
use web_sys::ElementRef;

use crate::{
    components::Stack,
    hooks::{use_app_configuration_value, use_translation},
};

const DONT_SHOW_THIS_AGAIN_CHECKBOX_ID = "dont-show-this-again-checkbox";

#[allow(non_snake_case)]
pub fn DontShowThisAgainCheckbox(
    set_checked: &web_sys::EventTarget,
) -> ElementRef<HtmlDivElement> {
    let (checked, set_checked_inner) = use_app_configuration_value(bool);

    let handleChange = async () => {
        set_checked_inner(!checked);
    };

    let label_component = create_label(t!("dontShowThisAgain"));

    Stack::new(vec![
        FormControlLabel {
            label: label_component,
            control: Checkbox::new(
                false,
                Some(set_checked),
                Some(handle_change),
            ),
        },
    ])
}

fn create_label(text: &'static str) -> ElementRef<HtmlSpan> {
    ElementRef::from_node(web_sys::window().unwrap().create_element("span").unwrap())
}
```