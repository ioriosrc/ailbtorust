// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LichtblickError {
    #[error("Player error: {0}")]
    Player(String),

    #[error("Data source error: {0}")]
    DataSource(String),

    #[error("MCAP error: {0}")]
    Mcap(String),

    #[error("Schema error: {0}")]
    Schema(String),

    #[error("Layout error: {0}")]
    Layout(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, LichtblickError>;
