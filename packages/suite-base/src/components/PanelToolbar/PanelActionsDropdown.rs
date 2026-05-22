```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2018-2021 Cruise LLC

use crate::components::{PanelContext, PanelToolbar};
use crate::context::{CurrentLayoutActions, MosaicWindowActions};
use crate::hooks::{get_panel_type_from_mosaic, use_current_layout_actions};
use crate::state::{MosaicNode, WindowPath};

#[derive(Debug)]
struct PanelActionsDropdownProps {
    is_unknown_panel: bool,
}

impl PanelActionsDropdownProps {
    pub fn new(is_unknown_panel: bool) -> Self {
        Self { is_unknown_panel }
    }
}

fn panel_actions_dropdown_component(props: PanelActionsDropdownProps) -> impl 'static + Clone + 'static {
    let (menu_anchor_el, set_menu_anchor_el) = use_state(Option::<Element>::None);
    let (sub_menu_anchor_el, set_sub_menu_anchor_el) = use_state(Option::<Element>::None);

    let is_touch_interaction = useRef(false);

    let { t } = use_translation("panelToolbar");

    let menu_open = props.is_unknown_panel || match &menu_anchor_el {
        Some(_) => true,
        None => false,
    };
    let submenu_open = props.is_unknown_panel || match &sub_menu_anchor_el {
        Some(_) => true,
        None => false,
    };

    let panel_context = useContext(PanelContext);
    let tab_id = panel_context?.tab_id;
    let { mosaic_actions } = useContext(MosaicContext);
    let { mosaic_window_actions } = useContext(MosaicWindowActions);

    let get_panel_type = use_callback(
        move || {
            let type = get_panel_type_from_mosaic(mosaic_window_actions, mosaic_actions);
            if type.is_none() {
                panic!("Trying to split unknown panel!");
            }
            type
        },
        [mosaic_actions, mosaic_window_actions],
    );
    let handle_touch_start = use_callback(move || {
        is_touch_interaction.current = true;
    }, []);

    let handle_menu_click = move |event: MouseEvent<HtmlElement>| {
        set_sub_menu_anchor_el(None);
        set_menu_anchor_el(event.currentTarget);
    };

    let handle_menu_close = move |_| {
        set_sub_menu_anchor_el(None);
        set_menu_anchor_el(None);
    };

    let handle_submenu_click = move |event: MouseEvent<HtmlElement>| {
        if sub_menu_anchor_el != event.currentTarget {
            set_sub_menu_anchor_el(event.currentTarget);
        }
        if !is_touch_interaction.current {
            set_menu_anchor_el(None);
        }
    };

    let handle_submenu_close = move |_| {
        set_sub_menu_anchor_el(None);
    };

    let handle_submenu_mouse_enter = move |event: MouseEvent<HtmlElement>| {
        set_sub_menu_anchor_el(event.currentTarget);
    };

    let close_panel = use_callback(
        move |id: Option<&str>, root: &MosaicNode<String>, path: WindowPath| {
            mosaic_window_actions.close_window(tab_id, id, root, path);
        },
        [mosaic_window_actions],
    );

    let split_panel = use_callback(
        move |id: &str, direction: &str, root: &MosaicNode<String>, path: WindowPath, config: &serde_json::Value| {
            mosaic_window_actions.split_window(tab_id, id, direction, root, path, config);
        },
        [mosaic_window_actions],
    );

    let enter_fullscreen = use_callback(move || {
        panel_context?.enter_fullscreen();
    }, [panel_context]);

    let menu_items = useMemo(|| {
        let mut items: Vec<MenuItem> = Vec::new();

        if !props.is_unknown_panel {
            items.push(MenuItem::with_text("Split Right".to_string()));
            items.push(MenuItem::with_text("Split Down".to_string()));
        }

        if panel_context?.is_fullscreen != true {
            items.push(
                MenuItem::with_text("Fullscreen".to_string())
                    .with_icon(Icons::ArrowRight)
                    .with_on_click(enter_fullscreen),
            );
        }

        items.push(MenuItem::Divider);

        items.push({
            let id = &panel_context?.id;
            MenuItem::with_text("Remove Panel".to_string())
                .with_icon(Icons::Cross)
                .with_on_click(close_panel)
                .with_data_test_id("panel-menu-remove")
                .with_class_name(ClassName::Error)
        });

        items
    }, [
        props.is_unknown_panel,
        close_panel,
        enter_fullscreen,
        panel_context?.id,
        panel_context?.is_fullscreen,
        split_panel,
        t,
    ]);

    let button_ref = use_state(None);
    let type = get_panel_type();

    if type.is_none() {
        return None;
    }

    Some(PanelActionsDropdownComponent {
        menu_anchor_el,
        set_menu_anchor_el,
        sub_menu_anchor_el,
        set_sub_menu_anchor_el,
        is_touch_interaction,
        t,
        menu_open,
        submenu_open,
        panel_context,
        tab_id,
        mosaic_actions,
        mosaic_window_actions,
        get_panel_type,
        handle_touch_start,
        handle_menu_click,
        handle_menu_close,
        handle_submenu_click,
        handle_submenu_close,
        handle_submenu_mouse_enter,
        close_panel,
        split_panel,
        enter_fullscreen,
        menu_items,
        button_ref,
    })
}
```