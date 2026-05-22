```rust
use styled_components::{css, cssfn as c, styled};

#[derive(Debug)]
pub struct TreeClasses;

impl TreeClasses {
    pub const IS_DRAGGING: &'static str = "isDragging";
    pub const SELECTED: &'static str = "selected";
    pub const ROW: &'static str = "row";
}

#[derive(Debug)]
struct UseTopicListStyles;

impl UseTopicListStyles {
    pub fn styles(theme: &c.Theme) -> c.StyleSheet<(), ()> {
        c.create({
            body: css! {
                isDragging: {},
                selected: {},
                row: css! {
                    display: "flex",
                    alignItems: "center",
                    whiteSpace: "nowrap",
                    boxSizing: "border-box",
                    position: "relative",
                    height: "100%",
                    backgroundColor: theme.color("background.paper"),
                    gap: theme.space("0.5"),
                    paddingInline: theme.space("1", 0.75),
                    borderTop: `1px solid ${theme.color("action.selected")}`,
                    boxShadow: `0 1px 0 0 ${theme.color("action.selected")}`,
                    userSelect: "none",
                },
                ":not(:hover) .${TreeClasses::DRAG_HANDLE}": {
                    visibility: "hidden",
                },
                ".${TreeClasses::SELECTED}, .${TreeClasses::IS_DRAGGING}:active": {
                    // use opaque color for better drag preview
                    backgroundColor: c.mix(
                        theme.color("background.paper"),
                        theme.color("primary.main"),
                        100 * theme.opacity("action.selectedOpacity")),
                        "string",
                    ),
                    ...(theme.mode == "dark" && {
                        ":after": css! {
                            content: "''",
                            position: "absolute",
                            inset: "-1px 0 -1px 0",
                            border: `1px solid ${theme.color("primary.main")}`,
                            pointerEvents: "none",
                        },
                        ".${TreeClasses::ROW}:not(:last-of-type)": {
                            borderTop: `1px solid ${theme.color("primary.main")}`,
                        },
                    }),
                },
            },
        })
    }
}

#[derive(Debug)]
struct FieldRow;

impl FieldRow {
    pub fn styles(theme: &c.Theme) -> c.StyleSheet<(), ()> {
        c.create({
            row: css! {
                borderTop: `1px solid ${theme.color("background.paper")}`,
                backgroundColor: theme.color("action.hover"),
            },
        })
    }
}

#[derive(Debug)]
struct CountBadge;

impl CountBadge {
    pub fn styles(theme: &c.Theme) -> c.StyleSheet<(), ()> {
        c.create({
            marginInline: theme.space(-0.5),

            [`.${badgeClasses::BADGE}`]: css! {
                position: "relative",
                transform: "none",
            },
        })
    }
}

#[derive(Debug)]
struct TextContent;

impl TextContent {
    pub fn styles(theme: &c.Theme) -> c.StyleSheet<(), ()> {
        c.create({
            maxWidth: "100%",
        })
    }
}

#[derive(Debug)]
struct AliasedTopicName;

impl AliasedTopicName {
    pub fn styles(theme: &c.Theme) -> c.StyleSheet<(), ()> {
        c.create({
            color: theme.color("primary.main"),
            display: "block",
            textAlign: "start",
        })
    }
}

#[derive(Debug)]
struct NavIconButton;

impl NavIconButton {
    pub fn styles(theme: &c.Theme) -> c.StyleSheet<(), ()> {
        c.create({
            padding: theme.space(0.25),
        })
    }
}
```