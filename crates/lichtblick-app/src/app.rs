// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;

use crate::components::workspace::Workspace;
use crate::state::app_state::{AppState, provide_app_state};

/// Root application component.
#[component]
pub fn App() -> impl IntoView {
    // Provide global application state
    provide_app_state();

    view! {
        <div class="lichtblick-app" data-theme="dark">
            <Workspace />
        </div>
    }
}
