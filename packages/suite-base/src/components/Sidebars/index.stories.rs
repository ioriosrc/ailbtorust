```rust
use dnd::Backend;
use dnd_html5_backend::Html5Backend;
use react::dom::render_with_props;
use react::html5::DndProvider;

use lichtblick_suite_base::AppSetting;
use lichtblick_suite_base::components::{SidebarItem, Sidebars};
use lichtblick_suite_base::hooks::use_app_configuration_value;

#[derive(Default)]
struct State {
    selected_key: Option<String>,
}

fn main() {
    let mut state = State::default();

    let items = [
        ("a", "A", "Add"),
        ("c", "C", "Cancel"),
        ("d", "D", "Delete"),
        ("e", "E", { count: 2, icon_name: "Edit" }),
    ];

    let bottom_items = [("b", "B", "ErrorBadge")];

    render_with_props! {
        <DndProvider backend=Html5Backend>
            <div style={{ height }}>
                <Sidebars
                    items={items.into_iter().map(|(key, title, icon_name)| SidebarItem { key, title, component: move || A(), iconName }).collect::<Vec<_>>()}
                    bottom_items={bottom_items.into_iter().map(|(key, title, icon_name)| SidebarItem { key, title, component: move || B(), iconName }).collect::<Vec<_>>()}
                    right_items=Vec::new()
                    left_items=Vec::new()
                    selected_key=&state.selected_key
                    onSelect_key=move |key| {
                        state.selected_key = Some(key);
                    }
                    selected_right_key=Option::<&str>::default()
                    onSelect_right_key=move |_| {}
                    selected_left_key=Option::<&str>::default()
                    onSelect_left_key=move |_| {}
                    left_sidebar_size=Option::default()
                    right_sidebar_size=Option::default()
                    set_left_sidebar_size=move |_| {}
                    set_right_sidebar_size=move |_| {}
                >
                    Main content
                </Sidebars>
            </div>
        </DndProvider>,
    };

    // Additional Rust code to handle state management, event handling, etc.
}
```

Note: This is a simplified version of the original TypeScript/React code. The actual implementation would include more complex logic for handling the state, events, and backend integration.