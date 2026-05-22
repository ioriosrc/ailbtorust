```rust
use mui_core::{
  AppBar, CircularProgress, IconButton, InputAdornment, TextField,
  Typography,
};
use mui_material::style::{createTheme, Stylesheet};

use chrono::{DateTime, Duration};
use serde_json::Value;

use crate::{
  context::{MessagePipelineContext, use_message_pipeline},
  hooks::{use_app_time_format, AppTimeFormat},
  store::{EventsStore, use_events},
  types::{TimelinePositionedEvent, TimelineInteractionStateStore, use_timeline_interaction_state},
};

pub fn EventsList() -> impl Into<crate::prelude::Element> {
  let events = use_events(select_events);
  let selected_event_id = use_events(select_selected_event_id);
  let select_event = use_events(select_select_event);
  let { format_time } = use_app_time_format();
  let seek = use_message_pipeline(select_seek);
  let events_at_hover_value = use_timeline_interaction_state(select_events_at_hover_value);
  let hovered_event = use_timeline_interaction_state(select_hovered_event);
  let set_hovered_event = use_timeline_interaction_state(select_set_hovered_event);
  let filter = use_events(select_event_filter);
  let set_filter = use_events(select_set_event_filter);

  let timestamped_events = useMemo(
    || {
      events.value.unwrap_or_default()
        .iter()
        .map(|event| {
          format!(
            "{}: {}",
            event.key,
            serde_json::to_string(&event.value).unwrap_or("N/A")
          )
        })
        .collect::<Vec<String>>()
    },
    [events],
  );

  let clear_filter = useCallback(
    || set_filter(Value::Null),
    [set_filter],
  );

  let onClick = useCallback(
    |event: TimelinePositionedEvent| {
      if event.event.id == selected_event_id() {
        select_event(None);
      } else {
        select_event(Some(event.event.id));
      }

      if seek() {
        seek(event.event.startTime);
      }
    },
    [seek, select_event, selected_event_id],
  );

  let on_hover_end = useCallback(() => set_hovered_event(None), [set_hovered_event]);

  let on_hover_start = useCallback(
    |event: TimelinePositionedEvent| set_hovered_event(Some(event)),
    [set_hovered_event],
  );

  let theme = createTheme({
    palette: {
      primary: {
        main: "#242933",
      },
      background: {
        paper: "#FFFFFF",
      },
      divider: "#EBEDEF",
    },
  });

  let classes = use_stylesheets!(theme);

  <Stack flex="auto" padding={2} fullHeight alignItems="center" justifyContent="center">
    <AppBar className={classes.appBar} position="sticky" color="inherit" elevation={0}>
      <TextField
        variant="filled"
        fullWidth
        size="small"
        value={filter()}
        onChange={|event| set_filter(event.value)}
        placeholder="Search by key, value, or key:value"
        slotProps={{
          input: {
            startAdornment: (
              <InputAdornment position="start">
                <SearchIcon fontSize="small" />
              </InputAdornment>
            ),
            endAdornment: filter().is_null() && (
              <IconButton edge="end" onClick={clear_filter} size="small">
                <ClearIcon fontSize="small" />
              </IconButton>
            ),
          },
        }}
      />
    </AppBar>
    {events.loading() && (
      <Stack flex="auto" padding={2} fullHeight alignItems="center" justifyContent="center">
        <CircularProgress />
      </Stack>
    )}
    {events.error() && (
      <Stack flex="auto" padding={2} fullHeight alignItems="center" justifyContent="center">
        <Typography align="center" color="error">
          Error loading events.
        </Typography>
      </Stack>
    )}
    {events.value().is_empty() && (
      <Stack flex="auto" padding={2} fullHeight alignItems="center" justifyContent="center">
        <Typography align="center" color="text.secondary">
          No Events
        </Typography>
      </Stack>
    )}
    <div className={classes.grid}>
      {timestamped_events().iter().enumerate().map(|(index, event)| {
        return (
          <EventView
            key={index}
            event={event.clone()}
            filter=filter()
            formatted_time=format_time(event.start())
            // When hovering within the event list only show hover state on directly
            // hovered event.
            is_hovered={
              hovered_event().map_or(false, |hovered| {
                hovered.event.id == event.event.id
                  || events_at_hover_value().contains_key(&event.event.id)
              })
            }
            isSelected={selected_event_id() == event.event.id}
            onClick={onClick}
            on_hover_start={on_hover_start}
            on_hover_end={on_hover_end}
          />
        );
      })}
    </div>
  </Stack>
}

fn use_stylesheets(theme: &mui_core::theme::Theme) -> crate::prelude::Element {
  let styles = createStylesheet!({
    appBar: {
      top: -1,
      zIndex: theme.zIndex.appBar - 1,
      display: "flex",
      flexDirection: "row",
      padding: theme.spacing(1),
      gap: theme.spacing(1),
      alignItems: "center",
      borderBottom: `1px solid ${theme.palette.divider}`,
    },
    grid: {
      display: "grid",
      flexShrink: 1,
      gridTemplateColumns: "auto 1fr",
      overflowY: "auto",
      padding: theme.spacing(1),
    },
    root: {
      backgroundColor: theme.palette.background.paper,
      maxHeight: "100%",
    },
  });

  styles
}
```