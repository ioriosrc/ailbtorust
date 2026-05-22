```rust
use std::any::{Any, TypeId};
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use crate::data_source::{DataSourceFactoryInterface, PlayerMetricsCollectorInterface};
use crate::file::File;
use crate::icon::RegisteredIconNames;

#[derive(Clone)]
pub struct DataSourceFactory {
    id: String,
    legacy_ids: Vec<String>,
    type_: DataSourceFactoryType,
    display_name: String,
    icon_name: Option<RegisteredIconNames>,
    description: Option<String>,
    docs_links: Option<Vec<{ label: &str; url: &str }>>,
    disabled_reason: Option<&'static str>,
    badge_text: Option<&'static str>,
    hidden: bool,
    warning: Option<&'static str>,
    sample_layout: Option<LayoutData>,
    form_config: Option<HashMap<String, Field>>,
    supported_file_types: Vec<String>,
    supports_multi_file: bool,
}

impl DataSourceFactory {
    fn new(
        id: String,
        legacy_ids: Vec<String>,
        type_: DataSourceFactoryType,
        display_name: String,
        icon_name: Option<RegisteredIconNames>,
        description: Option<String>,
        docs_links: Option<Vec<{ label: &str; url: &str }>>,
        disabled_reason: Option<&'static str>,
        badge_text: Option<&'static str>,
        hidden: bool,
        warning: Option<&'static str>,
        sample_layout: Option<LayoutData>,
        form_config: Option<HashMap<String, Field>>,
        supported_file_types: Vec<String>,
        supports_multi_file: bool,
    ) -> Self {
        Self {
            id,
            legacy_ids,
            type_,
            display_name,
            icon_name,
            description,
            docs_links,
            disabled_reason,
            badge_text,
            hidden,
            warning,
            sample_layout,
            form_config,
            supported_file_types,
            supports_multi_file,
        }
    }

    fn initialize(&self, args: DataSourceFactoryInitializeArgs) -> Option<Player> {
        unimplemented!("Implement the initialize method")
    }
}

#[derive(Clone)]
pub struct PlayerSelectionContext {
    select_source: Box<dyn Fn(String, Option<DataSourceArgs>)>,
    select_recent: Box<dyn Fn(String)>,
    available_sources: Vec<Box<dyn IDataSourceFactory>>,
    recent_sources: Vec<Box<dyn Any>>,
}

impl PlayerSelectionContext {
    pub fn new(
        select_source: Box<dyn Fn(String, Option<DataSourceArgs>)>,
        select_recent: Box<dyn Fn(String)>,
        available_sources: Vec<Box<dyn IDataSourceFactory>>,
        recent_sources: Vec<Box<dyn Any>>,
    ) -> Self {
        Self {
            select_source,
            select_recent,
            available_sources,
            recent_sources,
        }
    }

    pub fn select_source(&mut self, source_id: String, args: Option<DataSourceArgs>) {
        (self.select_source)(source_id, args);
    }

    pub fn select_recent(&mut self, recent_id: String) {
        (self.select_recent)(recent_id);
    }

    pub fn available_sources(&self) -> &Vec<Box<dyn IDataSourceFactory>> {
        &self.available_sources
    }

    pub fn recent_sources(&self) -> &Vec<Box<dyn Any>> {
        &self.recent_sources
    }
}

#[derive(Clone)]
pub struct PlayerSelectionContextProvider {
    context: PlayerSelectionContext,
}

impl PlayerSelectionContextProvider {
    pub fn new(
        select_source: Box<dyn Fn(String, Option<DataSourceArgs>)>,
        select_recent: Box<dyn Fn(String)>,
        available_sources: Vec<Box<dyn IDataSourceFactory>>,
        recent_sources: Vec<Box<dyn Any>>,
    ) -> Self {
        Self { context }
    }

    pub fn provide(&self) -> PlayerSelectionContext {
        self.context.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_selection_context() {
        let select_source = Box::new(|id, args| println!("Select source: {} with args: {:?}", id, args));
        let select_recent = Box::new(|id| println!("Select recent source: {}", id));
        let available_sources = vec![Box::new(DataSourceFactory::new(
            "file".to_string(),
            Vec::new(),
            DataSourceFactoryType::File,
            "File Data Source".to_string(),
            Some(RegisteredIconNames::File),
            Some("Description of File Data Source".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ))];
        let recent_sources = vec![Box::new(String::from("file_1")), Box::new(String::from("file_2"))];

        let context = PlayerSelectionContext {
            select_source,
            select_recent,
            available_sources,
            recent_sources,
        };

        let provider = PlayerSelectionContextProvider { context };

        let selected_source = provider.provide().select_source("file".to_string(), None);
        assert_eq!(selected_source, Some("file_1"));

        let select_recent = provider.provide().select_recent("file_1");
        assert_eq!(select_recent, Some("file_2"));
    }
}
```