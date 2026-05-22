```rust
use crate::components::{PanelRoot, PanelToolbar};
use crate::utils::tc;
use crate::styles::{makeStyles, useClasses};

#[derive(Debug, PartialEq)]
pub struct PanelOverlayProps {
    actions: Vec<Action>,
    variant: Variant,
    highlight_mode: HighlightMode,
    drop_message: Option<String>,
    open: bool,
    onClickAway: Option<fn() -> ()>,
}

#[derive(Debug, PartialEq)]
enum Action {
    Button { key: String, text: String, icon: Box<dyn std::fmt::Display>, onClick: fn(), color: ButtonProps },
}

#[derive(Debug, PartialEq)]
pub enum Variant {
    ValidDropTarget,
    InvalidDropTarget,
    Selected,
}

#[derive(Debug, PartialEq)]
pub enum HighlightMode {
    Active,
    All,
}

const PANEL_ROOT_CLASS_NAME = "PanelRoot";
const PANEL_TOOLBAR_MIN_HEIGHT = 80;

fn makeStyles() -> tss::Sheets {
    makeStyles!({
        backdrop: {
            position: "absolute",
            zIndex: 10, // Adjust as needed
            padding: theme.spacing(2),
            container: "backdrop / size", // Adjust as needed
            backgroundColor: tc(theme.palette.background.default).alpha(0.0).to_rgb(),
        },
        invalidTarget: {
            backgroundColor: tc(theme.palette.background.default)
                .alpha(1 - theme.palette.action.disabledOpacity)
                .to_rgb(),
        },
        validTarget: {
            alignItems: "flex-end",
            backgroundColor: tc(theme.palette.primary.main)
                .alpha(theme.palette.action.hoverOpacity)
                .to_rgb(),
        },
        selected: {
            backgroundImage: `linear-gradient(to bottom, ${tc(theme.palette.primary.main).to_rgb()}, ${tc(theme.palette.primary.main).to_rgb()})`,
            backgroundColor: tc(theme.palette.background.default),
        },
        highlightActive: {
            [`.${PANEL_ROOT_CLASS_NAME}:not(:hover) &`]: {
                visibility: "hidden",
            },
        },
        highlightAll: {
            [`.${PANEL_ROOT_CLASS_NAME}:not(:hover) &`]: {
                [`.${classes.buttonGroup}`]: { visibility: "hidden" },
            },
        },
        buttonGroup: {
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            gap: theme.spacing(1),

            #[container("backdrop (max-height: 80px)")]
            flex-direction: "row",

            #[container("backdrop (min-height: 120px)")]
            margin_top: PANEL_TOOLBAR_MIN_HEIGHT,

            #[container("backdrop (min-width: 240px)")]
            flex_direction: "row",
        },
        buttonPaper: {
            flex: "0 0 50%",
            min_width: "50%",
        },
        button: {
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            white_space: "nowrap",
            text_align: "left",

            #[container("backdrop (max-width: 120px)")]
            display: "none",

            #[container("backdrop (max-height: 80px)")]
            display: "none",

            button_start_icon: {
                position: "relative",
                margin: "0",

                svg: {
                    height: "1em",
                    width: "1em",
                    font_size: 32,
                },
            },
        },
        button_text: {
            #[container("backdrop (max-width: 120px)")]
            display: "none",
        },
    })
}

fn useClasses() -> tss::SheetsRef {
    makeStyles().to_ref()
}

pub fn PanelOverlay(props: PanelOverlayProps, ref: &mut web_sys::Element) {
    let { actions, variant, highlight_mode, drop_message, open, onClickAway } = props;
    let classes = useClasses();

    // Render the backdrop and other components here
}
```