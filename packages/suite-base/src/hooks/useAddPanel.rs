```rust
use crate::components::PanelCatalog;
use crate::context::CurrentLayoutContext;
use crate::util::layout::{get_panel_id_for_type};
use crate::data::PanelSelection;

pub fn use_add_panel() -> impl Fn(PanelSelection) {
    let mut add_panel = || ();

    let context = &current_layout_context();

    add_panel = move |selection: PanelSelection| {
        let id = get_panel_id_for_type(&selection.type);
        context.add_panel(id, selection.config);
    };

    return add_panel;
}
```