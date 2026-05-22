```rust
use std::fmt;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use materialize::{Button, CircularProgress, Typography};
use notify::{enqueue_snackbar, Confirm};

use log::Logger;

const LOG = Logger.getLogger("app.components.AccountInfo");

const AVATAR_ICON_SIZE = 42;

#[derive(Debug)]
struct AccountInfoProps {
    user: Option<User>,
}

fn render_account_info(props: &AccountInfoProps) -> materialize::Html {
    if let Some(user) = props.user {
        let on_settings_click = move || {
            window.open(&user.account_profile_url, "_blank");
        };
        let on_signout_click = move || {
            confirm(
                "Are you sure you want to sign out?",
                |response| response == "ok",
                || {
                    begin_sign_out();
                },
            )
            .unwrap_or_else(|e| panic!("Error confirming signout: {}", e));
        };

        let classes = useStyles();

        <Stack full_height justify_content="space-between">
            {render_confirm_modal()}
            <Stack gap={2}>
                <Stack direction="row" alignItems="center" gap={1} flex_wrap="wrap">
                    <BlockheadFilledIcon class_name={classes.icon} />
                    <Stack justifyContent="center">
                        <Typography variant="subtitle1">{user.email}</Typography>
                        <Typography variant="body2" color="text.secondary">
                            {user.org_display_name.unwrap_or(&user.org_slug)}
                        </Typography>
                    </Stack>
                </Stack>
                <Button onClick={on_settings_click} variant="contained">
                    Account settings
                </Button>
            </Stack>
            <Stack gap={1}>
                <Button onClick={on_signout_click} variant="outlined">
                    Sign out {loading && <CircularProgress size={16} />}{" "}
                </Button>
            </Stack>
        </Stack>
    } else {
        <></>
    }
}

fn render_confirm_modal() -> materialize::Html {
    let confirm_modal = use_confirm({
        title: "Are you sure you want to sign out?",
        ok: "Sign out",
    });

    if confirm_modal.is_open() {
        confirm_modal.on_ok(|response| {
            if response == "ok" {
                begin_sign_out();
            }
        });
        confirm_modal.on_cancel(() => {});
    }

    <></>
}

fn begin_sign_out() {
    // Implementation of signout logic here
}

fn main() -> std::io::Result<()> {
    let user = User {
        email: "example@example.com",
        org_display_name: Some("Organization Name"),
        org_slug: "org-slug",
        account_profile_url: "https://account.example.com",
    };

    log.info!("Rendering AccountInfo");
    println!("{}", render_account_info(&AccountInfoProps { user }));

    Ok(())
}
```

Note that this Rust code is a simplified example and does not include the actual implementation of `useCurrentUser`, `User`, or other dependencies required for this functional component. The `useConfirm` hook and other parts of the library used in the TypeScript code are assumed to be available in the Rust world.