```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use crate::common::{CLIFlags, Desktop};
use crate::extensions::{DesktopExtensionLoader, ExtensionLoaders};
use crate::layouts::{DesktopLayoutLoader, LayoutLoaders};
use crate::services::{
  NativeAppMenu, NativeWindow, OsContext, Storage, WindowState,
};

type RootProps = {
  appParameters: CLIFlags;
  appConfiguration: Rc<crate::common::AppConfiguration>;
  extraProviders: Vec<Rc<dyn std::fmt::Display>>;
  dataSources: Option<Vec<Box<dyn crate::data_source::DataSourceFactory>>>;
};

pub fn Root(props: RootProps) -> Rc<dyn std::fmt::Display> {
  if !props.appConfiguration.storage_bridge.is_some() {
    panic!("storageBridge is missing");
  }

  let mut extension_loaders = ExtensionLoaders::new();
  extension_loaders.push(Box::new(DesktopExtensionLoader::new(&props.appConfiguration.desktop_bridge)));
  extension_loaders.push(Box::new(NativeAppMenu::new(&props.appConfiguration.menu_bridge)));

  let layout_loaders = LayoutLoaders::new();
  layout_loaders.push(Box::new(DesktopLayoutLoader::new(&props.appConfiguration.desktop_bridge)));

  let data_sources: Vec<Box<dyn crate::data_source::DataSourceFactory>> = props
    .dataSources
    .unwrap_or_else(|| {
      let sources = vec![
        Box::new(FoxgloveWebSocketDataSourceFactory {}),
        Box::new(RosbridgeDataSourceFactory {}),
        Box::new(Ros1SocketDataSourceFactory {}),
        Box::new(Ros1LocalBagDataSourceFactory {}),
        Box::new(Ros2LocalBagDataSourceFactory {}),
        Box::new(UlogLocalDataSourceFactory {}),
        Box::new(VelodyneDataSourceFactory {}),
        Box::new(SampleNuscenesDataSourceFactory {}),
        Box::new(McapLocalDataSourceFactory {}),
        Box::new(RemoteDataSourceFactory {}),
      ];
      sources
    });

  let native_app_menu = Rc::new(NativeAppMenu::new(&props.appConfiguration.menu_bridge));
  let native_window = Rc::new(NativeWindow::new(&props.appConfiguration.desktop_bridge));

  let deep_links: Vec<String> = props
    .appConfiguration
    .deep_links
    .unwrap_or_else(|| {
      desktop_window().get_deep_links()
        .iter()
        .map(|url| url.to_string())
        .collect()
    });

  let mut is_full_screen = false;
  let mut is_maximized = native_window().is_maximized();

  let on_minimize_window = Rc::new(move || {
    native_window().minimize();
  });
  let on_maximize_window = Rc::new(move || {
    native_window().maximize();
  });
  let on_unmaximize_window = Rc::new(move || {
    native_window().unmaximize();
  });
  let onClose_window = Rc::new(move || {
    native_window().close();
  });

  let app_state = Rc::new(WindowState {
    full_screen: is_full_screen,
    maximized: is_maximized,
  });

  use crate::common::ipc;
  use wasm_bindgen::{prelude::*, JsCast};

  let on_full_screen_event = &crate::common::ipc::on_i32_event(|event| {
    if event == 10 { // enter-full-screen
      app_state.write().unwrap().full_screen = true;
      set_full_screen(true);
    } else if event == 11 { // leave-full-screen
      app_state.write().unwrap().full_screen = false;
      set_full_screen(false);
    }
  });

  let on_maximize_event = &crate::common::ipc::on_i32_event(|event| {
    if event == 101 { // maximize
      app_state.write().unwrap().maximized = true;
      set_maximized(true);
    } else if event == 102 { // unmaximize
      app_state.write().unwrap().maximized = false;
      set_maximized(false);
    }
  });

  let on_title_bar_double_click_event = &crate::common::ipc::on_i32_event(move || {
    native_window().handle_title_barDoubleClick();
  });

  use wasm_bindgen::prelude::*;

  let app = Rc::new(App {
    app_parameters: props.app_parameters,
    deep_links,
    data_sources,
    app_configuration: Rc::clone(&props.app_configuration),
    extension_loaders,
    layout_loaders,
    native_app_menu,
    native_window,
    enable_global_css: true,
    appBarLeftInset: if props.appConfiguration.os_context.platform() == "darwin" && !app_state.read().unwrap().full_screen {
      Some(72)
    } else {
      None
    },
    onAppBarDoubleClick: move || {
      native_window().handle_title_bar_double_click();
    },
    is_maximized: app_state.read().unwrap().maximized,
    onMinimizeWindow,
    onMaximizeWindow,
    onUnmaximizeWindow,
    onCloseWindow,
    extraProviders,
  });

  web_sys::window()
    .expect("Failed to access window")
    .document()
    .expect("Failed to access document")
    .body()
    .unwrap()
    .append_child(&app.into_node())
    .unwrap();

  let full_screen_event_listener = js_sys::EventTarget::from(app_state.clone());
  full_screen_event_listener
    .add_event_listener_with_callback(
      "message",
      on_full_screen_event,
      true as _,
    )
    .expect("Failed to add event listener");

  let maximize_event_listener = js_sys::EventTarget::from(app_state.clone());
  maximize_event_listener
    .add_event_listener_with_callback(
      "message",
      on_maximize_event,
      true as _,
    )
    .expect("Failed to add event listener");

  let title_bar_double_click_event_listener =
    js_sys::EventTarget::from(app_state.clone());
  title_bar_double_click_event_listener
    .add_event_listener_with_callback(
      "message",
      on_title_bar_double_click_event,
      true as _,
    )
    .expect("Failed to add event listener");

  app
}
```