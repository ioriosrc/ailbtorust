```rust
use crate::{Fixture, PanelSetup};

pub fn example(fixture: &Fixture) -> String {
    format!(
        r#"<div style={{ width: '400px' }}>{}</div>"#,
        <VariableSliderPanel as wasm_bindgen::JsCast>::unchecked_into::<crate::html::Element>(
            <PanelSetup as wasm_bindgen::JsCast>::unchecked_into::<crate::html::Element>(
                fixture
                    .get_element("panel_setup")
                    .expect("Failed to find panel setup element"),
            )
        )
        .append_child(&<VariableSliderPanel as wasm_bindgen::JsCast>::unchecked_into::<crate::html::Element>(
            fixture.get_element("variable_slider").expect("Failed to find variable slider element"),
        ))
    )
}

pub fn narrow_layout(fixture: &Fixture) -> String {
    format!(
        r#"<div style={{ width: '400px' }}>{}</div>"#,
        <VariableSliderPanel as wasm_bindgen::JsCast>::unchecked_into::<crate::html::Element>(
            <PanelSetup as wasm_bindgen::JsCast>::unchecked_into::<crate::html::Element>(
                fixture
                    .get_element("panel_setup")
                    .expect("Failed to find panel setup element"),
            )
        )
        .append_child(&<VariableSliderPanel as wasm_bindgen::JsCast>::unchecked_into::<crate::html::Element>(
            fixture.get_element("variable_slider").expect("Failed to find variable slider element"),
        ))
    )
}

pub fn with_settings(fixture: &Fixture) -> String {
    format!(
        r#"<div style={{ width: '400px' }}>{}</div>"#,
        <VariableSliderPanel as wasm_bindgen::JsCast>::unchecked_into::<crate::html::Element>(
            <PanelSetup as wasm_bindgen::JsCast>::unchecked_into::<crate::html::Element>(
                fixture
                    .get_element("panel_setup")
                    .expect("Failed to find panel setup element"),
            )
        )
        .append_child(&<VariableSliderPanel as wasm_bindgen::JsCast>::unchecked_into::<crate::html::Element>(
            fixture.get_element("variable_slider").expect("Failed to find variable slider element"),
        ))
    )
}
```