```rust
use mui::{
    components::{Menu, MenuItem},
    props::PaperProps,
    PopoverPosition, PopoverReference,
};

use crate::AppSettingsTab;
use crate::workspace_actions::dialog_actions::PreferencesActions;

pub fn SettingsMenu({
    anchor_el: anchorEl,
    anchor_reference: anchor_reference,
    anchor_position: anchor_position,
    disable_portal: disable_portal,
    open: open,
}: {
    anchor_el: Option<&Element>,
    anchor_reference: PopoverReference,
    anchor_position: PopoverPosition,
    disable_portal: bool,
    open: bool,
}) -> JSX.Element {
    let classes = useStyles();

    let t = use_translation("appBar");

    let on_settings_click = useCallback(
        |tab: AppSettingsTab| {
            dialog_actions.preferences.open(tab);
        },
        [dialog_actions.preferences],
    );

    return (
        <Menu
            anchorEl={anchor_el}
            anchorReference={anchor_reference}
            anchorPosition={anchor_position}
            disablePortal={disable_portal}
            id="user-menu"
            open={open}
            onClose={handle_close}
            onClick={handle_close}
            slotProps={{
                list: {
                    className: classes.menu_list,
                    dense: true,
                },
                paper: {
                    "data-tourid": "user-menu",
                } as Partial<PaperProps & { "data-tourid"?: string }>,
            }}
        >
            <MenuItem
                onClick={on_settings_click}
            >
                {t("settings")}
            </MenuItem>
            <MenuItem
                onClick={on_settings_click("extensions")}
            >
                {t("extensions")}
            </MenuItem>
        </Menu>
    );
}

fn useStyles() -> ClassesObject {
    makeStyles!({
        menu_list: {
            minWidth: 200,
        },
    })
}
```