```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use styled_components::{css, useTheme};
use styled_system::Css;
use tss_react::{css as tsx_css};

use crate::constants::NODE_HEADER_MIN_HEIGHT;

pub fn useStyles() -> css! {
    const theme = use_theme();
    css! {
        actionButton: {
            padding: theme.spacing(0.5);
        },
        editNameField: {
            font: "inherit",
            gridColumn: "span 2",
            width: "100%",
            
            & ".MuiInputBase-input": {
                fontSize: "0.75rem",
                padding: theme.spacing(0.75, 1),
            },
        },
        focusedNode: {
            animation: css! {
                from {
                    background-color: tinycolor(theme.palette.primary.main).set_alpha(0.3).to_rgb_string();
                }
                to {
                    background-color: transparent;
                }
            } 0.5s ease-in-out,
            
            & ":hover": {
                background-color: theme.palette.action.hover,

                & ".MuiCheckbox-root": {
                    visibility: "visible",
                },

                & "[data-node-function=edit-label]": {
                    visibility: "visible",
                },
            }
        },
        fieldPadding: {
            gridColumn: "span 2",
            height: theme.spacing(0.5),
        },
        iconWrapper: {
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
        },

        nodeHeader: {
            display: "flex",
            gridColumn: "span 2",
            paddingRight: theme.spacing(0.5),
            minHeight: NODE_HEADER_MIN_HEIGHT,

            media! (pointer: fine) {
                & ".MuiCheckbox-root": {
                    visibility: "hidden",
                },

                &:hover: {
                    & ".MuiCheckbox-root": {
                        visibility: "visible",
                    },
                },
            }
        },
        nodeHeaderVisible: {
            media! (pointer: fine) {
                & ".MuiCheckbox-root": {
                    visibility: "hidden",
                },
                &:hover: {
                    & ".MuiCheckbox-root": {
                        visibility: "visible",
                    },
                },
            }
        },

        nodeHeaderToggle: {
            display: "grid",
            alignItems: "center",
            gridTemplateColumns: "auto auto auto 1fr",
            opacity: 0.6,
            userSelect: "none",
            width: "100%",
            minWidth: 0,
        },
        nodeHeaderToggleHasProperties: {
            cursor: "pointer",
        },
        nodeHeaderToggleVisible: {
            opacity: 1,
        },
        errorTooltip: {
            whiteSpace: "pre-line",
            maxHeight: "15vh",
            overflowY: "auto",
        },
        nodeHeaderDragging: {
            opacity: 0.5,
        },
        nodeHeaderDropTarget: {
            background-color: tinycolor(theme.palette.primary.main).set_alpha(0.2).to_rgb_string(),
            borderTop: `2px solid ${theme.palette.primary.main}`,
        }
    }
}
```