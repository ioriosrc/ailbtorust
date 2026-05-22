```rust
use std::sync::Arc;
use std::time::{Duration, Instant};
use url::Url;

use crate::state::AppURLState;
use crate::utils::{update_app_url_state, decode_uri_component};

pub fn use_state_to_urlsynchronization() {
    let player_url_state = use_message_pipeline(select_player_url_state);
    let stable_player_url_state = use_deep_mem(player_url_state);
    let can_seek = use_message_pipeline(select_can_seek);
    let current_time = use_message_pipeline(select_current_time);
    let debounced_current_time = use_debounce(current_time, Duration::from_millis(500), 500);

    // Sync current time with the url.
    if let Some(debounced_current_time) = debounced_current_time {
        update_url_state(AppURLState {
            time: can_seek && Some(debounced_current_time),
        });
    }

    // Sync player state with the url.
    use_events(move |events| {
        let selected_event_id = events.selected_event_id;
        if stable_player_url_state.is_some() {
            update_url_state(AppURLState {
                ds: stable_player_url_state.as_ref().unwrap().source_id,
                ds_params: stable_player_url_state.as_ref().unwrap()
                    .parameters
                    .iter()
                    .filter_map(|(key, value)| {
                        if key == "eventId" {
                            Some(value.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<std::collections::HashMap<String, String>>(),
                ds_params_array: stable_player_url_state.as_ref().unwrap()
                    .parameters
                    .iter()
                    .filter_map(|(key, value)| {
                        if value.is_array() {
                            Some(value.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<std::collections::HashMap<String, Vec<String>>>(),
                selected_event_id,
            });
        }
    });
}

fn select_player_url_state(ctx: Arc<MessagePipelineContext>) -> Arc<MessagePipelineState> {
    ctx.player_state.as_ref().clone()
}

fn select_can_seek(ctx: Arc<MessagePipelineContext>) -> bool {
    ctx.player_state.capabilities.contains(PLAYER_CAPABILITIES.playback_control)
}

fn select_current_time(ctx: Arc<MessagePipelineContext>) -> Option<f64> {
    ctx.player_state.active_data.map(|data| data.current_time)
}

use use_debounce::use_debounce;
use use_events::use_events;
use crate::state::MessagePipelineState;
```