```rust
use crate::components::stack::{self as Stack, stack_props};

pub fn Box(children: impl Into<crate::components::stack::StackProps>) -> crate::prelude::Html {
    let theme = use_theme();
    html! {
        <Stack
            alignItems="center"
            justifyContent="center"
            padding={1}
            full_height
            style={{
                text_align: "center",
                border: format!("1px solid {}", theme.divider()),
                background_color: theme.action.hover,
            }}
        >
            {children.into()}
        </Stack>
    }
}

pub fn Default() -> crate::prelude::Html {
    html! {
        <Stack data-testid="padding" gap={2} full_height>
            <Stack direction="row" gap={2}>
                {ITEMS.iter().enumerate().map(|(index, _)| {
                    Stack::new(
                        children: Box::new(format!("Row item {}", index + 1)),
                        flex: "auto",
                    )
                })}
            </Stack>
            <Stack flexGrow={2} justifyContent="space-between" gap={2}>
                {ITEMS.iter().enumerate().map(|(index, _)| {
                    Stack::new(
                        direction: stack_props::Direction::Row,
                        gap: 2,
                        justify_content: stack_props::JustifyContent::FlexStart,
                        children: Box::new(format!("Row item {}", index + 1)),
                    )
                })}
            </Stack>
            <Stack flexGrow={2} justifyContent="space-between" gap={2}>
                {ITEMS.iter().enumerate().map(|(index, _)| {
                    Stack::new(
                        direction: stack_props::Direction::Row,
                        gap: 2,
                        justify_content: stack_props::JustifyContent::Center,
                        children: Box::new(format!("Row item {}", index + 1)),
                    )
                })}
            </Stack>
            <Stack flexGrow={2} justifyContent="space-between" gap={2}>
                {ITEMS.iter().enumerate().map(|(index, _)| {
                    Stack::new(
                        direction: stack_props::Direction::Row,
                        gap: 2,
                        justify_content: stack_props::JustifyContent::FlexEnd,
                        children: Box::new(format!("Row item {}", index + 1)),
                    )
                })}
            </Stack>
            <Stack gap={2} justifyContent="space-between">
                {ITEMS.iter().enumerate().map(|(index, _)| {
                    Stack::new(
                        direction: stack_props::Direction::Column,
                        flex: "auto",
                    )
                })}
            </Stack>
            <Stack flex="auto" gap={2} justify_content="space-between">
                {ITEMS.iter().enumerate().map(|(index, _)| {
                    Stack::new(
                        direction: stack_props::Direction::Column,
                        gap: 2,
                        align_self: stack_props::AlignSelf::FlexStart,
                    )
                })}
            </Stack>
            <Stack flex="auto" gap={2} justify_content="space-between">
                {ITEMS.iter().enumerate().map(|(index, _)| {
                    Stack::new(
                        direction: stack_props::Direction::Column,
                        gap: 2,
                        align_self: stack_props::AlignSelf::Center,
                    )
                })}
            </Stack>
            <Stack flex="auto" gap={2} justify_content="space-between">
                {ITEMS.iter().enumerate().map(|(index, _)| {
                    Stack::new(
                        direction: stack_props::Direction::Column,
                        gap: 2,
                        align_self: stack_props::AlignSelf::FlexEnd,
                    )
                })}
            </Stack>
        </Stack>
    }
}
```