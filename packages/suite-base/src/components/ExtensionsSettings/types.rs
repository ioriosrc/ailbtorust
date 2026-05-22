```rust
use std::sync::{Arc, Mutex};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use mui::ButtonProps;
use asyncronous::{AsyncState, Result};
use immutable::Immutable;
use suite_base::context::ExtensionMarketplaceContext;

pub type InstalledExtension = {
  id: String;
  installed: bool;
  name: String;
  displayName: String;
  description: String;
  publisher: String;
  homepage: Option<String>;
  license: Option<String>;
  version: String;
  keywords: Vec<&str>;
  namespace: String;
  qualifiedName: String;
};

pub type FocusedExtension = {
  installed: bool;
  entry: Immutable<ExtensionMarketplaceDetail>;
};

pub type EntryGroupedData = {
  namespace: String;
  entries: Vec<Immutable<ExtensionMarketplaceDetail>>;
};

pub type UseExtensionSettingsHook = Arc<Mutex<
  (
    fn(newFilterText: String) -> (),
    AsyncState<Vec<ExtensionMarketplaceDetail>>,
    fn() -> Result<Vec<ExtensionMarketplaceDetail>>,
    String,
    Vec<EntryGroupedData>,
    Vec<EntryGroupedData>,
    String,
  ),
>>;

pub type UseExtensionOperationsOptions = {
  onInstallSuccess?: Box<dyn Fn(String) + Send + Sync>;
  onUninstallSuccess?: Box<dyn Fn(String) + Send + Sync>;
};

pub type UseExtensionOperationsReturnHook = Arc<Mutex<
  (
    fn(Immutable<ExtensionMarketplaceDetail>) -> Result<(), ()>,
    fn(Immutable<ExtensionMarketplaceDetail>) -> Result<(), ()>,
    OperationStatus,
    Option<String>,
    bool,
  ),
>>;

pub type ExtensionActionButtonProps = {
  extension: Immutable<ExtensionMarketplaceDetail>;
  onAction: Box<dyn Fn(Immutable<ExtensionMarketplaceDetail>) + Send + Sync>;
  isOperating: bool;
  operationStatus: OperationStatus;
  className?: String;
  stopPropagation?: bool;
  color?: ButtonProps["color"];
  variant?: ButtonProps["variant"];
  label: String;
  loadingLabel: String;
};

pub const ExtensionActionsLabel = {
  INSTALL: "Install",
  UNINSTALL: "Uninstall",
};

pub const ExtensionOperationStatusLabel = {
  INSTALLING: "Installing...",
  UNINSTALLING: "Uninstalling...",
};
```