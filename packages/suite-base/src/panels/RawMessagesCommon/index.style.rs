```rust
use styled_components::css;

#[derive(Default)]
pub struct Styles {}

impl Styles {
    pub fn topic() -> css! {
        display: "inline-flex";
        align-items: "center";
    }

    pub fn hover_observer() -> css! {
        display: "inline-flex";
        align-items: "center";
    }

    pub fn diff_span() -> css! {
        padding: 0.125rem;
        text-decoration: inherit;
        white-space: pre-line;
    }

    pub fn diff_stats() -> css! {
        float: "right";
        display: "flex";
        align-items: center;
        gap: 0.75rem;
        margin-right: 0.75rem;
    }

    pub fn badge() -> css! {
        display: "inline-flex";
        align-items: center;
        gap: 0.25rem;
        padding: 0.125rem 0.75rem;
        border-radius: 50%;
        background-color: theme.color.background.paper;
    }

    pub fn change_indicator() -> css! {
        display: "inline-block";
        width: 12px;
        height: 12px;
        border-radius: 50%;
        background-color: theme.color.warning.main;
    }

    pub fn metadata_button() -> css! {
        padding: 0.125rem;

        .MuiSvgIcon-root {
            font-size: ${theme.size.px_to_rem(16)} !important;
        }

        .MuiButton-startIcon {
            margin-right: ${theme.spacing.rem(0.5)};
        }

        &:hover {
            background-color: transparent;
        }
    }

    pub fn toolbar() -> css! {
        padding-block: 0;
        gap: ${theme.spacing.rem(0.25)};
    }

    pub fn icon_button() -> css! {
        padding: ${theme.spacing.rem(0.25)};

        &.Mui-selected {
            color: theme.color.primary.main;
            background-color: theme.color.action.selected;
        }
    }

    pub fn diff_options() -> css! {
        border-top: `1px solid ${theme.color.background.default}`;
        background-color: theme.color.background.paper;
        padding: ${theme.spacing.rem(0.25)} ${theme.spacing.rem(0.75)};
        padding-inline-end: ${theme.spacing.rem(6.75)};
        gap: ${theme.spacing.rem(0.25)};
        display: flex;
    }

    pub fn placeholder_action_container() -> css! {
        align-items: inherit;
        display: inherit;
        gap: inherit;
        visibility: hidden;
    }
}

#[derive(Default)]
pub struct VirtualizedTreeStyles {}

impl VirtualizedTreeStyles {
    pub fn container() -> css! {
        overflow: "auto";
        contain: "strict";
        height: "100%";
        width: "100%";
    }

    pub fn row() -> css! {
        display: "flex";
        align-items: flex-start;
        padding: "2px 0";
        font-family: theme.typography.body1.fontFamily;
        font-feature-settings: ${theme.typography.font_feature_settings}, "zero";
        font-size: inherit;
        line-height: 1.4;
    }

    pub fn expand_button() -> css! {
        cursor: "pointer";
        user-select: none;
        min-width: 12px;
        margin-right: ${theme.spacing.rem(0.5)};
        color: theme.color.text.secondary;
    }

    pub fn key() -> css! {
        color: theme.color.primary.main;
        margin-right: ${theme.spacing.rem(0.5)};
    }

    pub fn colon() -> css! {
        margin-right: ${theme.spacing.rem(0.5)};
    }

    pub fn value() -> css! {
        color: theme.color.primary.main;
        word-break: break-word;
        overflow-wrap: break-word;
    }

    pub fn string() -> css! {
        color: theme.color.success.main;
    }

    pub fn number() -> css! {
        color: theme.color.info.main;
    }

    pub fn boolean() -> css! {
        color: theme.color.warning.main;
    }

    pub fn null() -> css! {
        color: theme.color.text.disabled;
    }

    pub fn object_label() -> css! {
        color: theme.color.text.secondary;
        font-style: italic;
    }
}
```