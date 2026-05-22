```rust
use std::marker::PhantomData;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use lichtblick_suite_base::context::current_layout_context as ctx;
use lichtblick_suite_base::models::{LayoutData, LayoutConfig};
use lichtblick_suite_base::state::{MigrateOptional};

#[derive(PartialEq)]
struct LegacyImagePanel {
    // Define the structure of LegacyImagePanel
}

#[derive(PartialEq)]
struct Legacy3DPanel {
    // Define the structure of Legacy3DPanel
}

fn migrate_legacy_to_new_image_panels<T: LayoutConfig>(data: &mut T) {
    // Implement the migration logic for LegacyImagePanels
    // Example:
    if let Some(mut image_panels) = data.image_panels_mut() {
        for panel in image_panels.iter_mut() {
            // Perform migration on each panel
            // Example:
            panel.migrate();
        }
    }
}

fn migrate_legacy_to_new_3d_panels<T: LayoutConfig>(data: &mut T) {
    // Implement the migration logic for Legacy3DPanels
    // Example:
    if let Some(mut 3d_panels) = data.3d_panels_mut() {
        for panel in 3d_panels.iter_mut() {
            // Perform migration on each panel
            // Example:
            panel.migrate();
        }
    }
}

pub fn migrate_panels_state(data: MigrateOptional<LayoutData, PhantomData>) -> LayoutData {
    let mut result = data.unwrap();

    migrate_legacy_to_new_image_panels(&mut result);
    migrate_legacy_to_new_3d_panels(&mut result);

    result
}
```