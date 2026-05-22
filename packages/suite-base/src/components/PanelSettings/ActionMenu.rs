```rust
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum FontSize {
    Medium,
}

impl Display for FontSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontSize::Medium => write!(f, "medium"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActionMenuProps {
    allow_share: bool,
    on_reset: Box<dyn Fn()>,
    on_share: Box<dyn Fn()>,
    font_size: FontSize,
}

impl Display for ActionMenuProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActionMenuProps {{ allow_share: {}, on_reset: {:?}, on_share: {:?}, font_size: {} }}", self.allow_share, self.on_reset, self.on_share, self.font_size)
    }
}

pub fn ActionMenu(props: ActionMenuProps) -> std::fmt::Result {
    let { classes, cx } = use_styles();
    let (anchor_el, set_anchor_el) = use_state::<Option<web_sys::HtmlElement>>(None);
    let { t } = use_t();

    let handle_click = move |event| {
        set_anchor_el(Some(event.target().unwrap()));
        set_menu_open(true);
    };

    let handleClose = move || {
        set_anchor_el(None);
        set_menu_open(false);
    };

    let handle_share = move || {
        props.on_share();
        handleClose();
    };

    let handle_reset = move || {
        props.on_reset();
        handleClose();
    };

    Ok::<_, std::fmt::Error>(format!(
        r#"<div>
            <IconButton
                class={cx({ [classes.iconButtonSmall]: props.font_size == FontSize::Medium })}}
                data-testid="basic-button"
                id="basic-button"
                aria-controls={"basic-menu"}
                aria-haspopup="true"
                aria-expanded={is_menu_open ? "true" : undefined}
                onClick={handle_click}
            >
                <MoreVertIcon fontSize={props.font_size} />
            </IconButton>
            <Menu
                data-testid="basic-menu"
                id="basic-menu"
                anchorEl={anchor_el}
                open={is_menu_open}
                onClose={handle_close}
                slotProps={{
                    list: {
                        "aria-labelledby": "basic-button",
                    },
                }}
            >
                <MenuItem disabled={!props.allow_share} aria-disabled={!props.allow_share} onClick={handle_share}>
                    {t("importOrExportSettingsWithEllipsis")}
                </MenuItem>
                <MenuItem onClick={handle_reset}>{t("resetToDefaults")}</MenuItem>
            </Menu>
        </div>"#
    ))
}
```