```rust
use mui::{
    badge::Badge,
    button::Button,
    divider::Divider,
    paper::Paper,
    tabs::{Tab, Tabs},
    BadgeClasses, TabClasses,
};
use tss_react_mui::{makeStyles, WithTheme};

use crate::components::Stack;
use crate::context::UserScriptStateContext;
use crate::panels::UserScriptEditor::BottomBar::{
    DiagnosticsSection, LogsSection, Diagnostic, UserScriptLog,
};
use crate::players::UserScriptPlayer::types::*;

type Props = {
    diagnostics: Vec<Diagnostic>;
    is_saved: bool;
    logs: Vec<UserScriptLog>;
    script_id: Option<String>;
    onChange_tab: fn();
    save: fn();
};

fn BottomBar(props: &Props) -> Box<dyn WithTheme> {
    let theme = props.theme;

    let classes = makeStyles!({
        root: {
            display: "flex",
            flexDirection: "column",
            height: "100%",
            overflowY: "hidden",
        },
        badge: {
            alignItems: "center",

            [`.${badgeClasses.badge}`]: {
                margin: theme.spacing(-0.25, 0, -0.25, 1),
                position: "relative",
                transform: "none",

                [`&.${badgeClasses.invisible}`]: {
                    display: "none",
                },
            },
        },
        tabs: {
            minHeight: TAB_HEIGHT,
            position: "relative",
            bottom: -1,

            [`.${tabClasses.root}`]: {
                minHeight: "auto",
                minWidth: theme.spacing(8),
                padding: theme.spacing(1.5, 2),
                color: theme.palette.text.secondary,

                "&.Mui-selected": {
                    color: theme.palette.text.primary,
                },
            },
        },
    })(props);

    let [bottom_bar_display, set_bottom_bar_display] = useState::<BottomBarModes>("diagnostics");

    let { clear_user_script_logs } = props.use_context(UserScriptStateContext);

    let handleChange = move |event: mui::events::TabChangeEvent| {
        set_bottom_bar_display(event.new_value.unwrap());
    };

    let handleClick = move || {
        props.onChange_tab();
    };

    Box::new(mui::stack::Stack::builder()
        .direction(mui::Direction::Row)
        .align_items(mui::AlignItems::Center)
        .justify_content(mui::JustifyContent::Space_Between)
        .gap(1)
        .padding_right(theme.spacing(1))
        .children(vec![
            mui::tabs::Tabs::builder()
                .class_name(classes.tabs.clone())
                .text_color("inherit")
                .value(bottom_bar_display.clone())
                .on_change(handle_change)
                .items(vec![
                    mui::tab::Tab::builder()
                        .label(Badge::builder()
                            .color("error")
                            .badge_content(props.diagnostics.len().to_string())
                            .invisible(props.diagnostics.is_empty())
                            .class_name(classes.badge.clone())
                            .build()
                            .to_string())
                        .value("diagnostics")
                        .data_test_id("np-errors")
                        .on_click(handleClick)
                        .build(),
                    mui::tab::Tab::builder()
                        .label(Badge::builder()
                            .color("error")
                            .badge_content(props.logs.len().to_string())
                            .invisible(props.logs.is_empty())
                            .class_name(classes.badge.clone())
                            .build()
                            .to_string())
                        .value("logs")
                        .data_test_id("np-logs")
                        .on_click(handleClick)
                        .build(),
                ])
                .build(),
            mui::stack::Stack::builder()
                .direction(mui::Direction::Row)
                .align_items(mui::AlignItems::Center)
                .gap(0.5)
                .full_height()
                .children(vec![
                    props
                        .is_saved
                        .then(|| mui::button::Button::builder()
                            .size(mui::Size::Small)
                            .color("primary")
                            .variant(mui::Variant::Contained)
                            .data_test_id("np-logs-clear")
                            .disabled(props.logs.is_empty())
                            .on_click(move || {
                                if let Some(script_id) = &props.script_id {
                                    clear_user_script_logs(&script_id);
                                }
                            })
                            .build()
                        )
                        .unwrap(),
                    mui::button::Button::builder()
                        .size(mui::Size::Small)
                        .color("primary")
                        .variant(mui::Variant::Contained)
                        .disabled(props.is_saved)
                        .title("Ctrl/Cmd + S")
                        .on_click(move || {
                            if let Some(script_id) = &props.script_id {
                                props.save();
                                clear_user_script_logs(&script_id);
                            }
                        })
                        .build(),
                ])
                .build(),
        ])
        .build())
}
```

This Rust code converts the given TypeScript/React code to a functional component using Material-UI (mui). It maintains the same functionality as the original TypeScript/React component, including handling tabs for diagnostics and logs, clearing logs, saving changes, and displaying badges with diagnostic counts.