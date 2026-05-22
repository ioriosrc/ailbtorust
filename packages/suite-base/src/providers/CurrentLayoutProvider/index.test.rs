```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::HashMap;

use fake::{mock, MockExt, MockFn, Result};
use test_case::test_case;

mod current_layout_provider {
    use super::*;

    #[derive(Clone)]
    struct CurrentLayoutProvider {
        layouts: HashMap<String, LayoutData>,
        layout_id: Option<String>,
        is_busy: bool,
        online: bool,
        error: Option<String>,
    }

    impl CurrentLayoutProvider {
        fn new() -> Self {
            Self {
                layouts: HashMap::new(),
                layout_id: None,
                is_busy: false,
                online: true,
                error: None,
            }
        }

        fn mock_layouts(&mut self, layouts: Vec<LayoutData>) {
            self.layouts = layouts.into_iter().collect();
        }

        fn get_layout(&self, layout_id: &str) -> Result<&LayoutData> {
            match self.layouts.get(layout_id) {
                Some(layout) => Ok(layout),
                None => Err("Layout not found".to_string()),
            }
        }

        fn mock_layouts_is_busy(&mut self, is_busy: bool) {
            self.is_busy = is_busy;
        }

        fn mock_online(&mut self, online: bool) {
            self.online = online;
        }

        fn mock_error(&mut self, error: &str) {
            self.error = Some(error.to_string());
        }
    }

    #[derive(Clone)]
    struct UserProfileStorage {
        user_profile: HashMap<String, String>,
    }

    impl UserProfileStorage {
        fn new() -> Self {
            Self {
                user_profile: HashMap::new(),
            }
        }

        fn mock_user_profile(&mut self, user_profile: HashMap<String, String>) {
            self.user_profile = user_profile;
        }

        fn get_user_profile(&self) -> &HashMap<String, String> {
            &self.user_profile
        }

        fn mock_get_user_profile(&mut self) -> Result<&HashMap<String, String>> {
            Ok(&self.user_profile)
        }
    }

    #[test_case]
    fn test_default_layout_logic(default_layout: &str) {
        let mut layouts = HashMap::new();
        layouts.insert("layout1".to_string(), LayoutData {
            data: TestLayout,
            permission: Permission::CreatorWrite,
        });
        layouts.insert("layout2".to_string(), LayoutData {
            data: TestLayout,
            permission: Permission::OrgRead,
        });

        let mut user_profile = HashMap::new();
        user_profile.insert("current_layout".to_string(), default_layout.to_string());

        let current_layout_provider = CurrentLayoutProvider {
            layouts,
            layout_id: None,
            is_busy: false,
            online: true,
            error: None,
        };

        let user_profile_storage = UserProfileStorage {
            user_profile,
        };

        let mut result = render_test_cases! {
            "layout selection through app parameters" => {
                default_layout_provider.mock_layouts(layouts.clone());
                user_profile_storage.mock_user_profile(user_profile);

                let mut mock_result = MockResult::new();
                mock_result.expect_get().return_ref(&layouts);
                mock_result.expect_is_busy().return_once(false);

                mock_current_layout_provider(current_layout_provider, &mut mock_result).await;
            },
            "default layout logic" => {
                default_layout_provider.mock_layouts(layouts.clone());
                user_profile_storage.mock_user_profile(user_profile);

                let mut mock_result = MockResult::new();
                mock_result.expect_get().return_ref(&layouts);
                mock_result.expect_is_busy().return_once(true);

                mock_current_layout_provider(current_layout_provider, &mut mock_result).await;
            },
        };

        assert_eq!(result.status_code(), 200);
    }

    async fn mock_current_layout_provider(
        current_layout_provider: CurrentLayoutProvider,
        mock_result: &mut MockResult,
    ) -> Result<()> {
        let layouts = current_layout_provider.layouts.clone();
        let layout_id = current_layout_provider.layout_id;
        let is_busy = current_layout_provider.is_busy;
        let online = current_layout_provider.online;
        let error = current_layout_provider.error;

        mock(mock_result, |_, _| {
            Ok(current_layout_provider)
        })?;

        mock(mock_result, |_req| {
            Ok(is_busy)
        })?;

        mock(mock_result, |_req| {
            Ok(online)
        })?;

        mock(mock_result, |_req| {
            Ok(error.clone().unwrap_or_default())
        })?;

        mock(mock_result, |_req| {
            Ok(layouts.get(&layout_id.unwrap()).map(|l| l.data).unwrap())
        })?;

        let result = current_layout_provider.get_layout(&layout_id.unwrap());

        match result {
            Ok(layout) => Ok(()),
            Err(error) => Err(format!("Failed to get layout: {}", error)),
        }
    }

    #[derive(Clone)]
    struct LayoutData {
        data: TestLayout,
        permission: Permission,
    }

    impl LayoutData {
        fn new(data: TestLayout, permission: Permission) -> Self {
            Self { data, permission }
        }
    }

    #[derive(Clone)]
    struct TestLayout;

    enum Permission {
        CreatorWrite,
        OrgRead,
    }

    mod render_test_cases {
        use super::*;

        async fn render_test_cases<T: 'static>(test_name: &str) -> Result<()> {
            let default_layout = "layout1";
            let layouts = HashMap::new();
            layouts.insert("layout1".to_string(), LayoutData {
                data: TestLayout,
                permission: Permission::CreatorWrite,
            });
            layouts.insert("layout2".to_string(), LayoutData {
                data: TestLayout,
                permission: Permission::OrgRead,
            });

            let mut user_profile = HashMap::new();
            user_profile.insert("current_layout".to_string(), default_layout.to_string());

            let current_layout_provider = CurrentLayoutProvider {
                layouts,
                layout_id: None,
                is_busy: false,
                online: true,
                error: None,
            };

            let user_profile_storage = UserProfileStorage {
                user_profile,
            };

            let mut mock_result = MockResult::new();
            mock_result.expect_get().return_ref(&layouts);
            mock_result.expect_is_busy().return_once(false);

            let result = current_layout_provider.get_layout(&default_layout.to_string());

            match result {
                Ok(layout) => Ok(()),
                Err(error) => Err(format!("Failed to get layout: {}", error)),
            }
        }

        #[tokio::test]
        async fn test_layout_selection_through_app_parameters() {
            render_test_cases!("layout selection through app parameters").await;
        }

        #[tokio::test]
        async fn test_default_layout_logic() {
            render_test_cases!("default layout logic").await;
        }
    }
}
```