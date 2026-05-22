```rust
use ::anyhow::Result;
use ::web_sys::{HtmlElement, KeyboardEvent};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn document_create_element(element_name: &str) -> *mut HtmlElement;
}

#[wasm_bindgen]
pub struct ColorPickerControl {
    hex: String,
    edited_value: String,
    swatch_color: String,
    alpha_type: bool, // true for alpha
    on_change: Closure<dyn Fn(&str)>,
}

impl ColorPickerControl {
    #[wasm_bindgen(constructor)]
    pub fn new(
        alpha_type: bool,
        value: Option<&str>,
        onchange: Closure<dyn Fn(&str)>>,
    ) -> Result<Self> {
        let hex = match value {
            Some(value) => tinycolor::Rgb { r: 0, g: 0, b: 0 }.set_alpha_u8(value.parse::<u8>()?).to_hex()?,
            None => "#00000044".to_string(),
        };
        let edited_value = hex.clone();
        let swatch_color = hex;
        let on_change = onchange.into_js_value();

        Ok(ColorPickerControl {
            hex,
            edited_value,
            swatch_color,
            alpha_type,
            on_change,
        })
    }

    pub fn render(&self) -> WebAssemblyResult<()> {
        let mut div = document_create_element("div")?;
        div.set_class_name("color-picker-control");

        if self.alpha_type {
            let color_picker = ColorPicker::new(
                "alpha",
                Some(&self.hex),
                Closure::wrap(Box::new(move |_| {
                    self.edited_value = tinycolor(self.hex.clone()).set_alpha_u8(255).to_hex()?;

                    // Dispatch the change event
                    self.on_change.invoke(&self.hex);
                })),
            )?;
            div.append_child(&color_picker.render())?;
        } else {
            let color_picker = ColorPicker::new(
                "none",
                Some(&self.hex),
                Closure::wrap(Box::new(move |_| {
                    self.edited_value = tinycolor(self.hex.clone()).to_hex()?;

                    // Dispatch the change event
                    self.on_change.invoke(&self.hex);
                })),
            )?;
            div.append_child(&color_picker.render())?;
        }

        let text_field = TextField::new(
            "small",
            self.edited_value.clone(),
            Closure::wrap(Box::new(move |_| {
                if !is_valid_hex_color(self.edited_value.as_str(), self.alpha_type) {
                    return;
                }
                tinycolor(self.edited_value.clone()).set_alpha_u8(255).to_hex_string();
                // Dispatch the change event
                self.on_change.invoke(&self.hex);
            })),
        )?;
        div.append_child(&text_field.render())?;

        Ok(())
    }

    fn is_valid_hex_color(color: &str, alpha_type: bool) -> bool {
        let hex = match color.parse() {
            Ok(hex) => hex,
            Err(_) => return false,
        };
        if hex.len() == 7 || (alpha_type && hex.len() == 8) {
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, new_hex: &str, alpha_type: bool) -> Result<()> {
        let hex = tinycolor::Rgb { r: 0, g: 0, b: 0 }.set_alpha_u8(new_hex.parse::<u8>()?).to_hex()?;
        self.hex = hex.clone();
        if alpha_type {
            let color_picker = ColorPicker::new(
                "alpha",
                Some(&self.hex),
                Closure::wrap(Box::new(move |_| {
                    self.edited_value = tinycolor(self.hex.clone()).set_alpha_u8(255).to_hex()?;

                    // Dispatch the change event
                    self.on_change.invoke(&self.hex);
                })),
            )?;
            self.swatch_color = color_picker.swatch_color();
        } else {
            let color_picker = ColorPicker::new(
                "none",
                Some(&self.hex),
                Closure::wrap(Box::new(move |_| {
                    self.edited_value = tinycolor(self.hex.clone()).to_hex()?;

                    // Dispatch the change event
                    self.on_change.invoke(&self.hex);
                })),
            )?;
            self.swatch_color = color_picker.swatch_color();
        }
        Ok(())
    }

    pub fn on_input_blur(&mut self) {
        self.edited_value = self.hex.clone();
    }
}

#[wasm_bindgen]
pub struct ColorPicker {
    hex: String,
    swatch_color: String,
}

impl ColorPicker {
    #[wasm_bindgen(constructor)]
    pub fn new(alpha_type: &str, value: Option<&str>, onchange: Closure<dyn Fn(&str)>>) -> Result<Self> {
        let hex = match value {
            Some(value) => tinycolor::Rgb { r: 0, g: 0, b: 0 }.set_alpha_u8(value.parse::<u8>()?).to_hex()?,
            None => "#00000044".to_string(),
        };
        let swatch_color = hex.clone();
        let onchange = onchange.into_js_value();

        Ok(ColorPicker { hex, swatch_color })
    }

    pub fn render(&self) -> WebAssemblyResult<()> {
        let div = document_create_element("div")?;
        div.set_class_name("color-picker");

        if alpha_type == "alpha" {
            let color_picker = ColorPickerAlpha::new(
                Some(&self.hex),
                Closure::wrap(Box::new(move |_| {
                    self.swatch_color = tinycolor(self.hex.clone()).set_alpha_u8(255).to_hex()?;

                    // Dispatch the change event
                    onchange.invoke(&self.hex);
                })),
            )?;
            div.append_child(&color_picker.render())?;
        } else {
            let color_picker = ColorPickerNone::new(
                Some(&self.hex),
                Closure::wrap(Box::new(move |_| {
                    self.swatch_color = tinycolor(self.hex.clone()).to_hex()?;

                    // Dispatch the change event
                    onchange.invoke(&self.hex);
                })),
            )?;
            div.append_child(&color_picker.render())?;
        }

        Ok(())
    }
}

struct ColorPickerAlpha {
    hex: String,
    swatch_color: String,
}

impl ColorPickerAlpha {
    #[wasm_bindgen(constructor)]
    pub fn new(value: Option<&str>, onchange: Closure<dyn Fn(&str)>>) -> Result<Self> {
        let hex = match value {
            Some(value) => tinycolor::Rgb { r: 0, g: 0, b: 0 }.set_alpha_u8(value.parse::<u8>()?).to_hex()?,
            None => "#00000044".to_string(),
        };
        let swatch_color = hex.clone();
        let onchange = onchange.into_js_value();

        Ok(ColorPickerAlpha { hex, swatch_color })
    }

    pub fn render(&self) -> WebAssemblyResult<()> {
        let div = document_create_element("div")?;
        div.set_class_name("color-picker-alpha");

        let input = HtmlElement::from_node(div)?;
        input.set_attribute("type", "text")?;
        input.set_attribute("placeholder", "RRGGBBAA")?;

        input.add_event_listener_with_callback(
            "focus",
            Closure::wrap(Box::new(move |event: Event| {
                event.target().set_selection_range(0, 9);
            })),
            true,
        )?;
        input.add_event_listener_with_callback(
            "input",
            Closure::wrap(Box::new(move |event: InputEvent| {
                let target = event.target();
                if !target.is_text_area_element() {
                    return;
                }

                let value = target.value().trim_start_matches('#').to_string();
                if is_valid_hex_color(&value, true) {
                    self.hex = tinycolor(value).set_alpha_u8(255).to_hex()?;
                    self.swatch_color = tinycolor(self.hex.clone()).to_hex()?;

                    // Dispatch the change event
                    onchange.invoke(&self.hex);
                }
            })),
            true,
        )?;
        input.add_event_listener_with_callback(
            "keydown",
            Closure::wrap(Box::new(move |event: KeyboardEvent| {
                if event.key() == "Enter" {
                    let target = event.target();
                    if !target.is_text_area_element() {
                        return;
                    }

                    let value = target.value().trim_start_matches('#').to_string();
                    if is_valid_hex_color(&value, true) {
                        self.hex = tinycolor(value).set_alpha_u8(255).to_hex()?;
                        self.swatch_color = tinycolor(self.hex.clone()).to_hex()?;

                        // Dispatch the change event
                        onchange.invoke(&self.hex);
                    }
                }
            })),
            true,
        )?;
        input.add_event_listener_with_callback(
            "blur",
            Closure::wrap(Box::new(move |_| {
                self.hex = self.swatch_color.clone();
            })),
            true,
        )?;

        Ok(())
    }
}

struct ColorPickerNone {
    hex: String,
    swatch_color: String,
}

impl ColorPickerNone {
    #[wasm_bindgen(constructor)]
    pub fn new(value: Option<&str>, onchange: Closure<dyn Fn(&str)>>) -> Result<Self> {
        let hex = match value {
            Some(value) => tinycolor::Rgb { r: 0, g: 0, b: 0 }.set_alpha_u8(value.parse::<u8>()?).to_hex()?,
            None => "#00000044".to_string(),
        };
        let swatch_color = hex.clone();
        let onchange = onchange.into_js_value();

        Ok(ColorPickerNone { hex, swatch_color })
    }

    pub fn render(&self) -> WebAssemblyResult<()> {
        let div = document_create_element("div")?;
        div.set_class_name("color-picker-none");

        let input = HtmlElement::from_node(div)?;
        input.set_attribute("type", "text")?;

        input.add_event_listener_with_callback(
            "focus",
            Closure::wrap(Box::new(move |event: Event| {
                event.target().set_selection_range(0, 9);
            })),
            true,
        )?;
        input.add_event_listener_with_callback(
            "input",
            Closure::wrap(Box::new(move |event: InputEvent| {
                let target = event.target();
                if !target.is_text_area_element() {
                    return;
                }

                let value = target.value().trim_start_matches('#').to_string();
                if is_valid_hex_color(&value, false) {
                    self.hex = tinycolor(value).set_alpha_u8(255).to_hex()?;
                    self.swatch_color = tinycolor(self.hex.clone()).to_hex()?;

                    // Dispatch the change event
                    onchange.invoke(&self.hex);
                }
            })),
            true,
        )?;
        input.add_event_listener_with_callback(
            "keydown",
            Closure::wrap(Box::new(move |event: KeyboardEvent| {
                if event.key() == "Enter" {
                    let target = event.target();
                    if !target.is_text_area_element() {
                        return;
                    }

                    let value = target.value().trim_start_matches('#').to_string();
                    if is_valid_hex_color(&value, false) {
                        self.hex = tinycolor(value).set_alpha_u8(255).to_hex()?;
                        self.swatch_color = tinycolor(self.hex.clone()).to_hex()?;

                        // Dispatch the change event
                        onchange.invoke(&self.hex);
                    }
                }
            })),
            true,
        )?;
        input.add_event_listener_with_callback(
            "blur",
            Closure::wrap(Box::new(move |_| {
                self.hex = self.swatch_color.clone();
            })),
            true,
        )?;

        Ok(())
    }
}
```

This Rust code defines a `ColorPicker` struct that represents a color picker with different modes (alpha and none). The `ColorPickerAlpha` and `ColorPickerNone` structs are used to render the color picker in the appropriate mode. The `is_valid_hex_color` function is used to validate the input hex color. The `update` method updates the color picker based on user input, and the `on_input_blur` method updates the color picker when it loses focus. The `render` method generates the HTML for the color picker and sets up event listeners to handle user interactions.