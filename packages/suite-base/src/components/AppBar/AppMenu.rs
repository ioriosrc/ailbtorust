```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use react::prelude::*;
use material_ui::{Menu, MenuItem};
use text_middle_truncate::TextMiddleTruncate;
use {LICHTBLICK_DOCUMENTATION_LINK, format_keyboard_shortcut};

use crate::{
    store::{WorkspaceContextStore, WorkspaceActions},
    hooks::use_layout_transfer,
    types::{AppBarMenuItem, AppMenuProps},
};

#[derive(Clone)]
pub struct AppMenu {
    open: bool,
    handleClose: Callback<(), ()>,
    anchorEl: Option<Rc<HTMLElement>>,
    anchorReference: AnchorReference,
    anchorPosition: AnchorPosition,
    disablePortal: bool,
}

impl AppMenu {
    pub fn new(props: AppMenuProps) -> Self {
        AppMenu {
            open: props.open,
            handleClose: props.handleClose,
            anchorEl: props.anchorEl,
            anchorReference: props.anchorReference,
            anchorPosition: props.anchorPosition,
            disablePortal: props.disablePortal,
        }
    }

    fn file_items() -> Vec<MenuItem> {
        let items: Vec<MenuItem> = vec![
            MenuItem::new()
                .text("Open")
                .key("open")
                .data_test_id("menu-item-open")
                .onClick(move || {
                    // Implementation for opening a data source
                }),
            MenuItem::new()
                .text("Open Local Files")
                .key("open-file")
                .shortcut(format_keyboard_shortcut("O", ["Meta"]))
                .data_test_id("menu-item-open-local-file")
                .onClick(move || {
                    // Implementation for opening local files
                }),
            MenuItem::new()
                .text("Open Connection")
                .key("open-connection")
                .shortcut(format_keyboard_shortcut("O", ["Meta", "Shift"]))
                .data_test_id("menu-item-open-connection")
                .onClick(move || {
                    // Implementation for opening a connection
                }),
            MenuItem::divider(),
            MenuItem::new()
                .text("Recent Data Sources")
                .key("recent-sources")
                .disabled(true), // Placeholder until recent sources are implemented
        ];

        items
    }

    fn view_items() -> Vec<MenuItem> {
        let mut items = vec![
            MenuItem::new()
                .text(if AppState::left_sidebar_open().is_some() {
                    "Hide Left Sidebar"
                } else {
                    "Show Left Sidebar"
                })
                .key("left-sidebar")
                .shortcut("[")
                .onClick(move || {
                    // Implementation for toggling left sidebar open/closed
                }),
            MenuItem::new()
                .text(if AppState::right_sidebar_open().is_some() {
                    "Hide Right Sidebar"
                } else {
                    "Show Right Sidebar"
                })
                .key("right-sidebar")
                .shortcut("]")
                .onClick(move || {
                    // Implementation for toggling right sidebar open/closed
                }),
            MenuItem::divider(),
            MenuItem::new()
                .text("Import Layout from File")
                .key("import-layout"),
            MenuItem::new()
                .text("Export Layout to File")
                .key("export-layout"),
        ];

        items
    }

    fn help_items() -> Vec<MenuItem> {
        let mut items = vec![
            MenuItem::new()
                .text(AppState::about().unwrap_or("About"))
                .key("about")
                .onClick(move || {
                    // Implementation for opening the about dialog
                }),
            MenuItem::divider(),
            MenuItem::new()
                .text(LICHTBLICK_DOCUMENTATION_LINK)
                .key("docs")
                .onClick(move || {
                    // Implementation for opening the documentation link in a new tab
                    std::web_sys::window().unwrap().open(LICHTBLICK_DOCUMENTATION_LINK, "_blank", "noopener,noreferrer");
                }),
            MenuItem::divider(),
            MenuItem::new()
                .text("Explore Sample Data")
                .key("demo")
                .onClick(move || {
                    // Implementation for opening the demo data
                }),
        ];

        items
    }

    fn render(&self) -> JSXElement {
        <Menu
            open={self.open}
            disable_auto_focus_item
            onClose=self.handle_close.clone()
            slot_props={{
                list: {
                    "aria-labelledby": "app-menu-button",
                    dense: true,
                    className: classes.menu_list,
                },
                paper: {
                    "data-tourid": "app-menu",
                } as Partial<PaperProps & { "data-tourid"?: string }>,
            }}
        >
            <NestedMenuItem
                on_pointer_enter={self.handle_item_pointer_enter.clone()}
                items=self.file_items()
                open=self.state.nested_menu == "app-menu-file"
                id="app-menu-file"
            >
                {t("file")}
            </NestedMenuItem>
            <NestedMenuItem
                on_pointer_enter={self.handle_item_pointer_enter.clone()}
                items=self.view_items()
                open=self.state.nested_menu == "app-menu-view"
                id="app-menu-view"
            >
                {t("view")}
            </NestedMenuItem>
            <NestedMenuItem
                on_pointer_enter={self.handle_item_pointer_enter.clone()}
                items=self.help_items()
                open=self.state.nested_menu == "app-menu-help"
                id="app-menu-help"
            >
                {t("help")}
            </NestedMenuItem>
        </>
    }
}

fn main() {
    AppMenu::new(AppMenuProps {
        open: false,
        handleClose: Callback::new(|_| {}),
        anchorEl: None,
        anchorReference: AnchorReference::None,
        anchorPosition: AnchorPosition::None,
        disablePortal: false,
    });
}
```

Note that this is a simplified version and does not include the actual implementation for opening data sources, local files, connections, toggling sidebar open/closed, or handling import/export layout actions. Additionally, the `AppState` and related state management are assumed to be defined elsewhere in the codebase.