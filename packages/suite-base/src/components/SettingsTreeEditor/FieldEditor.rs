```rust
useiced::widget::{self, Widget};

#[derive(Debug)]
enum InputType {
    Autocomplete,
    Number,
    Toggle,
    String,
    Boolean,
    Rgb,
    Rgba,
    MessagePath,
    Select,
    Gradient,
    Vec3,
    Vec2,
    LegendControls,
    Slider,
}

fn render_input(input_type: &InputType) -> Widget<()> {
    match input_type {
        InputType::Autocomplete => widget::autocomplete!(),
        InputType::Number => widget::number!(),
        InputType::Toggle => widget::toggle_group!(
            vec![widget::toggle("Off"), widget::toggle("On")],
        ),
        InputType::String => widget::text_field!(),
        InputType::Boolean => widget::toggle_group!(
            vec![widget::toggle(false), widget::toggle(true)],
        ),
        InputType::Rgb | InputType::Rgba => widget::color_picker_input!(),
        InputType::MessagePath => widget::message_path_input!(),
        InputType::Select => widget::select!(),
        InputType::Gradient => widget::color_gradient_input!(),
        InputType::Vec3 => widget::vec3_input!(),
        InputType::Vec2 => widget::vec2_input!(),
        InputType::LegendControls => widget::legend_controls!(),
        InputType::Slider => widget::slider!(),
    }
}

fn render_label(field: & Immutable<SettingsTreeField>) -> Widget<()> {
    match field.input {
        InputType::Vec2 | InputType::Vec3 => widget::column![
            widget::text_field!(
                title = field.label,
                variant = "subtitle2",
                color = "text.secondary",
                no_wrap = true,
                flex = "auto",
            ),
            field.labels.map(|label| widget::text_field!(
                key = label,
                title = field.label,
                variant = "subtitle2",
                color = "text.secondary",
                no_wrap = true,
                style = { grid_column: if label == &field.label { 1 } else { 2 } / span 1 },
                flex = "auto",
            )),
        ],
    }
}

fn render_field(field: &Immutable<SettingsTreeField>) -> Widget<()> {
    let render_input_widget = |input_type| {
        widget::column![
            render_label(field),
            render_input(input_type),
        ]
    };

    match field.input {
        InputType::Autocomplete => render_input_widget(InputType::Autocomplete),
        InputType::Number => render_input_widget(InputType::Number),
        InputType::Toggle => render_input_widget(InputType::Toggle),
        InputType::String => render_input_widget(InputType::String),
        InputType::Boolean => render_input_widget(InputType::Boolean),
        InputType::Rgb | InputType::Rgba => render_input_widget(InputType::Rgb),
        InputType::MessagePath => render_input_widget(InputType::MessagePath),
        InputType::Select => render_input_widget(InputType::Select),
        InputType::Gradient => render_input_widget(InputType::Gradient),
        InputType::Vec3 => render_input_widget(InputType::Vec3),
        InputType::Vec2 => render_input_widget(InputType::Vec2),
        InputType::LegendControls => render_input_widget(InputType::LegendControls),
        InputType::Slider => render_input_widget(InputType::Slider),
    }
}

fn FieldEditorComponent(field: &Immutable<SettingsTreeField>) -> Widget<()> {
    let indent = field.path.len().min(4);
    let paddingLeft = 0.75 + 2 * (indent - 1);

    widget::stack![
        widget::row![
            field.error.map(|error| widget::tooltip!(
                arrow = true,
                placement = "top",
                title = error,
            )),
            render_label(field),
        ],
        widget::tooltip!(
            arrow = true,
            placement = "right",
            title = field.tooltip,
        ),
        widget::column![
            render_field(field),
            widget::padding_bottom(0.25),
        ],
    ]
}

pub fn FieldEditor(field: &Immutable<SettingsTreeField>) -> Widget<()> {
    useiced::widget::{self, Widget};

    let indent = field.path.len().min(4);
    let paddingLeft = 0.75 + 2 * (indent - 1);

    widget::stack![
        widget::row![
            field.error.map(|error| widget::tooltip!(
                arrow = true,
                placement = "top",
                title = error,
            )),
            render_label(field),
        ],
        widget::tooltip!(
            arrow = true,
            placement = "right",
            title = field.tooltip,
        ),
        widget::column![
            render_field(field),
            widget::padding_bottom(0.25),
        ],
    ]
}
```