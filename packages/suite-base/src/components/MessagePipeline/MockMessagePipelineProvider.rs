```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2018-2021 Cruise LLC

use async_std::sync::Condvar;
use chrono::{Duration as ChronoDuration, Utc};
use crossbeam_channel::unbounded;

use crate::{
    frame::FramePromise,
    message_pipeline::MessagePipelineInternalState,
    player::constants::PLAYER_CAPABILITIES,
    player::types::{
        PublishPayload, SubscribePayload, Topic, TopicStats, Metadata, ParameterValue, PlayerState,
        Progress, PlayerURLState, Time,
    },
};

pub type MockMessagePipelineProps = {
    name: Option<String>,
    presence: PlayerPresence,
    topics: Option<Vec<crate::message_pipeline::Topic>>,
    services: Option<Vec<&'static str>>,
    topicStats: Option<Map<String, TopicStats>>,
    datatypes: RosDatatypes,
    messages: Option<Vec<crate::message_pipeline::MessageEvent>>,
    alerts: Option<Vec<crate::player::PlayerAlert>>,
    publish: Option<fn(PublishPayload)>,
    call_service: Option<fn(&'static str, &dyn std::any::Any) -> async_std::task::Result<(), crate::player::PlayerError>>>,
    setPublishers: Option<fn(&'static str, &[crate::message_pipeline::AdvertiseOptions])>,
    setSubscriptions: Option<fn(&'static str, Vec<crate::message_pipeline::SubscribePayload>)>,
    setParameter: Option<fn(&str, &dyn std::any::Any)>,
    fetch_asset: Option<BuiltinPanelExtensionContext>,
    no_active_data: bool,
    active_data: Option<Partial<PlayerStateActiveData>>,
    capabilities: Option<Vec<&'static str>>,
    profile: Option<String>,
    start_playback: fn(),
    pause_playback: fn(),
    seek_playback: fn(&Duration),
    get_current_time: fn() -> Time,
    current_start_time: fn() -> Option<Time>,
    current_end_time: fn() -> Option<Time>,
    is_playing: fn() -> bool,
    pause_frame: Option<Box<dyn Fn(&'static str) -> Box<dyn FnOnce()>>>,
    player_id: Option<&str>,
    progress: Option<Progress>,
    url_state: Option<PlayerURLState>,
};

pub type MockMessagePipelineState = MessagePipelineInternalState;

fn get_public_state(
    previous_state: &MockMessagePipelineState,
    props: MockMessagePipelineProps,
    dispatch: fn(MessagePipelineStateAction),
    promises_to_wait_for: Arc<Mutex<Vec<FramePromise>>>,
) -> Omit<MessagePipelineInternalState, "message_events_by_subscriber_id"> {
    let mut startTime = previous_state.public.player_state.active_data.as_ref().map(|a| a.start_time);
    let mut currentTime = props.current_time;
    if startTime.is_none() || !currentTime.is_some() {
        for message in props.messages.unwrap_or_default() {
            if startTime.is_none() || message.receive_time < *startTime.unwrap_or(&Utc::now()) {
                startTime = Some(message.receive_time.clone());
            }
            if !currentTime.is_some() || message.receive_time < *currentTime.unwrap_or(&Utc::now()) {
                currentTime = Some(message.receive_time.clone());
            }
        }
    }

    let mut public_state = get_public_state_internal(
        previous_state,
        props,
        dispatch,
        promises_to_wait_for,
        startTime,
        currentTime,
    );

    if !props.no_active_data.unwrap_or_default() {
        let mut active_data = public_state.public.active_data.as_mut().unwrap();
        active_data.last_seek_time = (active_data.last_seek_time.unwrap_or(0) + 1).clamp(1, u64::MAX);

        // mimic seek backfill behavior that happens after new subscribers are added.
        // note that for the mock use-case that this will only happen the first time subscribers are added, since we don't reset `has_subscribers`
        if props.active_data.is_some() {
            active_data = &mut props.active_data.unwrap();
        }

        public_state.public = public_state.public.map(|p| {
            p.clone().with_active_data(active_data.clone())
        });
    }

    public_state
}

fn get_public_state_internal(
    previous_state: &MockMessagePipelineState,
    props: MockMessagePipelineProps,
    dispatch: fn(MessagePipelineStateAction),
    promises_to_wait_for: Arc<Mutex<Vec<FramePromise>>>,
    start_time: Option<Utc>,
    current_time: Option<Utc>,
) -> MessagePipelineInternalState {
    let has_subscribers = previous_state.subscriptions_by_topic.is_empty();
    let new_topics_by_subscriber_id = Arc::new(Mutex::new(HashMap::<String, Vec<&'static str>>::new()));
    let subscribers = Arc::new(Mutex::new(
        HashMap::<&'static str, HashSet<String>>>::new(),
    ));

    MessagePipelineInternalState {
        mock_props: props,
        player: None,
        dispatch,
        reset,
        subscription_memorizer: make_subscription_memoizer(subscribers.clone()),
        publishers_by_id: Arc::new(Mutex::new(HashMap::<&'static str, Vec<crate::message_pipeline::AdvertiseOptions>>::new())),
        all_publishers: Arc::new(Mutex::new(vec![])),
        subscriptions_by_topic: new_topics_by_subscriber_id,
        subscriber_ids_by_topic: subscribers,
        new_topics_by_subscriber_id,
        last_message_event_by_topic: Arc::new(Mutex::new(HashMap::<&'static str, Option<crate::message_pipeline::MessageEvent>>>())),
        last_capabilities: previous_state.public.player_state.capabilities.clone(),
        public: {
            let mut public = MessagePipelineInternalStatePublic {
                player_state: previous_state.public.player_state.clone(),
                message_events_by_subscriber_id: HashMap::<&'static str, Vec<crate::message_pipeline::MessageEvent>>::new(),
            };

            if !has_subscribers {
                dispatch(MessagePipelineStateAction::UpdatePlayerState(previous_state.public.player_state));
            }

            public
        },
    }
}

fn make_subscription_memoizer(subscribers: Arc<Mutex<HashMap<&'static str, HashSet<String>>>>) -> crate::message_pipeline::SubscriptionMemoizer {
    let subscribers = subscribers.clone();
    crate::message_pipeline::SubscriptionMemoizer(move |topic| {
        subscribers.lock().unwrap().entry(topic).or_default().clone()
    })
}

#[derive(Debug)]
struct MockMessagePipelineState {
    mock_props: MockMessagePipelineProps,
    player: Option<MockPlayer>,
    dispatch: fn(MessagePipelineStateAction),
    reset: fn(),
    subscription_memorizer: crate::message_pipeline::SubscriptionMemoizer,
    publishers_by_id: Arc<Mutex<HashMap<&'static str, Vec<crate::message_pipeline::AdvertiseOptions>>>>,
    all_publishers: Arc<Mutex<Vec<crate::message_pipeline::AdvertiseOptions>>>>,
    subscriptions_by_topic: Arc<Mutex<HashMap<String, Vec<&'static str>>>>,
    subscriber_ids_by_topic: Arc<Mutex<HashMap<&'static str, HashSet<String>>>>,
    new_topics_by_subscriber_id: Arc<Mutex<HashMap<String, Vec<&'static str>>>>,
    last_message_event_by_topic: Arc<Mutex<HashMap<&'static str, Option<crate::message_pipeline::MessageEvent>>>>>,
    last_capabilities: Vec<&'static str>,
    public: MessagePipelineInternalStatePublic,
}

#[derive(Debug)]
struct MockPlayer {
    // Player-specific state and methods
}

impl MessagePipelineStateAction for MockMessagePipelineState {
    fn update_player_state(&mut self, player_state: crate::message_pipeline::PlayerState) {
        if let Some(ref mut player) = &mut self.player {
            player.update(player_state);
        } else {
            // Handle the case where there's no player yet
        }
    }

    fn reset(&mut self) {
        // Reset the state of the message pipeline
    }
}

#[derive(Debug)]
struct MessagePipelineInternalStatePublic {
    pub player_state: crate::message_pipeline::PlayerState,
    message_events_by_subscriber_id: HashMap<&'static str, Vec<crate::message_pipeline::MessageEvent>>,
}
```