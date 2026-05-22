```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

type Layout = String;
type SharedLayouts = Vec<Layout>;
type PersonalLayouts = Vec<Layout>;

type LayoutSelectionState = HashMap<String, Layout>;
type Dispatch = dyn Fn(&mut LayoutSelectionAction);

fn selected_layout_id_selector(state: &LayoutState) -> Option<&str> {
    state.get("lastSelectedId")
}

#[derive(Debug, Clone)]
struct UseLayoutNavigation {
    onSelectLayout: fn(Layout, &Option<HashMap<Layout, Layout>>),
    state: LayoutSelectionState,
    dispatch: Dispatch,
}

fn use_layout_navigation(menu_close: Option<Box<dyn Fn()>>) -> UseLayoutNavigation {
    let current_layout_id = use_current_layout_selector(selected_layout_id_selector);
    let layoutManager = use_layout_manager();
    let analytics = use_analytics();
    let { set_selected_layout_id } = use_current_layout_actions();

    let (state, dispatch) = use_layout_browser_reducer(
        HashMap::from([
            ("lastSelectedId", current_layout_id.clone()),
            ("busy", true),
            ("error", None),
            ("online", layoutManager.is_online()),
        ]),
    );

    let (_, layouts) = use_async_fn(
        async () -> (PersonalLayouts, SharedLayouts) {
            let layouts = await layoutManager.get_layouts();
            let (personal, shared) = layouts.into_iter().partition(|layout| layout.supports_sharing());
            (
                personal.iter().map(|&layout| layout).collect(),
                shared.iter().map(|&layout| layout).collect(),
            )
        },
        &[layout_manager],
        true,
    );

    let onSelect_layout = move |item: Layout, params: Option<HashMap<Layout, Layout>>| {
        if let Some(params) = params {
            analytics.log_event(AppEvent::LAYOUT_SELECT, &params);
        }
        if params.is_some() && params.as_ref().unwrap().get("ctrlKey").is_some()
            || params.is_some() && params.as_ref().unwrap().get("metaKey").is_some()
            || params.is_some() && params.as_ref().unwrap().get("shiftKey").is_some()
        {
            if item != current_layout_id {
                // selectedIds is empty on intial render
                // this adds the current layout to selection
                if state.get(&current_layout_id).is_none() {
                    dispatch(LayoutSelectionAction::select_id(current_layout_id.to_string()));
                }
                dispatch(LayoutSelectionAction::select_id(
                    item.to_string(),
                    Some(layouts.clone()),
                    params.as_ref().unwrap()["modKey"].clone().unwrap() as bool,
                    params.as_ref().unwrap()["shiftKey"].clone().unwrap() as bool,
                ));
            }
        } else {
            set_selected_layout_id(item);
            dispatch(LayoutSelectionAction::select_id(item.to_string()));
            menu_close.as_mut().unwrap().call();
        }
    };

    UseLayoutNavigation {
        onSelectLayout,
        state: state.clone(),
        dispatch,
    }
}
```