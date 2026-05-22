```rust
use leptos::prelude::*;

fn use_seek_time_from_cli() {
  let (player_state, mut set_player_state) = use_message_pipeline_getter();
  let (time, mut set_time) = use_app_parameters();

  useEffect(async move {
    if time.is_none() || seek_playback.is_none() {
      return;
    }

    // Wait until player is ready before we try to seek.
    if player_state.presence != PlayerPresence::PRESENT {
      return;
    }

    let parsed_time = parse_timestamp_str(time.as_ref());

    if parsed_time.is_err() {
      // Show an error message to the user indicating that the time format is invalid
      set_message(
        "Invalid time format using 'time' parameter on CLI. Please check and try again.",
        MessageType::Warning,
      );
      return;
    }

    seek_playback(parsed_time.unwrap());
  }, [time, seek_playback, player_state.presence]);
}
```