// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Extension system for Lichtblick Rust.
//!
//! Supports two extension formats:
//! - **Legacy (.foxe)**: ZIP archives from Lichtblick Node.js (contains dist/extension.js + package.json)
//! - **Native (.lbext)**: Future Rust-native extensions (WASM modules)
//!
//! Legacy extensions are executed via JS eval in a sandboxed context.
//! Native extensions will be loaded as WASM components (future).

pub mod types;
pub mod storage;
pub mod loader;
pub mod manager;
