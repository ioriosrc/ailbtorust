```rust
type PrimitiveType = &str;

enum OperatorType {
    Eq,
    Neq,
    Le,
    Ge,
    Lt,
    Gt,
}

struct MessagePathFilter {
    type: String,
    path: Vec<String>,
    value: Option<serde_json::Value>,
    name_loc: usize,
    value_loc: usize,
    repr: String, // the original string representation of the filter
    operator: OperatorType,
}

enum MessagePathName {
    Name(String),
    Slice { start: usize, end: usize },
}

struct MessagePathSlice {
    type: String,
    start: Option<serde_json::Value>,
    end: Option<serde_json::Value>,
}

// A parsed version of paths.
#[derive(Debug)]
struct MessagePathPart {
    message_path: Vec<MessagePathName | MessagePathSlice | MessagePathFilter>,
    modifier: Option<String>,
}

#[derive(Debug)]
struct MessagePath {
    topic_name: String,
    topic_name_repr: String,
    message_path: Vec<MessagePathPart>,
    modifier: Option<String>,
}

// "Structure items" are a more useful version of `datatypes`. They can be
// easily traversed to either validate message paths or generate message paths.
#[derive(Debug)]
struct MessagePathStructureItemMessage {
    next_by_name: std::collections::HashMap<String, Box<MessagePathStructureItem>>,
    datatype: String,
}

#[derive(Debug)]
struct MessagePathStructureItemArray {
    next: Box<MessagePathStructureItem>,
    datatype: String,
}

#[derive(Debug)]
struct MessagePathStructureItemPrimitive {
    primitive_type: PrimitiveType,
    datatype: String,
}

enum MessagePathStructureItem {
    Message(MessagePathStructureItemMessage),
    Array(MessagePathStructureItemArray),
    Primitive(MessagePathStructureItemPrimitive),
}
```