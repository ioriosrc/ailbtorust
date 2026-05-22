// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid message path: {0}")]
    InvalidPath(String),
    #[error("Unexpected token '{0}' at position {1}")]
    UnexpectedToken(String, usize),
    #[error("Unterminated filter at position {0}")]
    UnterminatedFilter(usize),
}

/// Represents a parsed message path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessagePath {
    /// The topic name (without leading /).
    pub topic_name: String,
    /// Path parts after the topic.
    pub parts: Vec<MessagePathPart>,
}

/// A single part of a message path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessagePathPart {
    /// Access a named field: `.field_name`
    Field(String),
    /// Array index: `[0]`
    Index(i64),
    /// Array slice: `[start:end]`
    Slice { start: Option<i64>, end: Option<i64> },
    /// Filter: `{field==value}`
    Filter(FilterCondition),
    /// Variable reference: `[$variable]`
    Variable(String),
}

/// A filter condition within a message path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterCondition {
    pub field: String,
    pub operator: FilterOperator,
    pub value: FilterValue,
}

/// Filter operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
}

/// Filter comparison value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterValue {
    Number(f64),
    String(String),
    Bool(bool),
}

static TOPIC_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^/[a-zA-Z_][a-zA-Z0-9_/]*").unwrap()
});

/// Parse a message path string into a structured MessagePath.
///
/// # Examples
/// ```
/// use lichtblick_messages::parse_message_path;
///
/// let path = parse_message_path("/camera/image.header.stamp.sec").unwrap();
/// assert_eq!(path.topic_name, "camera/image");
/// assert_eq!(path.parts.len(), 3);
/// ```
pub fn parse_message_path(input: &str) -> Result<MessagePath, ParseError> {
    let input = input.trim();
    if !input.starts_with('/') {
        return Err(ParseError::InvalidPath(
            "Message path must start with '/'".into(),
        ));
    }

    // Extract topic name
    let topic_match = TOPIC_REGEX
        .find(input)
        .ok_or_else(|| ParseError::InvalidPath("Invalid topic name".into()))?;

    let topic_name = topic_match.as_str()[1..].to_string(); // Remove leading /
    let remaining = &input[topic_match.end()..];

    // Parse the remaining path parts
    let parts = parse_parts(remaining)?;

    Ok(MessagePath { topic_name, parts })
}

fn parse_parts(input: &str) -> Result<Vec<MessagePathPart>, ParseError> {
    let mut parts = Vec::new();
    let mut chars = input.chars().peekable();
    let mut pos = 0;

    while let Some(&c) = chars.peek() {
        match c {
            '.' => {
                chars.next();
                pos += 1;
                // Parse field name
                let field = parse_identifier(&mut chars, &mut pos)?;
                if field.is_empty() {
                    return Err(ParseError::UnexpectedToken(".".into(), pos));
                }
                parts.push(MessagePathPart::Field(field));
            }
            '[' => {
                chars.next();
                pos += 1;
                // Check for variable reference
                if chars.peek() == Some(&'$') {
                    chars.next();
                    pos += 1;
                    let var_name = parse_until(&mut chars, &mut pos, ']')?;
                    parts.push(MessagePathPart::Variable(var_name));
                } else {
                    // Parse index or slice
                    let content = parse_until(&mut chars, &mut pos, ']')?;
                    if content.contains(':') {
                        let slice_parts: Vec<&str> = content.splitn(2, ':').collect();
                        let start = if slice_parts[0].is_empty() {
                            None
                        } else {
                            Some(slice_parts[0].parse::<i64>().map_err(|_| {
                                ParseError::InvalidPath(format!("Invalid slice start: {}", slice_parts[0]))
                            })?)
                        };
                        let end = if slice_parts[1].is_empty() {
                            None
                        } else {
                            Some(slice_parts[1].parse::<i64>().map_err(|_| {
                                ParseError::InvalidPath(format!("Invalid slice end: {}", slice_parts[1]))
                            })?)
                        };
                        parts.push(MessagePathPart::Slice { start, end });
                    } else {
                        let index = content.parse::<i64>().map_err(|_| {
                            ParseError::InvalidPath(format!("Invalid array index: {}", content))
                        })?;
                        parts.push(MessagePathPart::Index(index));
                    }
                }
            }
            '{' => {
                chars.next();
                pos += 1;
                let filter_content = parse_until(&mut chars, &mut pos, '}')?;
                let filter = parse_filter(&filter_content, pos)?;
                parts.push(MessagePathPart::Filter(filter));
            }
            _ => {
                return Err(ParseError::UnexpectedToken(c.to_string(), pos));
            }
        }
    }

    Ok(parts)
}

