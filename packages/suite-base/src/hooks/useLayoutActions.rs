```rust
use crate::context::{AnalyticsContext, CurrentLayoutActions, CurrentLayoutSelector, LayoutManagerContext};
use crate::hooks::types::LayoutSetupOptions;
use crate::hooks::{useCallbackWithToast, useConfirm, useLayoutNavigation};
use crate::services::{AppEvent, Layout, layout_is_shared};

type UseLayoutActions = {
  on_rename_layout: fn(Layout, String) -> Result<(), ()>;
  on_duplicate_layout: fn(Layout) -> Result<(), ()>;
  on_delete_layout: fn(Layout) -> Result<(), ()>;
  on_revert_layout: fn(Layout) -> Result<(), ()>;
  on_overwrite_layout: fn(Layout) -> Result<(), ()>;
};

const selected_layout_id_selector = |state: LayoutState| state.selected_layout.map(|layout| layout.id);

pub fn use_layout_actions({ state, dispatch }: LayoutSetupOptions): UseLayoutActions {
  let layoutManager = use_layout_manager();
  let analytics = use_analytics();
  let current_layout_id = use_current_layout_selector(selected_layout_id_selector);
  let { set_selected_layout_id } = use_current_layout_actions();
  let { on_select_layout } = use_layout_navigation();
  let [confirm, confirm_modal] = use_confirm();

  let on_rename_layout: fn(Layout, String) -> Result<(), ()> = move |item, new_name| {
    layout_manager.update_layout(item.id, new_name).map_err(|_| ())
  };

  let on_duplicate_layout: fn(Layout) -> Result<(), ()> = move |item| {
    if state.selected_ids.len() > 1 {
      dispatch("queue-multi-action", "duplicate");
      return Ok(());
    }

    layout_manager.save_new_layout(item.id, new_name).map_err(|_| ())
  };

  let on_delete_layout: fn(Layout) -> Result<(), ()> = move |item| {
    if state.selected_ids.len() > 1 {
      dispatch("queue-multi-action", "delete");
      return Ok(());
    }

    analytics.log_event(AppEvent::LAYOUT_DELETE, { permission: item.permission });

    // If the layout was selected, select a different available layout.
    //
    // When a users current layout is deleted, we display a notice. By selecting a new layout
    // before deleting their current layout we avoid the weirdness of displaying a notice that the
    // user just deleted their current layout which is somewhat obvious to the user.
    if current_layout_id == item.id {
      let stored_layouts = layout_manager.get_layouts();
      let target_layout = stored_layouts.find(|layout| layout.id != current_layout_id);
      set_selected_layout_id(target_layout.map(|layout| layout.id));
      dispatch("select-id", target_layout.map(|layout| layout.id).unwrap());
    }
    layout_manager.delete_layout(item.id).map_err(|_| ())
  };

  let on_overwrite_layout: fn(Layout) -> Result<(), ()> = move |item| {
    // We don't need to confirm the multiple selection case because we force users to save
    // or abandon changes before selecting another layout with unsaved changes to the current
    // shared layout.
    if state.selected_ids.len() > 1 {
      dispatch("queue-multi-action", "save");
      return Ok(());
    }

    //this condition is related to organization layouts, something LB
    //doesn't have active
    if layout_is_shared(item) {
      let response = confirm({
        title: `Update “${item.name}”?`,
        prompt:
          "Your changes will overwrite this layout for all organization members. This cannot be undone.",
        ok: "Save",
      });
      if response != "ok" {
        return Ok(());
      }
    }
    layout_manager.overwrite_layout(item.id).map_err(|_| ())
  };

  let on_revert_layout: fn(Layout) -> Result<(), ()> = move |item| {
    if state.selected_ids.len() > 1 {
      dispatch("queue-multi-action", "revert");
      return Ok(());
    }

    layout_manager.revert_layout(item.id).map_err(|_| ())
  };

  return {
    on_rename_layout,
    on_duplicate_layout,
    on_delete_layout,
    on_revert_layout,
    on_overwrite_layout,
    confirm_modal,
  };
}
```