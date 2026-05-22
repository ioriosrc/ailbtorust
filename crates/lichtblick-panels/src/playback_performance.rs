// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

/// Playback performance panel shows FPS, message rate, etc.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlaybackPerformanceConfig {}

/// Performance metrics.
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub fps: f64,
    pub message_rate: f64,
    pub messages_per_frame: f64,
    pub frame_time_ms: f64,
    pub render_time_ms: f64,
}
