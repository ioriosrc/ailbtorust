```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use lichtblick::den::image::{CameraModelsMap, CameraState};
use lichtblick::hooks::use_crash;
use lichtblick::suite_base::components::panel_extension_adapter::BuiltinPanelExtensionContext;
use lichtblick::suite_base::components::forwarded_analytics::ForwardedAnalytics;
use lichtblick::suite_base::panels::three_dee_render::IRenderer;
use lichtblick::suite_base::panels::three_dee_render::SceneExtensionConfig;
use lichtblick::suite_base::panels::three_dee_render::{FollowMode, TestOptions};

pub type InterfaceMode = "3d" | "image";

pub type Shared3DPanelState = {
  camera_state: CameraState,
  follow_mode: FollowMode,
  follow_tf: Option<String>,
};

pub type ThreeDeeRenderProps = {
  context: BuiltinPanelExtensionContext;
  interface_mode: InterfaceMode;
  test_options: TestOptions;
  /** Allow for injection or overriding of default extensions by custom extensions */
  custom_scene_extensions: Option<HashMap<String, SceneExtensionConfig>>;
  custom_camera_models: CameraModelsMap;
  /** Allow for accessing the parent toast snackbar from the new sync root */
  enqueue_snackbar_from_parent: Option<fn(String, &str) -> ()>;
  /** Allow for logging errors to panel logs */
  log_error: Option<fn(&str, Option<&dyn std::error::Error>) -> ()>;
};

pub type InitPanelArgs = {
  crash: Box<dyn use_crash::UseCrash>;
  forwarded_analytics: ForwardedAnalytics;
  interface_mode: InterfaceMode;
  test_options: TestOptions;
  custom_scene_extensions: Option<HashMap<String, SceneExtensionConfig>>;
  custom_camera_models: CameraModelsMap;
  enqueue_snackbar_from_parent: Option<fn(String, &str) -> ()>;
  log_error: Option<fn(&str, Option<&dyn std::error::Error>) -> ()>;
};
```