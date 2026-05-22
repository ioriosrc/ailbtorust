```rust
use crate::components::HighlightedText;
use crate::context::EventsContext::{DataSourceEvent, TimelinePositionedEvent};
use crate::theme::{alpha, palette};
use std::cmp::Ordering;

pub fn format_event_duration(event: DataSourceEvent) -> String {
    if event.duration_nanos == 0 {
        // instant
        return "-";
    }

    if event.duration_nanos.is_empty() {
        return "";
    }

    let int_duration = event.duration_nanos.parse::<u128>().unwrap();

    if int_duration >= 1_000_000_000 {
        return format!("{}s", int_duration / 1_000_000_000);
    }

    if int_duration >= 1_000_000 {
        return format!("{}ms", int_duration / 1_000_000);
    }

    if int_duration >= 1_000 {
        return format!("{}µs", int_duration / 1_000);
    }

    return format!("{}ns", event.duration_nanos);
}

pub fn EventViewComponent(params: {
    event: TimelinePositionedEvent;
    filter: &str;
    formatted_time: String;
    is_hovered: bool;
    isSelected: bool;
    onClick: fn(TimelinePositionedEvent);
    on_hover_start: fn(TimelinePositionedEvent);
    on_hover_end: fn(TimelinePositionedEvent);
}) -> React::Element {
    let { event, filter, formatted_time, is_hovered, isSelected, onClick, on_hover_start, on_hover_end } = params;
    let classes = useStyles();

    let fields = vec![
        ("start", formatted_time.clone()),
        if event.event.duration_nanos != 0 {
            Some(format!("duration", format_event_duration(event.event)))
        } else {
            None
        },
        ...event.event.metadata.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<Vec<_>>(),
    ]
    .iter()
    .filter_map(|&(key, value)| if key.contains(filter) || !value.contains(filter) { Some((key.clone(), value.clone())) } else { None })
    .collect::<Vec<_>>();

    html! {
        <div
            data-testid="sidebar-event"
            class={cx(classes.event, {
                [classes.event_selected]: isSelected,
                [classes.event_hovered]: is_hovered,
            })}
            onclick={move || onClick(event)}
            onmouseenter={move || on_hover_start(event)}
            onmouseleave={move || on_hover_end(event)}
        >
            {fields.iter().map(|&(key, value)| html! {
                <div class={cx(classes.event_metadata)}>
                    <HighlightedText text={key} highlight={filter} />
                </div>
                <div class={cx(classes.event_metadata)}>
                    <HighlightedText text={value} highlight={filter} />
                </div>
            })}
            <div class={classes.spacer} />
        </div>
    }
}

pub fn useStyles() -> crate::theme::create_use_styles! {
    type EventMetadata = String;
    type EventSelected = bool;
    type EventHovered = bool;

    #[derive(Default)]
    pub struct StyleProps {}

    impl css::WithTheme for StyleProps {
        fn with_theme(&self, theme: &crate::theme::Theme) -> Self::StyleData {
            let mut style_data = Default::default();
            style_data.spacing = theme.spacing(1);
            style_data.divider = theme.palette.divider;
            style_data.background_color = theme.palette.background.default;
            style_data.border_right = "1px solid".to_string();
            style_data.border_bottom = "1px solid".to_string();

            if style_data.nth_of_type(&"odd") {
                style_data.border_left = "1px solid".to_string();
            }

            if style_data.is_first_of_type() {
                style_data.border_top = "1px solid".to_string();
                style_data.border_top_left_radius = theme.shape.border_radius;
            } else if style_data.is_nth_last_of_type(&"2") {
                style_data.border_top = "1px solid".to_string();
                style_data.border_top_right_radius = theme.shape.border_radius;
            } else if style_data.is_nth_last_of_type(&"3") {
                style_data.border_bottom_left_radius = theme.shape.border_radius;
            }

            style_data
        }
    }

    #[derive(Default)]
    pub struct StyleData {
        spacing: u16,
        divider: String,
        background_color: String,
        border_right: String,
        border_bottom: String,
        nth_of_type: fn(&str) -> bool,
        is_first_of_type: fn() -> bool,
        is_nth_last_of_type: fn(&str) -> bool,
    }

    #[derive(Default)]
    pub struct Classes {
        event: css::Classes<StyleProps, StyleData>,
        event_metadata: css::Classes<StyleProps, StyleData>,
    }

    impl Classes {
        pub fn new() -> Self {
            let mut classes = Classes::default();
            classes.event = css::create_use_styles!();
            classes.event_metadata = css::create_use_styles!();
            classes
        }
    }

    pub fn cx(&self) -> &Classes {
        self
    }
}
```