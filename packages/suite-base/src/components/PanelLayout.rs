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
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use mui::material::{CircularProgress};
use react::prelude::*;
use react_dnd::{DragType, useDrop};
use serde_json::Value;
use crate::extensions::PanelConfig;
use crate::layout::MosaicNode;
use crate::panel_catalog::PanelCatalogContext;
use crate::extension_catalog::ExtensionCatalogContext;
use crate::extension_catalog::get_panel_id_for_type;
use crate::extension_catalog::get_panel_type_from_id;

fn TabMosaicWrapper(props: PropsWithChildren<{ tabId?: String }>) -> JSXElement {
    let { classes, cx } = useStyles();
    let (drop_target, drop_result) = use_drop({
        accept: DragType::WINDOW,
        drop: move |item, monitor| {
            if monitor.get_drop_result::<MosaicDropResult>().path.is_none() {
                return None;
            }
            Some({
                tab_id: props.tabId,
                ...monitor.get_drop_result::<MosaicDropResult>(),
            })
        },
    });

    let children = props.children;

    <div className={cx(classes.hide_top_level_drop_targets, "mosaic-tile")} ref={drop_target}>
        {children}
    </div>
}

fn UnconnectedPanelLayout(props: PropsWithChildren<{ tabId?: String }>) -> JSXElement {
    let save_panel_configs = use_current_layout_actions().save_panel_configs;
    let mosaic_id = use_panel_mosaic_id();
    let { layout, onChange, tabId, loading_component } = props;

    let create_tile = useCallback(
        |config: Option<PanelConfig> | -> String {
            let default_panel_type = "RosOut";
            let type = config.unwrap_or_default().type;
            let id = get_panel_id_for_type(type);
            if let Some(config) = config {
                save_panel_configs(vec![config]);
            }
            id
        },
        [save_panel_configs],
    );

    let panel_catalog = use_panel_catalog();
    let panel_components: HashMap<String, ComponentType<MosaicNode<String>>> = panel_catalog.panels.into_iter().collect();

    let render_tile = useCallback(
        |id: String, path: MosaicPath| -> JSXElement {
            if id.is_empty() || id == "undefined" {
                return <></>;
            }
            let type = get_panel_type_from_id(id);
            let PanelComponent = panel_components.get(&type).map(|p| p.clone());
            if PanelComponent.is_some() {
                panel_component.map(|PanelComponent| <PanelComponent child_id={id} tab_id={tabId}/>)
            } else {
                <UnknownPanel child_id={id} tab_id={tabId} override_config={{ type, id }}/>
            }
        },
        [panel_components],
    );

    let body_to_render = useMemo(
        || {
            if layout.is_some() {
                let mosaic_without_drag_drop_context: MosaicWithoutDragDropContext<MosaicNode<String>> = MosaicWithoutDragDropContext::new();
                mosaic_without_drag_drop_context.render_tile(render_tile).change_panel_layout(on_change).mosaic_id(mosaic_id)
                    .layout(layout.unwrap()).on_change(change_panel_layout).resize(Some(MosaicResize { minimum_pane_size_percentage: 2.0 })).value(layout)
            } else {
                <EmptyPanelLayout tab_id={tabId}/>
            }
        },
        [layout, mosaic_id, onChange, render_tile, tabId],
    );

    <ErrorBoundary>
        {loading_component}
        {body_to_render}
    </ErrorBoundary>
}

fn ExtensionsLoadingState() -> JSXElement {
    let body = Stack::new().gap(1).align_items("center").children(vec![
        CircularProgress::new().size(28),
        Text::new("Loading extensions…"),
    ]);

    <EmptyState>{body}</EmptyState>
}

const selected_layout_exists_selector = (state: LayoutState) -> bool {
    state.selected_layout.data.is_some()
};
const selected_layout_mosaic_selector = (state: LayoutState) -> Option<MosaicNode<String>> {
    state.selected_layout.data
};

pub fn PanelLayout() -> JSXElement {
    let { classes } = useStyles();
    let layout_empty_state = use_app_context().layout_empty_state;
    let change_panel_layout = use_current_layout_actions().change_panel_layout;
    let layout_exists = use_current_layout_selector(selected_layout_exists_selector);
    let mosaic_layout = use_current_layout_selector(selected_layout_mosaic_selector);
    let registered_extensions = use_extension_catalog((state) => state.installed_extensions);
    let installing_progress = use_installing_extensions_store();

    let is Installing_extensions = installing_progress.in_progress;

    let onChange = useCallback(
        move |new_layout: Option<MosaicNode<String>>| {
            if new_layout.is_some() {
                change_panel_layout(new_layout.unwrap());
            }
        },
        [change_panel_layout],
    );

    if registered_extensions.is_none() {
        return ExtensionsLoadingState();
    }

    let loading_component = is Installing_extensions.then(|| Stack::new().gap(1).align_items("center").children(vec![
        CircularProgress::new().size(28),
        Text::new("Loading extensions…"),
    ])).unwrap_or_else(|| <></>);

    if layout_exists {
        return UnconnectedPanelLayout {
            layout: mosaic_layout,
            onChange,
            loading_component,
        };
    }

    if layout_empty_state {
        return layout_empty_state;
    }

    <></>
}
```