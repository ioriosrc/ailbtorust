```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub type HUDItem = (
    String, // Unique identifier for the item
    String, // Designate what group this belongs to.
             // Allows items to be cleared by group.
             // Would allow scene extensions to only clear their own items when applicable.
    fn() -> String, // Function to return message content to show on HUD
    &str,          // Display type
);

pub const HUD_ID_PRIORITIES: [&str; 8] = [
    BOTH_TOPICS_DOES_NOT_EXIST_HUD_ITEM_ID,
    IMAGE_TOPIC_DOES_NOT_EXIST_HUD_ITEM_ID,
    CALIBRATION_TOPIC_DOES_NOT_EXIST_HUD_ITEM_ID,
    WAITING_FOR_BOTH_MESSAGES_HUD_ID,
    WAITING_FOR_IMAGES_EMPTY_HUD_ID,
    WAITING_FOR_CALIBRATION_HUD_ID,
    WAITING_FOR_SYNC_EMPTY_HUD_ID,
];

pub struct HUDItemManager {
    items: HashMap<String, HUDItem>,
}

impl HUDItemManager {
    pub fn new(on_change: impl Fn()) -> Self {
        HUDItemManager {
            items: HashMap::new(),
            on_change,
        }
    }

    pub fn add_hud_item(&mut self, item: HUDItem) {
        if !self.items.contains_key(&item.0) {
            self.items.insert(item.0.clone(), item);
            (self.on_change)();
        }
    }

    pub fn remove_hud_item(&mut self, id: &str) {
        if let Some(_removed_item) = self.items.remove(id) {
            (self.on_change)();
        }
    }

    pub fn remove_group(&mut self, group: &str) {
        for item in self.items.iter_mut() {
            if item.1.1 == group {
                self.remove_hud_item(item.0);
            }
        }
    }

    // eslint-disable-next-line @lichtblick/no-boolean-parameters
    pub fn display_if_true(&mut self, value: bool, hud_item: HUDItem) {
        if value {
            self.add_hud_item(hud_item);
        } else {
            self.remove_hud_item(&hud_item.0);
        }
    }

    /** Returns list of HUD items in ascending priority order.
     * High priority items will be last in the list.
     */
    pub fn get_hud_items(&self) -> Vec<HUDItem> {
        // sort by priority on return
        // high priority items should be at the end of the list
        let mut sorted_items: Vec<_> = self.items.iter().cloned().collect();
        sorted_items.sort_by(|a, b| HUD_ID_PRIORITIES.iter().position(|&p| p == a.0).unwrap() - HUD_ID_PRIORITIES.iter().position(|&p| p == b.0).unwrap());
        sorted_items
    }

    pub fn clear(&mut self) {
        self.items.clear();
        (self.on_change)();
    }
}
```