```rust
use std::sync::{Arc, Mutex};
use web_sys::EventEmitter;
use web_sys::{VideoDecoder, VideoDecoderInit, EncodedVideoChunk};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub type VideoPlayerEventTypes = {
    frame: fn(&VideoFrame),
    debug: fn(&str),
    warn: fn(&str),
    error: fn(&Error),
};

const MAX_DECODE_WAIT_MS = 30;

// foxglove-depcheck-used: @types/dom-webcodecs

export struct VideoPlayer {
    decoder_init: VideoDecoderInit,
    decoder: VideoDecoder,
    decoder_config: Option<VideoDecoderConfig>,
    mutex: Mutex<()>,
    timeout_id: Option<web_sys::TimeoutHandle>,
    pending_frame: Option<VideoFrame>,
    coded_size: Option<{ width: i32; height: i32 }>,
}

impl VideoPlayer {
    pub fn is_supported() -> bool {
        let is_secure_context = web_sys::window().unwrap().is_secure_context();
        is_secure_context && web_sys::window().unwrap().get_property("VideoDecoder").is_ok()
    }

    pub fn new() -> Self {
        Self {
            decoder_init: VideoDecoderInit {
                output: |video_frame| {
                    self.pending_frame.take().unwrap().close();
                    self.pending_frame = Some(video_frame);
                    self.emit("frame", video_frame);
                },
                error: |error| self.emit("error", error),
            },
            decoder: VideoDecoder::new(&self.decoder_init).unwrap(),
            decoder_config: None,
            mutex: Mutex::new(()),
            timeout_id: None,
            pending_frame: None,
            coded_size: None,
        }
    }

    pub async fn init(&mut self, decoder_config: VideoDecoderConfig) -> Result<(), Error> {
        let decoder_config = match &self.decoder_config {
            Some(c) if *c == decoder_config => return Ok(()),
            _ => decoder_config.clone(),
        };

        log::info!("Configuring VideoDecoder with {}", serde_json::to_string(&decoder_config).unwrap());

        self.mutex.lock().await;

        if let Err(err) = self.decoder.configure(&decoder_config) {
            log::warn!("VideoDecoder does not support configuration {:?}", err);
            return Err(Error::from("VideoDecoder does not support configuration"));
        }

        self.decoder_config = Some(decoder_config);

        if let Err(err) = self.decoder.decode(EncodedVideoChunk {
            type_: "key".to_string(),
            data: vec![],
            timestamp: 0,
        }) {
            log::warn!("Failed to decode chunk at time 0: {:?}", err);
            return Err(Error::from("Failed to decode chunk"));
        }

        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.decoder.state() == "configured"
    }

    pub fn decoder_config(&self) -> Option<VideoDecoderConfig> {
        self.decoder_config.clone()
    }

    pub fn coded_size(&self) -> Option<{ width: i32; height: i32 }> {
        self.coded_size.clone()
    }

    pub async fn decode(
        &mut self,
        data: Vec<u8>,
        timestamp_micros: u64,
        type_: &str,
    ) -> Result<Option<VideoFrame>, Error> {
        let maybe_video_frame = match self.mutex.lock().await {
            Ok(mutex) => Some(self.decoder.decode(EncodedVideoChunk {
                type_: type_.to_string(),
                data: data.clone(),
                timestamp: timestamp_micros,
            })?),
            Err(_) => None,
        };

        if let Some(video_frame) = maybe_video_frame {
            log::info!("Received decoded VideoFrame with size {:?}", video_frame.coded_width(), video_frame.coded_height());
            self.emit("frame", &video_frame);
        }

        Ok(maybe_video_frame)
    }

    pub fn reset_for_seek(&mut self) {
        if let Some(decoder_config) = self.decoder_config() {
            self.decoder.reset();
        }
        if let Some(timeout_id) = self.timeout_id.take() {
            web_sys::window().unwrap().clear_timeout_with_handle(timeout_id);
        }
        self.pending_frame.take();
    }

    pub fn close(&mut self) {
        if let Err(err) = self.decoder.close() {
            log::warn!("Failed to close VideoDecoder: {:?}", err);
        }
        self.reset_for_seek();
    }

    async fn emit<E>(&self, event_name: &str, event_data: E)
    where
        E: serde::Serialize + 'static,
    {
        if let Ok(mut emitter) = EventEmitter::from(web_sys::window().unwrap()) {
            emitter.emit(event_name, event_data);
        }
    }
}
```