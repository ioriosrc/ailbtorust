// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use std::collections::HashMap;

use crate::time::Time;

/// A topic that messages can be published on.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Topic {
    /// Topic name (e.g., "/camera/image_raw")
    pub name: String,
    /// Schema name / message type (e.g., "sensor_msgs/Image")
    pub schema_name: String,
    /// Encoding format (e.g., "cdr", "ros1msg", "json", "protobuf")
    pub encoding: Option<String>,
    /// Schema encoding (e.g., "ros2msg", "ros2idl", "protobuf", "jsonschema")
    pub schema_encoding: Option<String>,
}

/// Statistics for a single topic.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopicStats {
    /// Total number of messages on this topic.
    pub num_messages: u64,
    /// First message receive time.
    pub first_message_time: Option<Time>,
    /// Last message receive time.
    pub last_message_time: Option<Time>,
}

/// A single message event delivered to panels.
#[derive(Debug, Clone)]
pub struct MessageEvent {
    /// Topic this message was received on.
    pub topic: String,
    /// Time the message was received by the recording system.
    pub receive_time: Time,
    /// Time the message was published (header stamp).
    pub publish_time: Time,
    /// Message size in bytes.
    pub size_in_bytes: usize,
    /// Schema name.
    pub schema_name: String,
    /// Deserialized message data (as serde_json::Value for flexibility).
    pub message: serde_json::Value,
}

/// A field definition within a message schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageField {
    pub name: String,
    pub r#type: String,
    pub is_array: bool,
    pub array_length: Option<u32>,
    pub is_complex: bool,
    pub description: Option<String>,
}

/// A message definition / datatype.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDefinition {
    pub name: String,
    pub fields: Vec<MessageField>,
    /// Definitions for nested types.
    pub definitions: Vec<MessageDefinition>,
}

/// Map of schema name → MessageDefinition (preserves insertion order).
pub type RosDatatypes = IndexMap<String, MessageDefinition>;

/// Subscription request from a panel.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubscribePayload {
    pub topic: String,
    pub schema_name: Option<String>,
    pub encoding: Option<String>,
    /// How to preload data: "partial" (current frame) or "full" (all data).
    pub preload_type: Option<PreloadType>,
}

/// Preload strategy for subscriptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PreloadType {
    /// Only load messages around current playback time.
    Partial,
    /// Preload all messages for this topic.
    Full,
}

/// Publish request to send a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishPayload {
    pub topic: String,
    pub schema_name: String,
    pub message: serde_json::Value,
}

/// Service call request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCallPayload {
    pub service_name: String,
    pub request: serde_json::Value,
}

/// Service call response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCallResponse {
    pub service_name: String,
    pub response: serde_json::Value,
}

/// Represents a ROS parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: ParameterValue,
}

/// Parameter value types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterValue {
    Bool(bool),
    Integer(i64),
    Double(f64),
    String(String),
    ByteArray(Vec<u8>),
    BoolArray(Vec<bool>),
    IntegerArray(Vec<i64>),
    DoubleArray(Vec<f64>),
    StringArray(Vec<String>),
    Undefined,
}

/// Progress information during data loading.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Progress {
    /// Fraction loaded (0.0..1.0) if known.
    pub loading_progress: Option<f64>,
    /// Ranges that have been fully loaded.
    pub fully_loaded_fraction_ranges: Vec<FractionRange>,
    /// Cached message ranges by topic.
    pub message_cache: HashMap<String, Vec<TimeRange>>,
}

/// A range expressed as fractions of the total duration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FractionRange {
    pub start: f64,
    pub end: f64,
}

/// A range of time.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: Time,
    pub end: Time,
}

/// Alert severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
}

/// An alert/notification from the player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAlert {
    pub severity: AlertSeverity,
    pub message: String,
    pub tip: Option<String>,
}

/// Variable value that can be used in message path expressions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VariableValue {
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<VariableValue>),
}

/// Global variables accessible to all panels.
pub type GlobalVariables = HashMap<String, VariableValue>;
