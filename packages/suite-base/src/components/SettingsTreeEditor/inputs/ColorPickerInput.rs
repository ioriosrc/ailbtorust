```rust
use materialize::{Button, IconButton, Input, Popover};
use materialize::icon::Cancel;
use materialize::text_field::TextField;

use lichtblick::suite_base::components::Stack;
use lichtblick::theme::{custom_typography};

#[derive(Default)]
struct ColorPickerInputProps<'a> {
    alpha_type: &'a str,
    disabled: bool,
    value: Option<&'a str>,
    onChange: fn(&'a str),
    placeholder: &'a str,
    readOnly: bool,
    hide_clear_button: bool,
}

fn color_picker_input(props: ColorPickerInputProps<'_>) -> impl MaterializeElement {
    let alpha_type = props.alpha_type;
    let disabled = props.disabled;
    let onChange = props.onChange;
    let value = props.value;

    let styles = create_styles(props);

    let (swatch_color, display_value, update_prefixed_color, edited_value_is_invalid, edited_value) =
        use_color_picker_control(alpha_type, onChange, value);
    
    let (anchor_element, set_anchor_element) = State::new::<Option<HtmlElement>>();
    let handleClick = move |event: MouseEvent<HtmlElement>| {
        set_anchor_element(Some(event.target()));
    };
    let handleClose = move || {
        set_anchor_element(None);
    };

    let clear_value = move || {
        onChange("");
    };

    let open = anchor_element.is_some();

    let should_hide_clear_button = (display_value.unwrap_or("") == "") || props.hide_clear_button;

    Stack::with_styles(styles.root, |stack| {
        stack.append(
            TextField::with_styles(styles.text_field, |textField| {
                textField
                    .full_width()
                    .disabled(disabled)
                    .placeholder(props.placeholder)
                    .size("small")
                    .variant("filled")
                    .value(match edited_value.is_some() {
                        Some(_) => format!("#{edited_value.as_ref().unwrap().replace('#', '')}"),
                        None => edited_value.clone(),
                    })
                    .on_key_down(|event| {
                        if event.key == "Enter" {
                            handleClose();
                        }
                    })
                    .on_change(|event| update_edited_value(event.target().value()))
                    .on_blur(|_| on_input_blur())
            }),
        );
        stack.append(
            Popover::with_styles(styles.popover, |popover| {
                popover
                    .open(open)
                    .anchor_el(anchor_element.clone())
                    .on_close(handle_close)
                    .anchor_origin({
                        Vertical::Bottom,
                        Horizontal::Left,
                    })
                    .transform_origin({
                        Vertical::Top,
                        Horizontal::Center,
                    })
                    .append(
                        ColorPickerControl::with_styles(styles.color_picker_control, |picker| {
                            picker
                                .alpha_type(alpha_type)
                                .on_change(on_change)
                                .on_enter_key(handle_close)
                                .swatch_color(swatch_color)
                                .update_prefixed_color(update_prefixed_color)
                                .edited_value_is_invalid(edited_value_is_invalid)
                                .edited_value(edited_value)
                                .update_edited_value(update_edited_value)
                                .on_input_blur(on_input_blur())
                        }),
                    );
            }),
        );
    })
}

fn create_styles(props: ColorPickerInputProps) -> Styles {
    // Implementation of styles creation logic
}
```