```rust
use chrono::{DateTime, Utc};
use crate::log::Log;
use crate::rostime::{compare, Time};
use crate::suite_base::hooks::use_global_variables;
use crate::suite_base::panels::three_dee_render::normalize_messages::normalize_frame_transform;
use crate::suite_base::players::IterablePlayer::IIterableSource;
use crate::suite_base::players::Player;
use crate::suite_base::players::constants::*;
use crate::suite_base::players::types::{AdvertiseOptions, BlockCache, MessageBlock, PlayerState, PublishPayload, SubscribePayload, Topic, TopicStats};
use crate::suite_base::players::util::delay;
use crate::suite_base::types::RosDatatypes;

struct TransformPreloadingPlayer {
    name: String,
    listener: Option<Box<dyn Fn(PlayerState) -> Promise<()>>>,
    datatypes: RosDatatypes,
    startTime: Time,
    endTime: Time,
    topic_stats: std::collections::HashMap<String, TopicStats>,
    topics: Vec<Topic>,
}

impl Player for TransformPreloadingPlayer {
    fn get_batch_iterator(
        &self,
        _topic: String,
    ) -> Option<std::iter::Iterator<Readonly<IteratorResult>>> {
        None
    }

    fn set_listener(&mut self, listener: Box<dyn Fn(PlayerState) -> Promise<()>>) {
        self.listener = Some(listener);
    }
    fn close(&mut self) {}
    fn set_subscriptions(&mut self, _subs: SubscribePayload[]) {}
    fn set_publishers(&mut self, _publishers: AdvertiseOptions[]) {}
    fn set_parameter(_key: String, _value: unknown) {
        unimplemented!();
    }
    fn publish(_request: PublishPayload) {
        unimplemented!();
    }
    fn call_service(_service: String, _request: unknown) -> Promise<unknown> {
        unimplemented!();
    }
    fn set_global_variables(_globalVariables: GlobalVariables) {
        unimplemented!();
    }

    async fn #run(&mut self) {
        let listener = match &self.listener {
            Some(listener) => listener,
            None => panic!("Invariant: listener is not set"),
        };

        log::info!("Initializing transform preloading player");

        await listener(PlayerState {
            profile: None,
            presence: PlayerPresence::PRESENT,
            name: self.name.clone(),
            playerId: self.name.clone(),
            capabilities: CAPABILITIES.clone(),
            progress: {},
            url_state: {
                source_id: self.name.clone(),
            },
        });

        await listener(PlayerState {
            profile: None,
            presence: PlayerPresence::INITIALIZING,
            name: format!("{} getting messages", self.name),
            playerId: self.name.clone(),
            capabilities: CAPABILITIES.clone(),
            progress: {},
            active_data: ActiveData {
                messages: vec![],
                total_bytes_received: 0,
                start_time: self.startTime.clone(),
                end_time: self.endTime.clone(),
                current_time: self.startTime.clone(),
                is_playing: false,
                speed: 1,
                last_seek_time: Utc::now().timestamp() as f64,
                topics: self.topics.clone(),
                topic_stats: self.topic_stats.clone(),
                datatypes: self.datatypes.clone(),
            },
        });

        log::info!("Preloading messages");
        let start = Instant::now();
        let msgs100hz = get_tf_messages(Self {
            tf_params: TFParams {
                frequency_hz: 100,
                parent: "base_link".to_string(),
                axis: "x".to_string(),
                translation: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            },
            ..self
        });
        let msgs150hz = get_tf_messages(Self {
            tf_params: TFParams {
                frequency_hz: 150,
                parent: "100hz".to_string(),
                axis: "z".to_string(),
                translation: Vector3 { x: 0.0, y: 0.0, z: 1.0 },
            },
            ..self
        });

        let all_messages = msgs100hz.iter().chain(msgs150hz.iter()).cloned().collect::<Vec<MessageEvent<FrameTransform>>>();        
        all_messages.sort_by(|a, b| compare(a.receive_time(), b.receive_time()));

        let num_messages = all_messages.len();
        let mut seek_frames_ms: Vec<f64> = vec![];

        for _ in 0..10 {
            let seek_to_message = all_messages.iter().random::<usize>().cloned();
            let start_frame = Instant::now();
            await listener(PlayerState {
                active_data: ActiveData {
                    messages: vec![seek_to_message.clone()],
                    total_bytes_received: 1,
                    start_time: self.startTime.clone(),
                    end_time: self.endTime.clone(),
                    current_time: seek_to_message.receive_time(),
                    is_playing: false,
                    speed: 1.0,
                    last_seek_time: Utc::now().timestamp() as f64,
                    topics: self.topics.clone(),
                    topic_stats: self.topic_stats.clone(),
                    datatypes: self.datatypes.clone(),
                },
            });
            let end_frame = Instant::now();
            seek_frames_ms.push(end_frame.duration_since(start_frame).as_secs_f64());
        }

        log::info!("Number of messages: {}", num_messages);
        log::info!(
            "Seek frame average times from beginning to end of playtime. Should remain generally constant:\n {}ms, ",
            seek_frames_ms.iter().map(|&m| m.to_string()).join("ms, ")
        );

        await delay(1000);

        for _ in 0..10 {
            let seek_to_message = all_messages.iter().random::<usize>().cloned();
            let start_frame = Instant::now();
            await listener(PlayerState {
                active_data: ActiveData {
                    messages: vec![seek_to_message.clone()],
                    total_bytes_received: 1,
                    start_time: self.startTime.clone(),
                    end_time: self.endTime.clone(),
                    current_time: seek_to_message.receive_time(),
                    is_playing: false,
                    speed: 1.0,
                    last_seek_time: Utc::now().timestamp() as f64,
                    topics: self.topics.clone(),
                    topic_stats: self.topic_stats.clone(),
                    datatypes: self.datatypes.clone(),
                },
            });
            let end_frame = Instant::now();
            seek_frames_ms.push(end_frame.duration_since(start_frame).as_secs_f64());
        }

        log::info!(
            "Seek frame average times from end to beginning of playtime. Should start high and decrease:\n {}ms, ",
            seek_frames_ms.iter().map(|&m| m.to_string()).join("ms, ")
        );
    }
}

struct TFParams {
    frequency_hz: f64,
    parent: String,
    axis: &'static str,
    translation: Vector3,
}

const quat_identity: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

fn get_tf_messages(player: TransformPreloadingPlayer) -> Vec<MessageEvent<FrameTransform>> {
    let mut messages = Vec::new();
    let start_time_ns = player.startTime.nsec as u64;
    let end_time_ns = player.endTime.nsec as u64;

    for i in 0..10_000 { // Adjust number of messages based on required playtime
        let frame_tf = normalize_frame_transform(None);
        frame_tf.child_frame_id = format!("{}hz", player.tf_params.frequency_hz);
        frame_tf.parent_frame_id = player.tf_params.parent.clone();
        frame_tf.rotation = quat_identity;
        frame_tf.translation = Vector3 {
            x: (i % 10) as f64,
            y: i / 10.0,
            z: (i % 2) as f64,
        };

        let log_time_ns = start_time_ns + i * player.tf_params.frequency_hz * 1e9;
        frame_tf.timestamp = DateTime::from_utc(Utc.timestamp_millis(log_time_ns), Utc).into();

        messages.push(MessageEvent {
            receive_time: frame_tf.timestamp.clone(),
            topic: "your_topic".to_string(),
            schema_name: "foxglove.FrameTransform",
            size_in_bytes: 86 + frame_tf.parent_frame_id.len() + frame_tf.child_frame_id.len(),
            message: frame_tf,
        });
    }

    messages
}
```