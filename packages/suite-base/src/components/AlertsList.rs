```rust
use std::fmt::{Display, Error};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use fluent_ui::icons;
use flume_message_pipeline::MessagePipelineContext;
use flume_message_pipeline::PlayerAlert;
use flume_message_pipeline::NotificationSeverity;
use flume_util_send_notification::{DetailsType, Severity};
use flume_ui::theme;

use crate::ui::alerts::EmptyState;
use crate::ui::alerts::AlertIcon;
use crate::ui::alerts::AlertDetails;
use crate::ui::Stack;

pub fn AlertsList() -> impl Display {
    use fluent_ui::icons::{ArrowDropDownIcon, ErrorCircle16Regular, Info16Regular};
    use flume_message_pipeline::select_player_alerts;
    use flume_message_pipeline::select_alerts;
    use flume_util_send_notification::DetailsType;

    let t = &flume_i18n::use_t("alertsList");

    let classes = use_styles();

    let player_alerts = select_player_alerts();
    let session_alerts = select_alerts();
    let all_alerts = vec![&session_alerts, &player_alerts].concat();

    if all_alerts.is_empty() {
        return EmptyState::new(t("noAlertsFound"));
    }

    Stack {
        flex: "auto",
        overflow: "auto",
    }
    .children(all_alerts.into_iter().map(|alert| {
        Accordion {
            className: classes.acccordion,
            key: alert.message.to_string(),
            slot_props: flume_ui::accordion::AccordionTransitionProps {
                unmount_on_exit: true,
            },
            default_expanded: true,
        }
        .children(vec![
            AccordionSummary {
                className: classes.acccordion_summary,
                expand_icon: ArrowDropDownIcon::new(),
                title: alert.message.to_string(),
            }
            .children(vec![
                AlertIcon::new(alert.severity),
                Typography::new()
                    .variant("inherit")
                    .no_wrap()
                    .to_string(alert.message.clone()),
            ]),
            Divider::new(),
            AlertDetails {
                details: alert.error.clone(),
                tip: alert.tip.clone(),
            }
            .to_string(),
        ])
    }))
}

fn use_styles() -> flume_ui::theme::css::Stylesheet<flume_ui::theme::ColorScheme> {
    makeStyles()
        .use(theme::create_color_scheme())
        .map(|(scheme, stylesheet)| {
            stylesheet.add_rules({
                "accordion": {
                    "background": "none",
                    "box-shadow": "none",
                    "border-bottom": format!("1px solid {}", scheme.divider),
                    "margin-top": "0",
                },
                ".Mui-expanded .accordion-details": {
                    "margin-top": "0",
                },
                ".accordion-summary": {
                    "height": "30px",
                    "min-height": "auto",
                    "padding": format!("{}px 0.5em 0 0.75em", scheme.spacing),
                    "font-weight": "500",
                    "transition": format!("background-color {}ms", scheme.action.hover_duration),
                },
                ".Mui-expanded .accordion-summary": {
                    "min-height": "auto",
                },
                ".accordion-summary:hover": {
                    "background-color": format!("{} {}", scheme.action.hover, theme::color::primary.main),
                },
                ".accordion-summary.expand-icon-wrapper": {
                    transform: "rotate(-90deg)",
                },
                ".accordion-summary.expand-icon-wrapper.Mui-expanded": {
                    transform: "rotate(0deg)",
                },
                ".accordion-details .details-text": {
                    "color": theme::color::text.primary,
                    "font-size": scheme.typography.caption.fontSize,
                    "line-height": 1.5,
                    "white-space": "pre-wrap",
                    "max-height": "30vh",
                    "overflow": "auto",
                    "flex": "1",
                    "background-color": theme::color::action.hover,
                    "padding": scheme.spacing,
                },
            })
        })
}
```