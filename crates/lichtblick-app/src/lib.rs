// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Lichtblick Web Application - Rust/WASM Implementation
//!
//! This is the main entry point for the Lichtblick web application compiled to WebAssembly.

pub mod app;
pub mod components;
pub mod decoder;
pub mod mcap_reader;
pub mod panels;
pub mod player;
pub mod state;
pub mod hooks;

use wasm_bindgen::prelude::*;

/// WASM entry point - mounts the Leptos application.
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize logging at Info level (avoids mcap crate DEBUG spam)
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    log::info!("Lichtblick v{} starting...", env!("CARGO_PKG_VERSION"));

    // Mount the Leptos app
    leptos::mount::mount_to_body(app::App);
}
