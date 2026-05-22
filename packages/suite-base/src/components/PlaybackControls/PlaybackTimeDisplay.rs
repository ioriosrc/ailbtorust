```rust
use crate::components::{MessagePipelineContext, UnconnectedPlaybackTimeDisplay};
use crate::hooks::{AppConfigurationValue, useAppTimeFormat, useMessagePipeline};
use chrono::Duration;
use chrono::DateTime;
use log::{debug, error};

type Props = {
  onSeek: fn(Duration),
  onPause: fn(),
};

fn select_is_playing(ctx: &MessagePipelineContext) -> bool {
  ctx.player_state.active_data.is_playing.unwrap_or(false)
}

fn select_start_time(ctx: &MessagePipelineContext) -> Option<DateTime<chrono::Local>> {
  ctx.player_state.active_data.start_time.as_ref()
}

fn select_end_time(ctx: &MessagePipelineContext) -> Option<DateTime<chrono::Local>> {
  ctx.player_state.active_data.end_time.as_ref()
}

fn select_current_time(ctx: &MessagePipelineContext) -> Option<DateTime<chrono::Local>> {
  ctx.player_state.active_data.current_time.as_ref()
}

pub fn PlaybackTimeDisplay(props: Props): crate::prelude::Element<'_> {
  let timezone = AppConfigurationValue::get(AppSetting::TIMEZONE).unwrap_or("UTC".to_string());

  let is_playing = use_message_pipeline(select_is_playing);
  let start_time = use_message_pipeline(select_start_time);
  let end_time = use_message_pipeline(select_end_time);
  let current_time = use_message_pipeline(select_current_time);

  <UnconnectedPlaybackTimeDisplay
    app_time_format=crate::prelude::use_app_time_format()
    current_time={current_time.map(|dt| dt.format("%H:%M:%S %Z").to_string())}
    start_time={start_time.map(|dt| dt.format("%H:%M:%S %Z").to_string())}
    end_time={end_time.map(|dt| dt.format("%H:%M:%S %Z").to_string())}
    on_seek={props.on_seek}
    onPause={props.onPause}
    is_playing={is_playing.unwrap_or(false)}
    timezone={timezone}
  />
}
```