```rust
use crate::components::{ArrowDropDownIcon, Button, ButtonGroup, Stack};
use crate::themes::color_scheme::ColorScheme;
use crate::utils::storybook::*;

#[derive(Clone, Debug)]
pub struct ButtonGroupProps {
    children: Vec<Button>,
    variant: ButtonProps['variant],
    size: ButtonProps['size],
    color: ButtonProps['color],
    disable_elevation: bool,
}

impl ButtonGroupProps {
    pub fn new(children: Vec<Button>, variant: ButtonProps['variant'], size: ButtonProps['size'], color: ButtonProps['color'], disable_elevation: bool) -> Self {
        Self { children, variant, size, color, disable_elevation }
    }
}

#[derive(Clone, Debug)]
pub struct ButtonGroupStory {
    props: ButtonGroupProps,
}

impl StoryObj for ButtonGroupStory {
    fn new() -> Self {
        ButtonGroupStory::default()
    }

    fn render(&self) -> impl Node {
        Stack {
            padding: Some(2.0),
            gap: Some(2.0),
            align_items: Some(AxisAlignment::Center),
            children: vec![
                self.props.children.clone().into_iter().collect(),
            ],
        }
    }
}

fn variants() -> Vec<ButtonProps['variant']> {
    vec!["text", "outlined", "contained"]
}

fn sizes() -> Vec<ButtonProps['size']> {
    vec!["small", "medium", "large"]
}

fn colors() -> Vec<ButtonProps['color']> {
    vec![
        "inherit",
        "primary",
        "secondary",
        "success",
        "error",
        "info",
        "warning",
    ]
}

#[derive(Default)]
pub struct ButtonGroupComponent;

impl Component for ButtonGroupComponent {
    type Props = ButtonGroupProps;

    fn render(&self, props: &ButtonGroupProps) -> impl Node {
        let children = props.children.clone().into_iter().collect();

        ButtonGroup {
            children,
            variant: props.variant,
            size: props.size,
            color: props.color,
            disable_elevation: props.disable_elevation,
        }
    }
}

#[derive(Default)]
pub struct ButtonComponent;

impl Component for ButtonComponent {
    type Props = ButtonProps;

    fn render(&self, props: &ButtonProps) -> impl Node {
        Button {
            variant: props.variant,
            size: props.size,
            color: props.color,
            label: props.label.clone(),
            children: None,
            disabled: false,
            start_icon: None,
            end_icon: None,
            loading_state: None,
            start_icon_position: Default::default(),
            end_icon_position: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct ArrowDropDownIconComponent;

impl Component for ArrowDropDownIconComponent {
    type Props = ();

    fn render(&self) -> impl Node {
        <ArrowDropDownIcon />
    }
}
```