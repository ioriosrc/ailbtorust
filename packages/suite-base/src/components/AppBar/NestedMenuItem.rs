```rust
use fluentui::icons::ChevronRight12Regular;
use mui::{
    components::{AppBarMenuItem, Divider, Menu, MenuItem, Typography},
    prelude::*,
    theme::create_with_mdi_icons,
};
use serde_json::Value;

fn main() {
    create_with_mdi_icons!();

    let classes = makeStyles::<(), String>()(|theme| {
        props => ({
            menu: {
                pointer_events: "none",
            },
            paper: {
                pointer_events: "auto",
                margin_top: theme.spacing(-1),
            },
            menuItem: {
                justify_content: JustifyContent::SpaceBetween,
                cursor: Cursor::Pointer,
                gap: theme.spacing(2),

                &.Mui-selected, &.Mui-selected:hover => {
                    background_color: theme.palette.action.hover,
                },
                [`:not(:hover, :focus) .${classes.endIcon}`] => {
                    opacity: 0.6,
                },
                kbd => {
                    font: "inherit",
                    color: theme.palette.text.disabled,
                },
            },
            menuList: {
                min_width: 180,
                max_width: 280,
            },
            endIcon: {
                margin_right: theme.spacing(-0.75),
            },
        })
    });

    struct NestedMenuItemProps {
        id: Option<&'static str>,
        items: Vec<AppBarMenuItem>,
        open: bool,
        on_pointer_enter: fn(&'static str),
    }

    impl Component for NestedMenuItemProps {
        type State = ();

        fn render(self, cx) -> NodeRef<Node> {
            let { classes } = useStyles(cx);
            let { children, items, open, on_pointer_enter } = self;
            let mut anchor_el: Option<HtmlLIElement> = None;

            fn handle_pointer_enter(&self, id: &'static str) {
                if let Some(f) = &self.on_pointer_enter {
                    f(id);
                }
            }

            NodeRef::new((
                <MenuItem
                    id={id}
                    ref |ref| anchor_el = ref;
                    selected={open}
                    className={classes.menuItem}
                    onPointerEnter=handle_pointer_enter
                    data-testid={id}
                >
                    {children}
                    <ChevronRight12Regular className={classes.endIcon} />
                </MenuItem>,
                <Menu
                    classes={{
                        root: classes.menu,
                        paper: classes.paper,
                    }}
                    open={open}
                    disable_portal
                    anchor_el=anchor_el
                    onClose={() => {
                        anchor_el = None;
                    }}
                    onMouse_leave={() => {
                        anchor_el = None;
                    }}
                    anchor_origin={{ vertical: "top", horizontal: "right" }}
                    slotProps={{
                        list: {
                            dense: true,
                            className: classes.menuList,
                        },
                    }}
                    autoFocus={false}
                    disable_auto_focus
                    disable_enforce_focus
                    hide backdrop
                >
                    {items.iter().map(|item| {
                        match item.type {
                            "item" => {
                                let mut menu_item = MenuItem::new();
                                menu_item.label(item.label);
                                if let Some(shortcut) = &item.shortcut {
                                    menu_item.keyboard shortcut(shortcut);
                                }
                                menu_item.on_click(move |_| {
                                    item.onClick();
                                });
                                if item.disabled {
                                    menu_item.disabled();
                                }
                                node_ref!(menu_item)
                            },
                            "divider" => node_ref!(Divider::new()),
                            "subheader" => node_ref!(Typography::new().variant("overline").children(item.label)),
                        };
                    })}
                </Menu>,
            ))
        }
    }
}
```