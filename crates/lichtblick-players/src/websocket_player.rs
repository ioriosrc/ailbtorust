// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use lichtblick_core::error::LichtblickError;
use lichtblick_core::player::*;
use lichtblick_core::time::Time;
use lichtblick_core::types::*;

use crate::traits::{Player, PlayerListener};

/// Player that connects to a Foxglove WebSocket server for live data.
pub struct FoxgloveWebSocketPlayer {
    url: String,
    name: String,
    player_id: String,
    listener: Option<PlayerListener>,
    subscriptions: Vec<SubscribePayload>,
    is_connected: bool,
    topics: Vec<Topic>,
    datatypes: RosDatatypes,
}

impl FoxgloveWebSocketPlayer {
    pub fn new(url: String) -> Self {
        Self {
            name: format!("WebSocket: {}", url),
            url,
            player_id: uuid::Uuid::new_v4().to_string(),
            listener: None,
            subscriptions: Vec::new(),
            is_connected: false,
            topics: Vec::new(),
            datatypes: RosDatatypes::new(),
        }
    }

    /// Connect to the WebSocket server.
    /// In WASM, this uses the browser's WebSocket API via web-sys.
    #[cfg(target_arch = "wasm32")]
    pub async fn connect(&mut self) -> Result<(), LichtblickError> {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use web_sys::WebSocket;

        let ws = WebSocket::new(&self.url)
            .map_err(|e| LichtblickError::WebSocket(format!("Failed to create WebSocket: {:?}", e)))?;

        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        // Set up message handler
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MessageEvent| {
            // Handle incoming messages from foxglove websocket protocol
            // This would parse the binary protocol and dispatch messages
            log::debug!("WebSocket message received");
        });
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        // Set up open handler
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            log::info!("WebSocket connected");
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        self.is_connected = true;
        self.emit_state();
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn connect(&mut self) -> Result<(), LichtblickError> {
        Err(LichtblickError::WebSocket(
            "WebSocket player only supported in WASM target".into(),
        ))
    }

    fn emit_state(&self) {
        if let Some(listener) = &self.listener {
            let presence = if self.is_connected {
                PlayerPresence::Present
            } else {
                PlayerPresence::Initializing
            };

            let active_data = if self.is_connected {
                Some(ActiveData {
                    messages: Vec::new(),
                    current_time: Time::ZERO,
                    start_time: Time::ZERO,
                    end_time: Time::ZERO,
                    is_playing: true,
                    speed: 1.0,
                    topics: self.topics.clone(),
                    datatypes: self.datatypes.clone(),
                    topic_stats: std::collections::HashMap::new(),
                })
            } else {
                None
            };

            let state = PlayerState {
                presence,
                progress: Progress::default(),
                capabilities: vec![PlayerCapability::Publish, PlayerCapability::CallServices],
                profile: Some("foxglove-websocket".to_string()),
                player_id: self.player_id.clone(),
                alerts: Vec::new(),
                active_data,
            };

            listener(state);
        }
    }
}

impl Player for FoxgloveWebSocketPlayer {
    fn set_listener(&mut self, listener: PlayerListener) {
        self.listener = Some(listener);
        self.emit_state();
    }

    fn set_subscriptions(&mut self, subscriptions: Vec<SubscribePayload>) {
        self.subscriptions = subscriptions;
        // In a full implementation, this would send subscribe messages over the WebSocket
    }

    fn start_playback(&mut self) {
        // Live sources are always "playing"
    }

    fn pause_playback(&mut self) {
        // Live sources can't be paused
    }

    fn seek_playback(&mut self, _time: Time) {
        // Live sources can't seek
    }

    fn set_playback_speed(&mut self, _speed: f64) {
        // Live sources always play at 1x
    }

    fn publish(&mut self, payload: PublishPayload) -> Result<(), LichtblickError> {
        if !self.is_connected {
            return Err(LichtblickError::Player("Not connected".into()));
        }
        // In full implementation, serialize and send over WebSocket
        log::debug!("Publishing to topic: {}", payload.topic);
        Ok(())
    }

    fn call_service(
        &mut self,
        payload: ServiceCallPayload,
    ) -> Result<ServiceCallResponse, LichtblickError> {
        if !self.is_connected {
            return Err(LichtblickError::Player("Not connected".into()));
        }
        // In full implementation, send service call over WebSocket and await response
        log::debug!("Calling service: {}", payload.service_name);
        Err(LichtblickError::Player("Service calls not yet implemented".into()))
    }

    fn close(&mut self) {
        self.is_connected = false;
    }

    fn name(&self) -> &str {
        &self.name
    }
}
