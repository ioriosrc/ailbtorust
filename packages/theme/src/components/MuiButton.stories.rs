```rust
use crate::prelude::*;

#[component]
fn DefaultButton() -> impl Into<impl AnyElement> {
    Stack {
        direction: Direction::Row,
        padding: Padding::PaddingType::Value(2),
        gap: Gap::GapType::Value(1),
        justify_content: JustifyContentType::Center,
        align_items: AlignItemsType::Center,
    } {}
}

#[component]
fn TextButton() -> impl Into<impl AnyElement> {
    Stack {
        direction: Direction::Row,
        padding: Padding::PaddingType::Value(2),
        gap: Gap::GapType::Value(1),
        justify_content: JustifyContentType::Center,
        align_items: AlignItemsType::Center,
    } {}
}

#[component]
fn ContainedButton() -> impl Into<impl AnyElement> {
    Stack {
        direction: Direction::Row,
        padding: Padding::PaddingType::Value(2),
        gap: Gap::GapType::Value(1),
        justify_content: JustifyContentType::Center,
        align_items: AlignItemsType::Center,
    } {}
}

#[component]
fn DisableElevationButton() -> impl Into<impl AnyElement> {
    Button {
        variant: Variant::Contained,
        disabled: true,
        children: "Disable Elevation",
    }
}

#[component]
fn OutlinedButton() -> impl Into<impl AnyElement> {
    Button {
        variant: Variant::Outlined,
        disabled: true,
        href: Some(String::from("#outlined-buttons")),
        target: Target::Self,
    }
}

#[component]
fn ColorButton(variant: Variant, color: &str) -> impl Into<impl AnyElement> {
    Button {
        variant,
        color,
        children: color.to_string(),
    }
}

#[component]
fn SizesButton(size: Size, variant: Variant) -> impl Into<impl AnyElement> {
    Button {
        size,
        variant,
        children: size.to_string(),
    }
}

#[component]
fn ButtonsWithIcons() -> impl Into<impl AnyElement> {
    Stack {
        direction: Direction::Row,
        padding: Padding::PaddingType::Value(2),
        gap: Gap::GapType::Value(1),
        justify_content: JustifyContentType::Center,
        align_items: AlignItemsType::Center,
    } {}
}

#[component]
fn DeleteIconButton() -> impl Into<impl AnyElement> {
    Button {
        variant: Variant::Outlined,
        start_icon: Some(Icons::Delete20Regular),
        children: "Delete",
    }
}

#[component]
fn SendIconButton() -> impl Into<impl AnyElement> {
    Button {
        variant: Variant::Contained,
        end_icon: Some(Icons::Send20Filled),
        href: Some(String::from("#send-icons")),
        target: Target::Self,
    }
}
```