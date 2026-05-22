```rust
use crate::components::{MessagePipelineContext, useMessagePipeline};
use crate::context::{
    AppContext, EventsStore, useAppContext, useEvents,
};
use crate::hooks::{formatTime, useAppTimeFormat};
use crate::services::app_module_create_event;
use crate::utils::{count_by, to_nanosec};

#[derive(Default)]
struct EventFormState {
    start_time: Option<chrono::DateTime<chrono::Utc>>,
    duration: Option<f64>,
    duration_unit: String,
    metadata_entries: Vec<(String, String)>,
}

impl EventFormState {
    fn update_metadata(&mut self, index: usize, key_type: &str, value: &str) {
        let mut updated_entries = self.metadata_entries.clone();
        if let Some(entry) = updated_entries.get_mut(index) {
            entry.0 = key_type.to_string();
            entry.1 = value.to_string();

            // Automatically add new row if we're at the end and have both key and value.
            if index == updated_entries.len() - 1 && !entry.0.is_empty() && !entry.1.is_empty() {
                updated_entries.push((String::new(), String::new()));
            }
        }

        self.metadata_entries = updated_entries;
    }

    fn add_row(&mut self, index: usize) {
        let mut updated_entries = self.metadata_entries.clone();
        updated_entries.insert(index + 1, (String::new(), String::new()));
        self.metadata_entries = updated_entries;
    }

    fn remove_row(&mut self, index: usize) {
        if self.metadata_entries.len() > 1 {
            self.metadata_entries.remove(index);
        }
    }

    fn validate_state(&self) -> bool {
        self.start_time.is_some()
            && self.duration.is_some()
            && !self.metadata_entries.contains(&(String::new(), String::new()))
    }
}

pub fn CreateEventDialog(props: &CreateEventDialogProps) -> JSX.Element {
    let onClose = props.onClose;

    let refresh_events = use_events(select_refresh_events);
    let currentTime = use_message_pipeline(select_current_time);
    let event = use_immer(EventFormState::default());
    let device_id = use_events(select_device_id);

    let create_event = use_async_fn(async move {
        if event.start_time.is_none() || event.duration.is_none() || device_id.is_none() {
            return;
        }

        let filtered_meta = event.metadata_entries
            .iter()
            .filter(|(_, value)| !value.is_empty())
            .collect::<Vec<_>>();

        let keyed_metadata = filtered_meta.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect();

        app_module_create_event(&device_id, &event.start_time, to_nanosec(&event.duration), &keyed_metadata);
        onClose();
        refresh_events();
    }, [app_module_create_event, device_id, event, onClose, refresh_events]);

    let { format_time } = use_app_time_format();
    let { create_event: app_module_create_event } = use_app_context();

    let can_submit = event.validate_state();

    let formatted_start_time = currentTime.map_or("-", |dt| format_time(&dt));

    (
        <Dialog open onClose={onClose} fullWidth maxWidth="sm">
            <DialogTitle>Create event</DialogTitle>
            <DialogContent>
                <div className={classes.grid}>
                    <FormControl>
                        <FormLabel>Start Time</FormLabel>
                        <Typography paddingY={1}>{formatted_start_time}</Typography>
                    </FormControl>
                    <TextField
                        value={event.duration.map(|d| format!("{:.3}"), |d| d.to_string()).unwrap_or("-")}
                        fullWidth
                        label="Duration"
                        onChange={(ev) => {
                            let duration = ev.target.value.parse::<f64>().map_err(|_| ()).unwrap_or(0.0);
                            event.update_metadata(0, "duration", &duration.to_string());
                        }}
                        type="number"
                        slotProps={{
                            input: {
                                endAdornment: (
                                    <ToggleButtonGroup
                                        className={classes.toggleButtonGroup}
                                        size="small"
                                        exclusive
                                        value={event.duration_unit}
                                        onChange={|ev, duration_unit| event.update_metadata(0, "duration_unit", &duration_unit)}
                                    >
                                        <ToggleButton className={classes.toggleButton} tabIndex={-1} value="sec">
                                            sec
                                        </ToggleButton>
                                        <ToggleButton className={classes.toggleButton} tabIndex={-1} value="nsec">
                                            nsec
                    </ToggleButtonGroup>
                  </ToggleButton>
                },
            }}
          />
          <ButtonGroup style={{ visibility: "hidden" }}>
            <IconButton tabIndex={-1} data-testid="add">
              <AddIcon />
            </IconButton>
            <IconButton tabIndex={-1}>
              <AddIcon />
            </IconButton>
          </ButtonGroup>
        </div>
        <div>
          <FormLabel>Metadata</FormLabel>
          <div className={classes.grid}>
            {event.metadata_entries.iter().enumerate().map(|(index, (key, value))| {
                let has_duplicate = count_by(&event.metadata_entries).get(key.as_str()).unwrap_or(0) > 1;
                return (
                    <div className={classes.row} key={index}>
                        <TextField
                            fullWidth
                            value={key}
                            autoFocus={index == 0}
                            placeholder="Key (string)"
                            error={has_duplicate}
                            onKeyDown={on_metadataKeyDown}
                            onChange={(ev) => event.update_metadata(index, "key", ev.target.value)}
                        />
                        <TextField
                            fullWidth
                            value={value}
                            placeholder="Value (string)"
                            error={has_duplicate}
                            onKeyDown={on_metadataKeyDown}
                            onChange={(ev) => event.update_metadata(index, "value", ev.target.value)}
                        />
                        <ButtonGroup>
                          <IconButton
                            tabIndex={-1}
                            onClick={() => event.add_row(index)}
                          >
                            <AddIcon />
                          </IconButton>
                          <IconButton
                            tabIndex={-1}
                            onClick={() => event.remove_row(index)}
                            style={{
                              visibility: event.metadata_entries.len() > 1 ? "visible" : "hidden",
                            }}
                          >
                            <RemoveIcon />
                          </IconButton>
                        </ButtonGroup>
                    </div>
                );
            })}
          </div>
        </div>
      </DialogContent>
      {event.metadata_entries.contains(&(String::new(), String::new())) && (
        <Stack paddingX={3}>
          <Alert severity="error">Duplicate key {event.metadata_entries.iter().find(|(_, value)| !value.is_empty()).unwrap_or(&"").0}</Alert>
        </Stack>
      )}
      {create_event.error.map(|err| (
        <Stack paddingX={3}>
          <Alert severity="error">{err.message}</Alert>
        </Stack>
      ))}
      <DialogActions>
        <Button variant="outlined" onClick={onClose}>
          Cancel
        </Button>
        <Button
          variant="contained"
          onClick={create_event}
          disabled={!can_submit || create_event.loading}
        >
          {create_event.loading && (
            <CircularProgress color="inherit" size="1rem" style={{ marginRight: "0.5rem" }} />
          )}
          Create Event
        </Button>
      </DialogActions>
    </Dialog>,
    []
}

struct CreateEventDialogProps {
    onClose: () => void;
}
```