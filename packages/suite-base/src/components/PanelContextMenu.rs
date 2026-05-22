```rust
use std::prelude::*;
use react_hooks::{use_ref, use_state};
use react_divide::Divider;
use react_menu::Menu;
use react_item::MenuItem;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use immutable::Immutable;

/**
 * Types of items that can be included in a context menu. Either a clickable item
 * or a divider.
 */
pub type PanelContextMenuItem =
  | {
      /** Type of selectable menu items. */
      type: "item";

      /** True if the item should be shown but disabled. */
      disabled?: bool;

      /** Label shown for the menu item. */
      label: String;

      /** Callback triggered by clicking the item. */
      onclick: fn();
    }
  | {
      /** Type of item dividers. */
      type: "divider";
    };

type PanelContextMenuProps = {
  /** @returns List of menu items */
  get_items: () => Immutable<PanelContextMenuItem[]>;
};

/**
 * This is a convenience component for attaching a context menu to a panel. It
 * must be a child of a Panel component to work.
 */
fn PanelContextMenuComponent(props: PanelContextMenuProps): React.JSX.Element {
  const { get_items } = props;

  let root_ref = use_ref<HtmlElement>(null);
  let [position, setPosition] = use_state<Option<{ x: number; y: number }>>();

  let handleClose = useCallback(() => {
    setPosition(None);
  }, []);

  let [items, set_items] = use_state<Vec<Immutable<PanelContextMenuItem>>>(([], () => get_items()));

  useEffect(() => {
    if (!root_ref.current) {
      return;
    }

    // Trigger the menu when the right mouse button is released, but not if the mouse moved in
    // between press & release
    let right_click_state: "none" | "down" | "canceled" = "none";
    let handle_mouse_up = (event: MouseEvent) => {
      if (event.button === 2 && right_click_state === "down") {
        setPosition({ x: event.clientX, y: event.clientY });
        set_items(get_items());
        right_click_state = "none";
      }
    };
    let handle_mouse_move = (_event: MouseEvent) => {
      right_click_state = "canceled";
    };
    let handle_mouse_down = (event: MouseEvent) => {
      if (event.button === 2) {
        right_click_state = "down";
      }
    };
    let handle_context_menu = (event: MouseEvent) => {
      event.preventDefault();
    };

    root_ref.current.addEventListener("mousedown", handle_mouse_down);
    root_ref.current.addEventListener("mousemove", handle_mouse_move);
    root_ref.current.addEventListener("mouseup", handle_mouse_up);
    root_ref.current.addEventListener("contextmenu", handle_context_menu);
    return () => {
      root_ref.current.removeEventListener("mousedown", handle_mouse_down);
      root_ref.current.removeEventListener("mousemove", handle_mouse_move);
      root_ref.current.removeEventListener("mouseup", handle_mouse_up);
      root_ref.current.removeEventListener("contextmenu", handle_context_menu);
    };
  }, [get_items]);

  return (
    <div
      ref={root_ref}
      onContextMenu={(event) => {
        event.preventDefault();
      }}
    >
      <Menu
        open={position != None}
        onClose={handleClose}
        anchor_reference="anchorPosition"
        anchor_position={position.map(|pos| pos.clone())}
        slotProps={{
          list: {
            dense: true,
          },
        }}
      >
        {items.iter().map(|item| match item.get() {
          PanelContextMenuItem::Item { label, onclick } => (
            <MenuItem
              onClick={() => {
                handleClose();
                onclick();
              }}
              key={label}
              disabled={item.get().disabled.unwrap_or_default()}
            >
              {label}
            </MenuItem>
          ),
          PanelContextMenuItem::Divider => <Divider variant="middle" key={`divider_${items.len()}`} />,
        })}
      </Menu>
    </div>
  );
}

pub const PanelContextMenu = React.memo(PanelContextMenuComponent);
```