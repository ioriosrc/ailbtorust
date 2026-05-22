// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Message path parsing and evaluation.
//!
//! Supports paths like:
//! - `/topic.field.nested_field`
//! - `/topic.array[0]`
//! - `/topic.array[5:10]`
//! - `/topic.field{value==5}`
//! - `/topic.field[${variable}]`

pub mod parser;
pub mod evaluator;

pub use parser::{MessagePath, MessagePathPart, FilterCondition, parse_message_path};
pub use evaluator::evaluate_message_path;
