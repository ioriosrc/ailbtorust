```rust
use lighthouse::{Time, Immutable, MessageEvent, Metadata, ParameterValue};
use lighthouse_base::players::types::{
  AdvertiseOptions, PlayerState, PublishPayload, SubscribePayload, Topic,
};
use lighthouse_base::players::IterablePlayer as IPlayer;
use std::collections::HashMap;

type ResumeFrame = () -> ();

pub type MessagePipelineContext = Immutable<{
  player_state: PlayerState,
  sorted_topics: Vec<Topic>,
  sorted_services: Vec<String>,
  datatypes: RosDatatypes,
  subscriptions: Vec<SubscribePayload>,
  message_events_by_subscriber_id: HashMap<String, Vec<MessageEvent>>,
  set_subscriptions: fn(&mut Self, &str, Immutable<Vec<SubscribePayload>>),
  set_publishers: fn(&mut Self, &str, Vec<AdvertiseOptions>),
  set_parameter: fn(&mut Self, &str, ParameterValue),
  publish: fn(&mut Self, &PublishPayload),
  get_metadata: fn() -> Vec<Metadata>,
  call_service: fn(&self, &str, &dyn std::any::Any) -> std::result::Result<(), String>,
  fetch_asset: fn(&self) -> Box<dyn std::future::Future<Item = ()>>,
  start_playback: Option<Box<dyn std::future::Future<Item = ()>>>,
  pause_playback: Option<Box<dyn std::future::Future<Item = ()>>>,
  play_until: Option<Box<dyn std::future::Future<Item = Time>>>,
  set_playback_speed: Option<Box<dyn std::future::Future<Item = ()>>>,
  seek_playback: Option<Box<dyn std::future::Future<Item = Time>>>,
  pause_frame: fn(&mut Self, &str) -> ResumeFrame,
  get_batch_iterator: Option<Box<dyn std::future::Future<Item = Vec<IteratorResult>>>>,
}>;
```