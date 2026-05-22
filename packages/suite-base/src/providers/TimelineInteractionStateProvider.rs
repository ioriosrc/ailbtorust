```rust
use crate::{TimelinePositionedEvent, HoverValue};
use std::collections::HashMap;

#[derive(Default)]
struct TimelineInteractionStateStore {
    events_at_hover_value: HashMap<String, TimelinePositionedEvent>,
    global_bounds: Option<SyncBounds>,
    hovered_event: Option<TimelinePositionedEvent>,
    hover_value: Option<HoverValue>,
}

impl TimelineInteractionStateStore {
    fn clear_hover_value(&mut self, component_id: &str) {
        if let Some(event) = self.events_at_hover_value.remove(component_id) {
            self.hover_value = match event.type_str() {
                "PLAYBACK_SECONDS" => Some(HoverValue::Seconds(event.seconds_since_start)),
                _ => None,
            };
        }
    }

    fn set_events_at_hover_value(&mut self, events: Vec<TimelinePositionedEvent>) {
        let events_by_id = HashMap::from_iter(events.into_iter().map(|event| (event.event.id.clone(), event)));
        self.events_at_hover_value = events_by_id;
    }

    fn set_global_bounds(&mut self, new_bounds: Option<SyncBounds>) {
        self.global_bounds = new_bounds;
    }

    fn set_hovered_event(&mut self, hovered_event: Option<TimelinePositionedEvent>) {
        if let Some(event) = hovered_event {
            self.hover_value = match event.type_str() {
                "PLAYBACK_SECONDS" => Some(HoverValue::Seconds(event.seconds_since_start)),
                _ => None,
            };
        } else {
            self.hover_value = None;
        }
    }

    fn set_hover_value(&mut self, hover_value: HoverValue) {
        self.hover_value = Some(hover_value);
    }
}

#[derive(Default)]
struct SyncBounds;

#[derive(Clone, Debug, PartialEq)]
pub struct TimelinePositionedEvent {
    pub event: Event,
    pub seconds_since_start: f64,
}

#[derive(Debug)]
enum Event {
    PlaybackSeconds(f64),
    // Other event types
}
```