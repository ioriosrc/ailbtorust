```rust
use serde_json::Value;
use std::convert::{TryFrom, TryInto};

type Channel = {
    message_encoding: String,
    schema: Option<Schema>,
};

type Schema = {
    name: String,
    encoding: String,
    data: Vec<u8>,
};

fn parse_flatbuffer_schema(name: &str, data: &[u8]) -> ParsedChannel {
    // Implementation to parse flatbuffer schema
}

fn parse_json_schema(data: &[u8]) -> ParsedChannel {
    let json_string = String::from_utf8_lossy(data).into();
    let value: Value = serde_json::from_str(&json_string).unwrap();
    parse_flatbuffer_schema("ros1msg", &value.to_string().as_bytes())
}

fn parse_protobuf_schema(name: &str, data: &[u8]) -> ParsedChannel {
    // Implementation to parse protobuf schema
}

fn parse_ros1schema(name: &str, data: &[u8]) -> ParsedChannel {
    let json_string = String::from_utf8_lossy(data).into();
    let value: Value = serde_json::from_str(&json_string).unwrap();
    parse_flatbuffer_schema("ros2msg", &value.to_string().as_bytes())
}

fn parse_omgidl_schema(name: &str, data: &[u8]) -> ParsedChannel {
    // Implementation to parse omgidl schema
}

fn parse_message_definition(data: &[u8]) -> Vec<MessageDefinitionField> {
    // Implementation to parse message definition from msg file
}

fn parse_idl_definitions_to_datatypes(parsed_definitions: Vec<IDLMessageDefinition>, root_name: Option<&str>) -> MessageDefinitionMap {
    // Implementation to convert parsed IDL definitions to datatypes
}

fn parse_channel(channel: Channel, options: Option<{ allow_empty_schema: bool }>) -> ParsedChannel {
    match channel.message_encoding.as_str() {
        "json" => parse_json_schema(channel.schema.unwrap().data),
        "flatbuffer" => parse_flatbuffer_schema(channel.schema.unwrap().name, &channel.schema.unwrap().data),
        "protobuf" => parse_protobuf_schema(channel.schema.unwrap().name, &channel.schema.unwrap().data),
        "ros1" => parse_ros1schema(channel.schema.unwrap().name, &channel.schema.unwrap().data),
        "cdr" => match channel.schema.unwrap().encoding.as_str() {
            "ros2msg" | "ros2idl" | "omgidl" => parse_idl_definitions_to_datatypes(parse_message_definition(&channel.schema.unwrap().data), Some(channel.schema.unwrap().name)),
            _ => Err("Unsupported encoding for cdr schema"),
        },
        _ => Err("Unsupported encoding"),
    }
}

#[derive(Debug)]
struct IDLMessageDefinition {
    name: String,
    // Other fields
}

#[derive(Debug)]
struct MessageReader<'a> {
    definitions: &'a [IDLMessageDefinitionField],
}

impl MessageReader<'_> {
    fn read_message(&self, data: &[u8]) -> serde_json::Value {
        // Implementation to read message from data using MessageReader
    }
}

#[derive(Debug)]
struct ParsedChannel {
    datatypes: MessageDefinitionMap,
    deserialize: fn(data: &[u8]) -> serde_json::Value,
}
```