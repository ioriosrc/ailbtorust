// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use leptos::prelude::*;

/// Hook providing access to the message pipeline from panels.
///
/// Equivalent to `useMessagePipeline` in the TypeScript version.
pub fn use_message_pipeline() -> MessagePipelineContext {
    expect_context::<MessagePipelineContext>()
}

/// Context provided by the message pipeline.
#[derive(Clone, Copy)]
pub struct MessagePipelineContext {
    pub is_playing: RwSignal<bool>,
    pub current_time: RwSignal<f64>,
}
