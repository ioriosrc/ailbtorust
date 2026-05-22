```rust
use fluentui::{
    icons::{ChevronDown12Regular, PanelLeft24Filled, PanelRight24Regular, SlideAdd24Regular},
    material::Avatar,
    material::IconButton,
    material::Tooltip,
};
use mui_core::{grid::GridArea, use_tss, use_window_event, use_window_size, use_window_state};
use react::prelude::*;
use react_i18next::{Trans, TFunction};
use tinycolor2::TColor;
use suite_base::{
    app_config::AppSetting,
    context::{CurrentLayoutContext, WorkspaceContext},
    hooks::{use_app_configuration_value},
    types::{DataSource, MemoryUseIndicator, AppMenu, CustomWindowControlsProps},
};

type AppBarProps = CustomWindowControlsProps;

fn useStyles() -> use_tss<"app-bar"> {
    use_tss!(|theme: &fluentui::theme::Theme| {
        grid_area: "start middle end".to_string(),
        grid_template_columns: "1fr auto 1fr".to_string(),
        align_items: "center".to_string(),

        logo: {
            padding: theme.spacing(0.75, 0.5),
            font_size: "2rem".to_string(),
            color: theme.palette.app_bar.primary.to_rgb().to_string(),
            border_radius: 0.0,

            svg_not_fluentui_icon_root: {
                font_size: "1em".to_string(),
            },

            &:hover: {
                background_color: tc(theme.palette.common.white).set_alpha(0.08).to_rgb().to_string(),
            },

            &.Mui-selected: {
                background_color: theme.palette.app_bar.primary.to_rgb().to_string(),
                color: theme.palette.common.white.to_string(),
            },

            &.Mui-disabled: {
                color: "currentColor",
                opacity: theme.palette.action.disabled_opacity,
            },
        },

        drop_down_icon: {
            font_size: "12px".to_string(),
        },

        start: {
            grid_area: "start",
            display: "flex",
            flex: 1.0,
            align_items: "center",

            &:hover: {
                background_color: tc(theme.palette.common.white)
                    .darken(45)
                    .to_rgb()
                    .to_string(),
            },

            [`.${classes.avatar}`]: {
                background_color: theme.palette.app_bar.main.to_rgb().lighten(20).to_rgb().to_string(),
            },
        },

        start_inner: {
            display: "flex",
            align_items: "center",
            ...NOT_DRAGGABLE_STYLE, // make buttons clickable for desktop app
        },

        middle: {
            grid_area: "middle",
            justify_self: "center",
            overflow: "hidden",
            max_width: "100%",
            ...NOT_DRAGGABLE_STYLE, // make buttons clickable for desktop app
        },

        end: {
            grid_area: "end",
            flex: 1.0,
            display: "flex",
            justify_content: "flex-end",
        },

        end_inner: {
            display: "flex",
            align_items: "center",
            ...NOT_DRAGGABLE_STYLE, // make buttons clickable for desktop app
        },

        key_equivalent: {
            font_family: custom_typography.font_monospace.to_string(),
            background_color: tc(theme.palette.common.white)
                .darken(45)
                .to_rgb()
                .to_string(),
            padding: theme.spacing(0.0, 0.5),
            aspect_ratio: 1.0,
            border_radius: theme.shape.border_radius,
            margin_left: theme.spacing(1.0),
        },

        tooltip: {
            margin_top: `${theme.spacing(0.5)} !important`,
        },

        avatar: {
            color: theme.palette.common.white.to_string(),
            background_color: tc(theme.palette.app_bar.main).lighten().to_rgb().to_string(),
            height: theme.spacing(3.5),
            width: theme.spacing(3.5),
        },

        icon_button: {
            padding: theme.spacing(1.0),
            border_radius: 0.0,

            &:hover: {
                background_color: tc(theme.palette.common.white)
                    .darken(0.08)
                    .to_rgb()
                    .to_string(),

                [`.${classes.avatar}`]: {
                    background_color: tc(theme.palette.app_bar.main).lighten(20).to_rgb().to_string(),
                },
            },

            &.Mui-selected: {
                background_color: theme.palette.app_bar.primary.to_rgb().to_string(),

                [`.${classes.avatar}`]: {
                    background_color: tc(theme.palette.app_bar.main).lighten(30).to_rgb().to_string(),
                },
            },
        },
    })
}

fn AppBar(props: AppBarProps) -> JSX.Element {
    let {
        debug_drag_region,
        is_maximized,
        left inset,
        onClose_window,
        onDoubleClick,
        onMaximize_window,
        onMinimize_window,
        show_custom_window_controls = false,
    } = props;

    let { classes, cx, theme } = useStyles(debug_drag_region);

    let { appBar_layout_button } = use_app_context();
    let [enable_memory_use_indicator = false] = use_app_configuration_value(AppSetting.ENABLE_MEMORY_USE_INDICATOR);

    let has_current_layout = use_current_layout_selector(|state| state.selected_layout.is_some());

    let left_sidebar_open = use_workspace_store(|store| store.sidebars.left.open);
    let right_sidebar_open = use_workspace_store(|store| store.sidebars.right.open);

    let { sidebar_actions } = use_workspace_actions();

    let [app_menu_el, set_app_menu_el] = useState<undefined | HTMLElement>(undefined);
    let [user_anchor_el, setUser_anchor_el] = useState<undefined | HTMLElement>(undefined);
    let [panel_anchor_el, set_panel_anchor_el] = useState<undefined | HTMLElement>(undefined);

    let app_menu_open = Boolean(app_menu_el);
    let user_menu_open = Boolean(user_anchor_el);
    let panel_menu_open = Boolean(panel_anchor_el);

    return (
        <>
            <AppBarContainer onDoubleClick={onDoubleClick} leftInset={left inset}>
                <div className={classes.toolbar}>
                    <div className={classes.start}>
                        <div className={classes.start_inner}>
                            <IconButton
                                className={cx(classes.logo, { "Mui-selected": app_menu_open })}
                                color="inherit"
                                id="app-menu-button"
                                data-testid="AppMenuButton"
                                title="Menu"
                                aria-controls={app_menu_open ? "user-menu" : undefined}
                                aria-haspopup="true"
                                aria-expanded={app_menu_open ? "true" : undefined}
                                onClick={(event) => {
                                    set_app_menu_el(event.currentTarget);
                                }}
                                data-testid="app-menu-button"
                            >
                                <Avatar className={classes.avatar} variant="rounded" />
                            </IconButton>
                            <Tooltip classes={{ tooltip: classes.tooltip }} title="Profile" arrow={false}>
                                <IconButton
                                    className={cx(classes.icon_button, { "Mui-selected": user_menu_open })}
                                    aria-label="User profile menu button"
                                    color="inherit"
                                    id="user-button"
                                    data-testid="user-button"
                                    aria-controls={user_menu_open ? "user-menu" : undefined}
                                    aria-haspopup="true"
                                    aria-expanded={user_menu_open ? "true" : undefined}
                                    onClick={(event) => {
                                        setUser_anchor_el(event.currentTarget);
                                    }}
                                >
                                    <Avatar className={classes.avatar} variant="rounded" />
                                </IconButton>
                            </Tooltip>
                        </div>
                    </div>

                    <div className={classes.middle}>
                        <DataSource />
                    </div>

                    <div className={classes.end}>
                        <div className={classes.end_inner}>
                            <NetworkStatusIndicator />
                            {enable_memory_use_indicator && <MemoryUseIndicator />}
                            {appBar_layout_button}
                            <Stack direction="row" alignItems="center" data-tourid="sidebar-button-group">
                                <IconButton
                                    title={
                                        <>
                                            {left_sidebar_open ? t("hideLeftSidebar") : t("showLeftSidebar")}{" "}
                                            <kbd className={classes.key_equivalent}>[</kbd>
                                        </>
                                    }
                                    aria-label={left_sidebar_open ? t("hideLeftSidebar") : t("showLeftSidebar")}
                                    onClick={() => {
                                        sidebar_actions.left.setOpen(!left_sidebar_open);
                                    }}
                                    data-tourid="left-sidebar-button"
                                    data-testid="left-sidebar-button"
                                >
                                    {left_sidebar_open ? <PanelLeft24Filled /> : <PanelLeft24Regular />}
                                </IconButton>
                                <IconButton
                                    title={
                                        <>
                                            {right_sidebar_open ? t("hideRightSidebar") : t("showRightSidebar")}{" "}
                                            <kbd className={classes.key_equivalent}>]</kbd>
                                        </>
                                    }
                                    aria-label={right_sidebar_open ? t("hideRightSidebar") : t("showRightSidebar")}
                                    onClick={() => {
                                        sidebar_actions.right.setOpen(!right_sidebar_open);
                                    }}
                                    data-tourid="right-sidebar-button"
                                    data-testid="right-sidebar-button"
                                >
                                    {right_sidebar_open ? <PanelRight24Filled /> : <PanelRight24Regular />}
                                </IconButton>
                            </Stack>
                            <Tooltip classes={{ tooltip: classes.tooltip }} title="Profile" arrow={false}>
                                <IconButton
                                    className={cx(classes.icon_button, { "Mui-selected": user_menu_open })}
                                    aria-label="User profile menu button"
                                    color="inherit"
                                    id="user-button"
                                    data-testid="user-button"
                                    aria-controls={user_menu_open ? "user-menu" : undefined}
                                    aria-haspopup="true"
                                    aria-expanded={user_menu_open ? "true" : undefined}
                                    onClick={(event) => {
                                        setUser_anchor_el(event.currentTarget);
                                    }}
                                >
                                    <Avatar className={classes.avatar} variant="rounded" />
                                </IconButton>
                            </Tooltip>
                        </div>
                    </div>
                </div>
            </AppBarContainer>
            <AddPanelMenu
                anchorEl={panel_anchor_el}
                open={panel_menu_open}
                handleClose={() => {
                    set_panel_anchor_el(undefined);
                }}
            />
            <SettingsMenu
                anchorEl={user_anchor_el}
                open={user_menu_open}
                handleClose={() => {
                    setUser_anchor_el(undefined);
                }}
            />
        </>
    );
}

fn NOT_DRAGGABLE_STYLE() -> GridArea {
    GridArea::from("start middle end".to_string())
}

fn main() {
    // Main function to set up the application
    AppBuilder::new()
        .add_component(AppBar)
        .build();
}
```