fn parse_identifier(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    pos: &mut usize,
) -> Result<String, ParseError> {
    let mut name = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            name.push(c);
            chars.next();
            *pos += 1;
        } else {
            break;
        }
    }
    Ok(name)
}

fn parse_until(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    pos: &mut usize,
    end_char: char,
) -> Result<String, ParseError> {
    let start_pos = *pos;
    let mut content = String::new();
    let mut depth = 0;

    loop {
        match chars.next() {
            Some(c) if c == end_char && depth == 0 => {
                *pos += 1;
                return Ok(content);
            }
            Some('[') | Some('{') => {
                depth += 1;
                content.push(chars.peek().copied().unwrap_or(' '));
                *pos += 1;
            }
            Some(']') | Some('}') => {
                depth -= 1;
                content.push(chars.peek().copied().unwrap_or(' '));
                *pos += 1;
            }
            Some(c) => {
                content.push(c);
                *pos += 1;
            }
            None => {
                return Err(ParseError::UnterminatedFilter(start_pos));
            }
        }
    }
}

fn parse_filter(content: &str, pos: usize) -> Result<FilterCondition, ParseError> {
    // Try operators in order of length (longest first)
    let operators = [
        ("==", FilterOperator::Equal),
        ("!=", FilterOperator::NotEqual),
        ("<=", FilterOperator::LessOrEqual),
        (">=", FilterOperator::GreaterOrEqual),
        ("<", FilterOperator::LessThan),
        (">", FilterOperator::GreaterThan),
    ];

    for (op_str, op) in &operators {
        if let Some(idx) = content.find(op_str) {
            let field = content[..idx].trim().to_string();
            let value_str = content[idx + op_str.len()..].trim();

            let value = if value_str.starts_with('"') && value_str.ends_with('"') {
                FilterValue::String(value_str[1..value_str.len() - 1].to_string())
            } else if value_str == "true" {
                FilterValue::Bool(true)
            } else if value_str == "false" {
                FilterValue::Bool(false)
            } else {
                value_str
                    .parse::<f64>()
                    .map(FilterValue::Number)
                    .map_err(|_| {
                        ParseError::InvalidPath(format!("Invalid filter value: {}", value_str))
                    })?
            };

            return Ok(FilterCondition {
                field,
                operator: *op,
                value,
            });
        }
    }

    Err(ParseError::InvalidPath(format!(
        "No valid operator found in filter at position {}",
        pos
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_topic() {
        let path = parse_message_path("/camera/image").unwrap();
        assert_eq!(path.topic_name, "camera/image");
        assert!(path.parts.is_empty());
    }

    #[test]
    fn test_field_access() {
        let path = parse_message_path("/topic.header.stamp.sec").unwrap();
        assert_eq!(path.topic_name, "topic");
        assert_eq!(path.parts.len(), 3);
        assert_eq!(path.parts[0], MessagePathPart::Field("header".into()));
        assert_eq!(path.parts[1], MessagePathPart::Field("stamp".into()));
        assert_eq!(path.parts[2], MessagePathPart::Field("sec".into()));
    }

    #[test]
    fn test_array_index() {
        let path = parse_message_path("/topic.array[0]").unwrap();
        assert_eq!(path.parts.len(), 2);
        assert_eq!(path.parts[1], MessagePathPart::Index(0));
    }

    #[test]
    fn test_slice() {
        let path = parse_message_path("/topic.data[5:10]").unwrap();
        assert_eq!(path.parts[1], MessagePathPart::Slice { start: Some(5), end: Some(10) });
    }

    #[test]
    fn test_filter() {
        let path = parse_message_path("/topic{status==1}").unwrap();
        assert_eq!(path.parts.len(), 1);
        if let MessagePathPart::Filter(f) = &path.parts[0] {
            assert_eq!(f.field, "status");
            assert_eq!(f.operator, FilterOperator::Equal);
            assert_eq!(f.value, FilterValue::Number(1.0));
        } else {
            panic!("Expected filter");
        }
    }

    #[test]
    fn test_variable() {
        let path = parse_message_path("/topic.array[$idx]").unwrap();
        assert_eq!(path.parts[1], MessagePathPart::Variable("idx".into()));
    }
}
