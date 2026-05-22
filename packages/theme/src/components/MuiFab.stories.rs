```rust
use fluentui_rust::prelude::*;
use mui_rsx::prelude::*;

fn main() {
    let mut app = App::new()
        .add_theme(Theme::default())
        .mount("app-root");

    let add_icon = Add20Filled::new();
    let edit_icon = Edit20Filled::new();
    let toolbox_icon = Toolbox20Filled::new();
    let heart_icon = Heart20Filled::new();

    let default_fab = Fab::new()
        .color("primary")
        .aria_label("add")
        .render(|_| view! { add_icon });

    let secondary_fab = Fab::new()
        .color("secondary")
        .aria_label("edit")
        .render(|_| view! { edit_icon });

    let extended_fab = Fab::new()
        .variant("extended")
        .render(|_| view! { toolbox_icon });

    let disabled_fab = Fab::new()
        .disabled(true)
        .aria_label("like")
        .render(|_| view! { heart_icon });

    let mut content_stack = Stack::new();

    content_stack.add(
        default_fab,
        Default::Attributes
            .padding(2.0)
            .gap(2.0)
            .justify_content(StackJustifyColumn::Center)
            .align_items(StackAlignItems::Center),
    );

    content_stack.add(
        secondary_fab,
        Default::Attributes
            .padding(2.0)
            .gap(2.0)
            .justify_content(StackJustifyColumn::Center)
            .align_items(StackAlignItems::Center),
    );

    content_stack.add(
        extended_fab,
        Default::Attributes
            .padding(2.0)
            .gap(2.0)
            .justify_content(StackJustifyColumn::Center)
            .align_items(StackAlignItems::Center),
    );

    content_stack.add(
        disabled_fab,
        Default::Attributes
            .padding(2.0)
            .gap(2.0)
            .justify_content(StackJustifyColumn::Center)
            .align_items(StackAlignItems::Center),
    );

    app.add_child(content_stack);
}
```