```rust
use crate::components::styled::{ListItem, MenuItem};
use crate::styles::spacing;
use crate::types::Theme;

pub struct StyledListItem {
    pub editing_name: bool,
    pub has_modifications: bool,
    pub deleted_on_server: bool,
}

impl ListItem for StyledListItem {
    fn style(&self) -> styled::Styles {
        styled::Styles {
            should_forward_prop: |prop| prop != "hasModifications" && prop != "deletedOnServer" && prop != "editingName",
            &".MuiListItemSecondaryAction-root": {
                right: spacing(0.25),
            },
            &".MuiListItemButton-root": {
                max_width: "100%",
            },
            "@media (pointer: fine)": {
                &".MuiListItemButton-root": {
                    padding_right: spacing(4.5),
                },
                &".MuiListItemSecondaryAction-root": {
                    visibility: !self.has_modifications && !self.deleted_on_server && "hidden",
                },
                "&:hover .MuiListItemSecondaryAction-root": {
                    visibility: "visible",
                },
            },
        }
    }
}

pub struct StyledMenuItem {
    pub debug: Option<bool>,
}
```

```rust
use crate::components::styled::{MenuItem};
use crate::styles::{spacing};
use crate::types::Theme;

pub struct MenuItemDebug {
    pub debug: bool,
}

impl MenuItem for MenuItemDebug {
    fn style(&self) -> styled::Styles {
        styled::Styles {
            position: "relative",
            &".MuiListItem-secondaryAction-root": {
                right: spacing(0.25),
            },
            &".MuiListItemText-root": {
                margin: 0,
            },
        }
    }
}
```