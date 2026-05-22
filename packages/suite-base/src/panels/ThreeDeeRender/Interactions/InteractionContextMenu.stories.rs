```rust
use crate::{ThreeDeeRender, InteractionContextMenu};

#[test]
fn light_interaction_context_menu() {
    let shared_props = ThreeDeeRender::new_shared_props();

    let light_context_menu = InteractionContextMenu::new_light(shared_props.clone());

    assert_eq!(light_context_menu.get_color_scheme(), "light");
}

#[test]
fn dark_interaction_context_menu() {
    let shared_props = ThreeDeeRender::new_shared_props();

    let dark_context_menu = InteractionContextMenu::new_dark(shared_props);

    assert_eq!(dark_context_menu.get_color_scheme(), "dark");
}
```