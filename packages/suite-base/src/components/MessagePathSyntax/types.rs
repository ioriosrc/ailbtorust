```rust
use std::collections::{HashMap, HashSet};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use mui::material::{TextFieldProps, CSSProperties};
use lichtblick::message_path::src::types::*;
use lichtblick::suite_base::players::types::*;

pub type Options = {
  history_size: Option<usize>,
  sampling_request: SubscribePayload["samplingRequest"],
};

pub type ReducedValue = {
  // Matched message (events) oldest message first
  matches: Vec<MessageAndData>;

  // The latest set of message events recevied to addMessages
  message_events: Vec<&MessageEvent>;

  // The path used to match these messages.
  path: String;
};

pub type MessagePathsForStructureArgs = {
  valid_types: Option<Vec<&str>>,
  no_multi_slices: bool,
  message_path: Option<Vec<MessagePathPart>>,
};

pub type MessagePathsForStructure = Vec<(String, MessagePathStructureItem)>;

pub type MessagePathInputBaseProps = {
  supports_math_modifiers: bool,
  path: String, // A path of the form `/topic.some_field[:]{id==42}.x`
  index: Option<usize>, // Optional index field which gets passed to `onChange` (so you don't have to create anonymous functions)
  onChange: fn(&str, Option<usize>),
  valid_types: Option<Vec<&str>>, // Valid types, like "message", "array", or "primitive", or a ROS primitive like "float64"
  no_multi_slices: bool, // Don't suggest slices with multiple values `[:]`, only single values like `[0]`.
  placeholder: String,
  input_style: Option<CSSProperties>,
  disabled: bool,
  disable_autocomplete: bool, // Treat this as a normal input, with no autocomplete.
  readOnly: bool,
  prioritized_datatype: Option<&str>,
  variant: TextFieldProps["variant"],
};

pub type StructureTraversalResult = {
  valid: bool;
  msg_path_part: Option<MessagePathPart>;
  structure_item: Option<MessagePathStructureItem>;
};

pub type StructureAllItemsByPathProps = {
  no_multi_slices: bool,
  valid_types: Option<Vec<&str>>,
  message_path_structures_for_dataype: HashMap<String, MessagePathStructureItemMessage>,
  topics: Vec<Topic>,
};
```