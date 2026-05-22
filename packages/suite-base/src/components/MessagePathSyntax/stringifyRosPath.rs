```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessagePath {
    topic_name_repr: String,
    message_path: Vec<MessagePathPart>,
    modifier: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum MessagePathPart {
    Name(String),
    Filter(MessagePathFilter),
    Slice(Slice),
}

#[derive(Serialize, Deserialize)]
pub struct Slice {
    start: SlicePart,
    end: SlicePart,
}

fn stringify_message_path(path: &MessagePath) -> String {
    format!(
        "{}{}@{}",
        path.topic_name_repr,
        path.message_path.iter().map(stringify_message_path_part).collect::<String>(),
        path.modifier.as_deref()
    )
}

fn stringify_message_path_part(part: &MessagePathPart) -> String {
    match part {
        MessagePathPart::Name(name) => format!(".{}", name),
        MessagePathPart::Filter(filter) => filter.to_string(),
        MessagePathPart::Slice(slice) => slice_to_string(slice),
    }
}

fn slice_to_string(slice: &Slice) -> String {
    if let (start, end) = (&slice.start, &slice.end) {
        match (start, end) {
            (SlicePart::Number(start), SlicePart::Number(end)) if start == end => format!("[{}]", start),
            (SlicePart::Number(start), SlicePart::Number(end)) | (SlicePart::Infinity, SlicePart::Infinity) if start.is_zero() => format!("[:{}]"), // :∞
            (SlicePart::Infinity, SlicePart::Number(end)) => format!("[:{end}]"), // ∞:end
            _ => format!("[{}:{}]", start, end),
        }
    } else {
        let start_str = slice_part_to_string(&slice.start);
        let end_str = slice_part_to_string(&slice.end);
        if start_str == end_str {
            format!("[{}]", start_str)
        } else {
            format!("[{}:{}] {}", start_str, end_str)
        }
    }
}

fn slice_part_to_string(part: &SlicePart) -> String {
    match part {
        SlicePart::Number(num) if num.is_infinite() => "".to_string(),
        SlicePart::Number(num) => num.to_string(),
        SlicePart::Variable(name) => format!("${}", name),
    }
}

fn filter_to_string(filter: &MessagePathFilter) -> String {
    format!(
        "{{{}}}",
        filter.repr
            .iter()
            .map(|(key, value)| format!("{}.{}={}", key, if value.is_infinite() { "∞" } else { JSON.stringify(value) }))
            .collect::<String>()
    )
}
```