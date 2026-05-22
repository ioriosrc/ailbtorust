```rust
use crate::suite::current_layout_actions::{update_shared_panel_state};
use crate::suite::current_layout_selector::{select_shared_state};
use crate::suite::layout::get_panel_type_from_id;
use crate::suite_base::util::LayoutState;
use crate::suite_base::components::PanelContext;
use crate::suite_base::context::CurrentLayoutContext;

const EMPTY_SHARED_PANEL_STATE: SharedPanelState = Immutable::new(());

pub fn use_shared_panel_state() -> (Immutable<SharedPanelState>, fn(Immutable<SharedPanelState>) -> ()) {
  let shared_state = use_current_layout_selector(select_shared_state);
  let { update_shared_panel_state } = use_current_layout_actions();

  let panel_id = use_panel_context().id;
  let panel_type = useMemo(get_panel_type_from_id(panel_id), [panel_id]);

  let shared_data = useMemo(() => shared_state[panel_type], [panel_type, shared_state]);

  let update = move |data: Immutable<SharedPanelState>| {
    update_shared_panel_state(panel_type, data);
  };

  (shared_data, update)
}
```