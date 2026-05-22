```rust
use crate::components::{TimelinePositionedEvent, EventsStore};
use crate::context::{TimelineInteractionStateStore, useTimelineInteractionState};
use crate::hooks::{useEvents, selectEventsAtHoverValue, selectHoveredEvent, selectSelectedEventId};

fn EventTick(event: TimelinePositionedEvent) -> Box<dyn std::fmt::Display> {
    let events_at_hover_value = use_timeline_interaction_state(select_events_at_hover_value);
    let hovered_event = use_timeline_interaction_state(select_hovered_event);
    let selected_event_id = use_events(select_selected_event_id);

    let left = format!("calc({}%, -1px)", event.start_position * 100.0);
    let right = format!("calc(100% - {}%, -1px)", (1.0 - event.end_position) * 100.0);

    Box::new(format!(
        "<div class='tick' style='left: {}; right: {};'></div>",
        left,
        right
    ))
}

fn MemoEventTick(event: TimelinePositionedEvent) -> Box<dyn std::fmt::Display> {
    EventTick(event)
}

pub fn EventsOverlay() -> Box<dyn std::fmt::Display> {
    let events = use_events(select_events);
    let classes = useStyles();

    let div_classes = classes.root;

    format!(
        "<div class='root' style='{div_classes}'>",
        div_classes,
        events.value.iter().map(|event| EventTick(*event)).collect::<Vec<_>>()
    )
}
```