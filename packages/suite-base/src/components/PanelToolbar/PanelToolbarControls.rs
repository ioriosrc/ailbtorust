```rust
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::PanelStateStore;
use crate::context::{CurrentLayoutContext, PanelCatalogContext};
use crate::workspace::use_workspace_actions;

#[function_component]
pub fn PanelToolbarControls(props: &PanelToolbarControlsProps) -> Html {
    let additional_icons = props.additional_icons.clone();
    let classes = styles::PanelToolbarControlsStyles().unwrap();

    let panel_context = Context::<PanelContext>::new();
    let panel_id = panel_context.get_panel_id();
    let panel_type = panel_context.get_panel_type();
    let show_logs = panel_context.get_show_logs();
    let set_show_logs = panel_context.set_show_logs;
    let log_count = panel_context.get_log_count();

    let toggle_logs = move || {
        if let Some(set_show_logs) = set_show_logs {
            set_show_logs(!show_logs);
        }
    };

    let panel_catalog = Context::<PanelCatalogContext>::new();
    let selected_panel_ids = use_selected_panels();
    let open_panel_settings = use_workspace_actions().open_panel_settings;

    let has_settings_selector = move |store: &PanelStateStore| {
        store.settings_trees.contains_key(panel_id)
    };

    let panel_info = use_panel_state_store(has_settings_selector);

    let has_settings = use_panel_state_store(move |state| state.has_settings(panel_id));

    let open_settings = move || {
        if let Some(panel_id) = panel_id {
            selected_panel_ids.push(panel_id);
            open_panel_settings();
        }
    };

    let show_settings_button = !panel_info.is_custom_toolbar() || has_settings;

    html! {
        <Stack
            direction="row"
            align_items="center"
            padding_left="1rem"
            ref={props.ref}
            full_height="true"
            padding_top="1rem"
        >
            {additional_icons.clone()}
            <Badge
                color="error"
                variant="dot"
                invisible={log_count == 0}
                class={classes.logs_badge}
            >
                <ToolbarIconButton
                    disabled=log_count == 0
                    title={
                        if show_logs {
                            "Hide logs"
                        } else {
                            format!("Show logs{}", log_count > 0.then(|| format!(" ({})", log_count)).unwrap_or(""))
                        }
                    }
                    onClick=toggle_logs
                >
                    <ListAltIcon color={if show_logs { "primary" } else { None }} />
                </ToolbarIconButton>
            </Badge>
            {show_settings_button && html! {
                <ToolbarIconButton title="Settings" onClick=open_settings>
                    <SettingsIcon />
                </ToolbarIconButton>
            }}
            <PanelActionsDropdown is_unknown_panel={props.is_unknown_panel} />
        </Stack>
    }
}
```