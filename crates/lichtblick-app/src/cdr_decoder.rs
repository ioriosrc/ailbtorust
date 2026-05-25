// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Message decoder for MCAP files.
//! Supports ROS1 message encoding and Protobuf encoding.
//! Decodes binary data into serde_json::Value matching Lichtblick Node.js output.

use std::collections::HashMap;

/// Parsed schema collection: schema_name → MessageSchema
pub type SchemaMap = HashMap<String, MessageSchema>;

/// A parsed field definition from a ROS .msg schema.
#[derive(Clone, Debug)]
pub struct SchemaField {
    pub name: String,
    pub type_name: String,
    pub is_array: bool,
    pub array_length: Option<u32>,
    pub is_complex: bool,
}

/// A parsed message definition with fields and constants.
#[derive(Clone, Debug)]
pub struct MessageSchema {
    pub name: String,
    pub fields: Vec<SchemaField>,
    pub constants: HashMap<String, (String, String)>,
    pub value_to_constant: HashMap<(String, String), String>,
}

// =============================================================================
// Schema Parsing (ROS .msg format)
// =============================================================================

/// Parse a ROS .msg schema text into a SchemaMap.
pub fn parse_ros_msg_schema(name: &str, data: &[u8]) -> SchemaMap {
    let content = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return SchemaMap::new(),
    };

    let mut schemas = SchemaMap::new();
    let mut current_name = name.to_string();
    let mut current_fields = Vec::new();
    let mut current_constants: HashMap<String, (String, String)> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("===") || line == "===" {
            finalize_schema(&mut schemas, &current_name, &mut current_fields, &mut current_constants);
            let rest = line.trim_start_matches('=').trim();
            if !rest.is_empty() {
                current_name = rest.strip_prefix("MSG: ").unwrap_or(rest).to_string();
            }
            continue;
        }

        if line.starts_with("MSG: ") {
            finalize_schema(&mut schemas, &current_name, &mut current_fields, &mut current_constants);
            current_name = line.strip_prefix("MSG: ").unwrap().trim().to_string();
            continue;
        }

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Constants: "type NAME=value" or "type NAME =value"
        if let Some((type_and_name, value)) = line.split_once('=') {
            let parts: Vec<&str> = type_and_name.trim().splitn(2, ' ').collect();
            if parts.len() == 2 && !parts[0].contains('[') {
                let type_str = parts[0].trim();
                let const_name = parts[1].trim();
                let value_str = value.trim().split('#').next().unwrap_or("").trim();
                current_constants.insert(
                    const_name.to_string(),
                    (type_str.to_string(), value_str.to_string()),
                );
                continue;
            }
        }

        // Fields
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() < 2 { continue; }
        let type_str = parts[0].trim();
        let field_name = parts[1].trim().split('#').next().unwrap_or("").trim();
        if field_name.is_empty() { continue; }

        let (base_type, is_array, array_length) = if type_str.contains('[') {
            let bracket_start = type_str.find('[').unwrap();
            let bracket_end = type_str.find(']').unwrap_or(type_str.len());
            let base = &type_str[..bracket_start];
            let len_str = &type_str[bracket_start + 1..bracket_end];
            let len = len_str.parse::<u32>().ok();
            (base.to_string(), true, len)
        } else {
            (type_str.to_string(), false, None)
        };

        let is_complex = !is_primitive_type(&base_type);
        current_fields.push(SchemaField {
            name: field_name.to_string(),
            type_name: base_type,
            is_array,
            array_length,
            is_complex,
        });
    }

    finalize_schema(&mut schemas, &current_name, &mut current_fields, &mut current_constants);
    schemas
}

fn finalize_schema(
    schemas: &mut SchemaMap,
    name: &str,
    fields: &mut Vec<SchemaField>,
    constants: &mut HashMap<String, (String, String)>,
) {
    if fields.is_empty() && constants.is_empty() { return; }

    let mut value_to_constant = HashMap::new();
    for (const_name, (type_name, value_str)) in constants.iter() {
        value_to_constant.insert(
            (type_name.clone(), value_str.clone()),
            const_name.clone(),
        );
    }

    schemas.insert(name.to_string(), MessageSchema {
        name: name.to_string(),
        fields: std::mem::take(fields),
        constants: std::mem::take(constants),
        value_to_constant,
    });
}

fn is_primitive_type(t: &str) -> bool {
    matches!(t,
        "bool" | "byte" | "char" |
        "uint8" | "int8" | "uint16" | "int16" |
        "uint32" | "int32" | "uint64" | "int64" |
        "float32" | "float64" |
        "string" | "wstring" |
        "time" | "duration" |
        "Header" | "std_msgs/Header"
    )
}

// =============================================================================
// Protobuf Schema Parsing (binary FileDescriptorSet)
// =============================================================================

/// Parse a protobuf schema (binary FileDescriptorSet) into a SchemaMap.
/// MCAP stores protobuf schemas as serialized google.protobuf.FileDescriptorSet.
pub fn parse_protobuf_schema(name: &str, data: &[u8]) -> SchemaMap {
    let mut schemas = SchemaMap::new();

    // Parse the FileDescriptorSet (repeated FileDescriptorProto in field 1)
    let fds_fields = parse_proto_wire_fields(data);
    for file_bytes in fds_fields.get_repeated_bytes(1) {
        parse_file_descriptor_proto(&file_bytes, &mut schemas);
    }

    // If the target schema wasn't found by exact dotted name, try variations
    if !schemas.contains_key(name) {
        let short = name.rsplit('.').next().unwrap_or(name);
        if let Some(schema) = schemas.get(short).cloned() {
            schemas.insert(name.to_string(), schema);
        } else {
            // Try suffix match
            let found = schemas.iter()
                .find(|(k, _)| k.ends_with(short) || k.ends_with(name))
                .map(|(_, v)| v.clone());
            if let Some(schema) = found {
                schemas.insert(name.to_string(), schema);
            }
        }
    }

    schemas
}

/// Helper for reading protobuf wire format fields into a map.
struct ProtoFields {
    fields: HashMap<u32, Vec<Vec<u8>>>,
    varints: HashMap<u32, Vec<u64>>,
}

impl ProtoFields {
    fn get_bytes(&self, field_num: u32) -> Option<&[u8]> {
        self.fields.get(&field_num)?.last().map(|v| v.as_slice())
    }

    fn get_repeated_bytes(&self, field_num: u32) -> Vec<Vec<u8>> {
        self.fields.get(&field_num).cloned().unwrap_or_default()
    }

    fn get_string(&self, field_num: u32) -> Option<String> {
        self.get_bytes(field_num)
            .and_then(|b| std::str::from_utf8(b).ok())
            .map(|s| s.to_string())
    }

    fn get_varint(&self, field_num: u32) -> Option<u64> {
        self.varints.get(&field_num)?.last().copied()
    }
}

fn parse_proto_wire_fields(data: &[u8]) -> ProtoFields {
    let mut fields: HashMap<u32, Vec<Vec<u8>>> = HashMap::new();
    let mut varints: HashMap<u32, Vec<u64>> = HashMap::new();
    let mut offset = 0;

    while offset < data.len() {
        let (tag, new_offset) = match read_varint(data, offset) {
            Some(v) => v,
            None => break,
        };
        offset = new_offset;
        let field_number = (tag >> 3) as u32;
        let wire_type = (tag & 0x07) as u8;

        match wire_type {
            0 => { // Varint
                let (val, new_off) = match read_varint(data, offset) {
                    Some(v) => v,
                    None => break,
                };
                offset = new_off;
                varints.entry(field_number).or_default().push(val);
            }
            1 => { // 64-bit
                if offset + 8 > data.len() { break; }
                let bytes = data[offset..offset + 8].to_vec();
                offset += 8;
                fields.entry(field_number).or_default().push(bytes);
            }
            2 => { // Length-delimited
                let (len, new_off) = match read_varint(data, offset) {
                    Some(v) => v,
                    None => break,
                };
                offset = new_off;
                let len = len as usize;
                if offset + len > data.len() { break; }
                let bytes = data[offset..offset + len].to_vec();
                offset += len;
                fields.entry(field_number).or_default().push(bytes);
            }
            5 => { // 32-bit
                if offset + 4 > data.len() { break; }
                let bytes = data[offset..offset + 4].to_vec();
                offset += 4;
                fields.entry(field_number).or_default().push(bytes);
            }
            _ => break,
        }
    }

    ProtoFields { fields, varints }
}

/// Parse a FileDescriptorProto and extract message definitions.
fn parse_file_descriptor_proto(data: &[u8], schemas: &mut SchemaMap) {
    let fields = parse_proto_wire_fields(data);

    // field 2: package name
    let package = fields.get_string(2).unwrap_or_default();

    // field 4: repeated DescriptorProto message_type
    for msg_bytes in fields.get_repeated_bytes(4) {
        parse_descriptor_proto(&msg_bytes, &package, schemas);
    }
}

/// Parse a DescriptorProto (message definition).
fn parse_descriptor_proto(data: &[u8], package: &str, schemas: &mut SchemaMap) {
    let fields = parse_proto_wire_fields(data);

    // field 1: name
    let msg_name = match fields.get_string(1) {
        Some(n) => n,
        None => return,
    };

    let full_name = if package.is_empty() {
        msg_name.clone()
    } else {
        format!("{}.{}", package, msg_name)
    };

    // field 2: repeated FieldDescriptorProto
    let mut schema_fields = Vec::new();
    let mut field_numbers = Vec::new(); // Store actual protobuf field numbers

    for field_bytes in fields.get_repeated_bytes(2) {
        let f = parse_proto_wire_fields(&field_bytes);

        let field_name = match f.get_string(1) {
            Some(n) => n,
            None => continue,
        };

        let field_number = f.get_varint(3).unwrap_or(0) as u32;
        let label = f.get_varint(4).unwrap_or(1); // 1=optional, 2=required, 3=repeated
        let field_type = f.get_varint(5).unwrap_or(0); // protobuf type enum
        let type_name_str = f.get_string(6).unwrap_or_default();

        let is_repeated = label == 3;

        // Convert protobuf type number to our type string
        let (our_type, is_complex) = proto_field_type_to_str(field_type, &type_name_str);

        schema_fields.push((field_number, SchemaField {
            name: field_name,
            type_name: our_type,
            is_array: is_repeated,
            array_length: None,
            is_complex,
        }));
    }

    // Sort fields by field number (protobuf field order)
    schema_fields.sort_by_key(|(num, _)| *num);

    // Store field numbers for decoding
    let ordered_fields: Vec<SchemaField> = schema_fields.iter().map(|(_, f)| f.clone()).collect();
    field_numbers = schema_fields.iter().map(|(n, _)| *n).collect();

    // Store the field number mapping
    PROTO_FIELD_NUMBERS.with(|map| {
        map.borrow_mut().insert(full_name.clone(), field_numbers.clone());
    });

    schemas.insert(full_name.clone(), MessageSchema {
        name: full_name.clone(),
        fields: ordered_fields,
        constants: HashMap::new(),
        value_to_constant: HashMap::new(),
    });

    // Also insert by short name
    if !schemas.contains_key(&msg_name) {
        if let Some(s) = schemas.get(&full_name).cloned() {
            schemas.insert(msg_name.clone(), s);
        }
    }

    // field 3: repeated DescriptorProto nested_type
    for nested_bytes in fields.get_repeated_bytes(3) {
        parse_descriptor_proto(&nested_bytes, &full_name, schemas);
    }
}

/// Convert protobuf field type enum to our type string.
fn proto_field_type_to_str(type_num: u64, type_name: &str) -> (String, bool) {
    match type_num {
        1 => ("float64".to_string(), false),  // TYPE_DOUBLE
        2 => ("float32".to_string(), false),  // TYPE_FLOAT
        3 => ("int64".to_string(), false),    // TYPE_INT64
        4 => ("uint64".to_string(), false),   // TYPE_UINT64
        5 => ("int32".to_string(), false),    // TYPE_INT32
        6 => ("uint64".to_string(), false),   // TYPE_FIXED64
        7 => ("uint32".to_string(), false),   // TYPE_FIXED32
        8 => ("bool".to_string(), false),     // TYPE_BOOL
        9 => ("string".to_string(), false),   // TYPE_STRING
        11 => { // TYPE_MESSAGE
            let clean = type_name.trim_start_matches('.');
            (clean.to_string(), true)
        }
        12 => ("byte".to_string(), false),    // TYPE_BYTES
        13 => ("uint32".to_string(), false),  // TYPE_UINT32
        14 => { // TYPE_ENUM - treat as int32
            ("int32".to_string(), false)
        }
        15 => ("int32".to_string(), false),   // TYPE_SFIXED32
        16 => ("int64".to_string(), false),   // TYPE_SFIXED64
        17 => ("int32".to_string(), false),   // TYPE_SINT32
        18 => ("int64".to_string(), false),   // TYPE_SINT64
        _ => ("int32".to_string(), false),
    }
}

/// Thread-local storage for protobuf field number mappings.
/// Maps full_message_name → Vec<field_number> (in schema field order).
use std::cell::RefCell;
thread_local! {
    static PROTO_FIELD_NUMBERS: RefCell<HashMap<String, Vec<u32>>> = RefCell::new(HashMap::new());
}

// =============================================================================
// ROS1 Message Decoder (no alignment, little-endian)
// =============================================================================

/// ROS1 binary reader - NO alignment padding, just sequential reads.
struct Ros1Reader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Ros1Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Ros1Reader { data, offset: 0 }
    }

    fn read_u8(&mut self) -> Option<u8> {
        if self.offset >= self.data.len() { return None; }
        let val = self.data[self.offset];
        self.offset += 1;
        Some(val)
    }

    fn read_i8(&mut self) -> Option<i8> {
        self.read_u8().map(|v| v as i8)
    }

    fn read_bool(&mut self) -> Option<bool> {
        self.read_u8().map(|v| v != 0)
    }

    fn read_u16(&mut self) -> Option<u16> {
        if self.offset + 2 > self.data.len() { return None; }
        let val = u16::from_le_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        self.offset += 2;
        Some(val)
    }

    fn read_i16(&mut self) -> Option<i16> {
        self.read_u16().map(|v| v as i16)
    }

    fn read_u32(&mut self) -> Option<u32> {
        if self.offset + 4 > self.data.len() { return None; }
        let val = u32::from_le_bytes([
            self.data[self.offset], self.data[self.offset + 1],
            self.data[self.offset + 2], self.data[self.offset + 3],
        ]);
        self.offset += 4;
        Some(val)
    }

    fn read_i32(&mut self) -> Option<i32> {
        self.read_u32().map(|v| v as i32)
    }

    fn read_u64(&mut self) -> Option<u64> {
        if self.offset + 8 > self.data.len() { return None; }
        let val = u64::from_le_bytes([
            self.data[self.offset], self.data[self.offset + 1],
            self.data[self.offset + 2], self.data[self.offset + 3],
            self.data[self.offset + 4], self.data[self.offset + 5],
            self.data[self.offset + 6], self.data[self.offset + 7],
        ]);
        self.offset += 8;
        Some(val)
    }

    fn read_i64(&mut self) -> Option<i64> {
        self.read_u64().map(|v| v as i64)
    }

    fn read_f32(&mut self) -> Option<f32> {
        if self.offset + 4 > self.data.len() { return None; }
        let val = f32::from_le_bytes([
            self.data[self.offset], self.data[self.offset + 1],
            self.data[self.offset + 2], self.data[self.offset + 3],
        ]);
        self.offset += 4;
        Some(val)
    }

    fn read_f64(&mut self) -> Option<f64> {
        if self.offset + 8 > self.data.len() { return None; }
        let val = f64::from_le_bytes([
            self.data[self.offset], self.data[self.offset + 1],
            self.data[self.offset + 2], self.data[self.offset + 3],
            self.data[self.offset + 4], self.data[self.offset + 5],
            self.data[self.offset + 6], self.data[self.offset + 7],
        ]);
        self.offset += 8;
        Some(val)
    }

    fn read_string(&mut self) -> Option<String> {
        let len = self.read_u32()? as usize;
        if len == 0 { return Some(String::new()); }
        if self.offset + len > self.data.len() { return None; }
        let s = String::from_utf8_lossy(&self.data[self.offset..self.offset + len]).to_string();
        self.offset += len;
        Some(s)
    }
}

// =============================================================================
// Decode to JSON (matching Lichtblick output)
// =============================================================================

/// Decode a message to serde_json::Value.
/// This is the main entry point used by the Raw Messages panel.
pub fn decode_message_to_json(
    data: &[u8],
    schema_name: &str,
    encoding: &str,
    schemas: &SchemaMap,
) -> serde_json::Value {
    match encoding {
        "ros1msg" => decode_ros1_to_json(data, schema_name, schemas),
        "protobuf" => decode_protobuf_to_json(data, schema_name, schemas),
        "cdr" => decode_cdr_to_json(data, schema_name, schemas),
        _ => serde_json::json!({ "__encoding": encoding, "__size": data.len() }),
    }
}

/// Decode ROS1 serialized message to JSON.
fn decode_ros1_to_json(data: &[u8], schema_name: &str, schemas: &SchemaMap) -> serde_json::Value {
    let schema = match find_schema(schema_name, schemas) {
        Some(s) => s,
        None => return serde_json::json!({ "__error": format!("Schema '{}' not found", schema_name) }),
    };

    let mut reader = Ros1Reader::new(data);
    decode_ros1_struct(&mut reader, schema, schemas)
}

fn decode_ros1_struct(reader: &mut Ros1Reader, schema: &MessageSchema, schemas: &SchemaMap) -> serde_json::Value {
    let mut map = serde_json::Map::new();

    for field in &schema.fields {
        let value = if field.is_array {
            decode_ros1_array(reader, field, schemas)
        } else {
            decode_ros1_field(reader, &field.type_name, field.is_complex, schemas)
        };

        let value = match value {
            Some(v) => v,
            None => break,
        };

        map.insert(field.name.clone(), value);
    }

    serde_json::Value::Object(map)
}

fn decode_ros1_array(reader: &mut Ros1Reader, field: &SchemaField, schemas: &SchemaMap) -> Option<serde_json::Value> {
    let count = if let Some(fixed_len) = field.array_length {
        fixed_len as usize
    } else {
        reader.read_u32()? as usize
    };

    if count > 10_000_000 { return Some(serde_json::json!(format!("[{} items]", count))); }

    // Byte arrays: encode as base64 or show length
    if (field.type_name == "uint8" || field.type_name == "byte") && count > 64 {
        if reader.offset + count > reader.data.len() { return None; }
        reader.offset += count;
        return Some(serde_json::json!(format!("<{} bytes>", count)));
    }

    let mut items = Vec::with_capacity(count.min(1024));
    for _ in 0..count {
        match decode_ros1_field(reader, &field.type_name, field.is_complex, schemas) {
            Some(v) => items.push(v),
            None => break,
        }
    }

    Some(serde_json::Value::Array(items))
}

fn decode_ros1_field(reader: &mut Ros1Reader, type_name: &str, is_complex: bool, schemas: &SchemaMap) -> Option<serde_json::Value> {
    if is_complex {
        let schema = find_schema(type_name, schemas)?;
        Some(decode_ros1_struct(reader, schema, schemas))
    } else {
        decode_ros1_primitive(reader, type_name)
    }
}

fn decode_ros1_primitive(reader: &mut Ros1Reader, type_name: &str) -> Option<serde_json::Value> {
    match type_name {
        "bool" => reader.read_bool().map(|v| serde_json::Value::from(v)),
        "byte" | "uint8" | "char" => reader.read_u8().map(|v| serde_json::Value::from(v)),
        "int8" => reader.read_i8().map(|v| serde_json::Value::from(v)),
        "uint16" => reader.read_u16().map(|v| serde_json::Value::from(v)),
        "int16" => reader.read_i16().map(|v| serde_json::Value::from(v)),
        "uint32" => reader.read_u32().map(|v| serde_json::Value::from(v)),
        "int32" => reader.read_i32().map(|v| serde_json::Value::from(v)),
        "uint64" => reader.read_u64().map(|v| serde_json::Value::from(v)),
        "int64" => reader.read_i64().map(|v| serde_json::Value::from(v)),
        "float32" => reader.read_f32().map(|v| serde_json::json!(v)),
        "float64" => reader.read_f64().map(|v| serde_json::json!(v)),
        "string" | "wstring" => reader.read_string().map(|v| serde_json::Value::from(v)),
        "time" | "duration" => {
            let sec = reader.read_u32()?;
            let nsec = reader.read_u32()?;
            // Format as floating point seconds (matching mcapcli/Lichtblick output)
            let time_f64 = sec as f64 + nsec as f64 / 1_000_000_000.0;
            Some(serde_json::json!(time_f64))
        }
        "Header" | "std_msgs/Header" => {
            // ROS1 Header: uint32 seq + time stamp + string frame_id
            let seq = reader.read_u32()?;
            let stamp_sec = reader.read_u32()?;
            let stamp_nsec = reader.read_u32()?;
            let frame_id = reader.read_string()?;
            let stamp_f64 = stamp_sec as f64 + stamp_nsec as f64 / 1_000_000_000.0;
            Some(serde_json::json!({
                "seq": seq,
                "stamp": stamp_f64,
                "frame_id": frame_id
            }))
        }
        _ => reader.read_u8().map(|v| serde_json::Value::from(v)),
    }
}

// =============================================================================
// Protobuf Wire Format Decoder
// =============================================================================

/// Decode protobuf message to JSON using schema.
/// For all-zero messages, shows default values for all fields (matching Lichtblick).
fn decode_protobuf_to_json(data: &[u8], schema_name: &str, schemas: &SchemaMap) -> serde_json::Value {
    let schema = match find_schema(schema_name, schemas) {
        Some(s) => s,
        None => return serde_json::json!({ "__error": format!("Schema '{}' not found", schema_name) }),
    };

    // Parse protobuf wire format
    let parsed = parse_protobuf_wire(data);

    // Build JSON from schema fields, filling in defaults for missing fields
    build_protobuf_json(schema, &parsed, schemas)
}

/// Parsed protobuf field from wire format.
#[derive(Clone, Debug)]
enum ProtoWireValue {
    Varint(u64),
    Fixed64(u64),
    Fixed32(u32),
    LengthDelimited(Vec<u8>),
}

/// Parse protobuf wire format into field_number → values map.
fn parse_protobuf_wire(data: &[u8]) -> HashMap<u32, Vec<ProtoWireValue>> {
    let mut fields: HashMap<u32, Vec<ProtoWireValue>> = HashMap::new();
    let mut offset = 0;

    while offset < data.len() {
        let (tag, new_offset) = match read_varint(data, offset) {
            Some(v) => v,
            None => break,
        };
        offset = new_offset;

        let field_number = (tag >> 3) as u32;
        let wire_type = (tag & 0x07) as u8;

        let value = match wire_type {
            0 => { // Varint
                let (val, new_off) = match read_varint(data, offset) {
                    Some(v) => v,
                    None => break,
                };
                offset = new_off;
                ProtoWireValue::Varint(val)
            }
            1 => { // 64-bit
                if offset + 8 > data.len() { break; }
                let val = u64::from_le_bytes([
                    data[offset], data[offset+1], data[offset+2], data[offset+3],
                    data[offset+4], data[offset+5], data[offset+6], data[offset+7],
                ]);
                offset += 8;
                ProtoWireValue::Fixed64(val)
            }
            2 => { // Length-delimited
                let (len, new_off) = match read_varint(data, offset) {
                    Some(v) => v,
                    None => break,
                };
                offset = new_off;
                let len = len as usize;
                if offset + len > data.len() { break; }
                let bytes = data[offset..offset + len].to_vec();
                offset += len;
                ProtoWireValue::LengthDelimited(bytes)
            }
            5 => { // 32-bit
                if offset + 4 > data.len() { break; }
                let val = u32::from_le_bytes([
                    data[offset], data[offset+1], data[offset+2], data[offset+3],
                ]);
                offset += 4;
                ProtoWireValue::Fixed32(val)
            }
            _ => break, // Unknown wire type
        };

        fields.entry(field_number).or_default().push(value);
    }

    fields
}

fn read_varint(data: &[u8], mut offset: usize) -> Option<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift = 0;
    loop {
        if offset >= data.len() { return None; }
        let byte = data[offset];
        offset += 1;
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 { break; }
        shift += 7;
        if shift >= 64 { return None; }
    }
    Some((result, offset))
}

/// Build JSON object from schema + parsed wire data.
fn build_protobuf_json(
    schema: &MessageSchema,
    wire_fields: &HashMap<u32, Vec<ProtoWireValue>>,
    schemas: &SchemaMap,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();

    // Get actual field numbers from the thread-local map
    let field_numbers = PROTO_FIELD_NUMBERS.with(|m| {
        m.borrow().get(&schema.name).cloned()
    });

    for (idx, field) in schema.fields.iter().enumerate() {
        // Use actual protobuf field number if available, otherwise fall back to idx+1
        let field_number = field_numbers
            .as_ref()
            .and_then(|nums| nums.get(idx).copied())
            .unwrap_or((idx + 1) as u32);

        let value = if field.is_array {
            match wire_fields.get(&field_number) {
                Some(values) => {
                    let items: Vec<serde_json::Value> = values.iter().map(|v| {
                        proto_wire_to_json(v, &field.type_name, field.is_complex, schemas)
                    }).collect();
                    serde_json::Value::Array(items)
                }
                None => serde_json::Value::Array(Vec::new()),
            }
        } else {
            match wire_fields.get(&field_number).and_then(|v| v.last()) {
                Some(wire_val) => proto_wire_to_json(wire_val, &field.type_name, field.is_complex, schemas),
                None => proto_default_value(&field.type_name, field.is_complex),
            }
        };

        map.insert(field.name.clone(), value);
    }

    serde_json::Value::Object(map)
}

fn proto_wire_to_json(
    wire_val: &ProtoWireValue,
    type_name: &str,
    is_complex: bool,
    schemas: &SchemaMap,
) -> serde_json::Value {
    if is_complex {
        // Nested message - decode length-delimited bytes
        if let ProtoWireValue::LengthDelimited(bytes) = wire_val {
            if let Some(schema) = find_schema(type_name, schemas) {
                let nested = parse_protobuf_wire(bytes);
                return build_protobuf_json(schema, &nested, schemas);
            }
        }
        return serde_json::json!({});
    }

    match type_name {
        "float64" => {
            match wire_val {
                ProtoWireValue::Fixed64(v) => serde_json::json!(f64::from_bits(*v)),
                ProtoWireValue::Varint(v) => serde_json::json!(*v as f64),
                _ => serde_json::json!(0.0),
            }
        }
        "float32" => {
            match wire_val {
                ProtoWireValue::Fixed32(v) => serde_json::json!(f32::from_bits(*v)),
                ProtoWireValue::Varint(v) => serde_json::json!(*v as f32),
                _ => serde_json::json!(0.0),
            }
        }
        "int32" => {
            match wire_val {
                ProtoWireValue::Varint(v) => serde_json::json!(*v as i32),
                ProtoWireValue::Fixed32(v) => serde_json::json!(*v as i32),
                _ => serde_json::json!(0),
            }
        }
        "int64" => {
            match wire_val {
                ProtoWireValue::Varint(v) => serde_json::json!(*v as i64),
                ProtoWireValue::Fixed64(v) => serde_json::json!(*v as i64),
                _ => serde_json::json!(0),
            }
        }
        "uint32" => {
            match wire_val {
                ProtoWireValue::Varint(v) => serde_json::json!(*v as u32),
                ProtoWireValue::Fixed32(v) => serde_json::json!(*v),
                _ => serde_json::json!(0),
            }
        }
        "uint64" => {
            match wire_val {
                ProtoWireValue::Varint(v) => serde_json::json!(*v),
                ProtoWireValue::Fixed64(v) => serde_json::json!(*v),
                _ => serde_json::json!(0),
            }
        }
        "bool" => {
            match wire_val {
                ProtoWireValue::Varint(v) => serde_json::json!(*v != 0),
                _ => serde_json::json!(false),
            }
        }
        "string" => {
            match wire_val {
                ProtoWireValue::LengthDelimited(bytes) => {
                    serde_json::json!(String::from_utf8_lossy(bytes).to_string())
                }
                _ => serde_json::json!(""),
            }
        }
        _ => {
            match wire_val {
                ProtoWireValue::Varint(v) => serde_json::json!(*v),
                ProtoWireValue::Fixed32(v) => serde_json::json!(*v),
                ProtoWireValue::Fixed64(v) => serde_json::json!(*v),
                ProtoWireValue::LengthDelimited(bytes) => serde_json::json!(format!("<{} bytes>", bytes.len())),
            }
        }
    }
}

fn proto_default_value(type_name: &str, is_complex: bool) -> serde_json::Value {
    if is_complex {
        return serde_json::json!({});
    }
    match type_name {
        "float32" | "float64" => serde_json::json!(0),
        "bool" => serde_json::json!(false),
        "string" => serde_json::json!(""),
        _ => serde_json::json!(0),
    }
}

// =============================================================================
// CDR Decoder (ROS2)
// =============================================================================

fn decode_cdr_to_json(data: &[u8], schema_name: &str, schemas: &SchemaMap) -> serde_json::Value {
    // CDR has 4-byte encapsulation header
    if data.len() < 4 {
        return serde_json::json!({});
    }
    // Reuse ROS1 decoder but skip 4-byte header (CDR alignment handled separately)
    // For simplicity, treat CDR like ROS1 with a 4-byte prefix skip
    let schema = match find_schema(schema_name, schemas) {
        Some(s) => s,
        None => return serde_json::json!({ "__error": format!("Schema '{}' not found", schema_name) }),
    };

    let mut reader = Ros1Reader::new(&data[4..]);
    decode_ros1_struct(&mut reader, schema, schemas)
}

// =============================================================================
// Helpers
// =============================================================================

/// Find schema by name with fallback matching.
fn find_schema<'a>(name: &str, schemas: &'a SchemaMap) -> Option<&'a MessageSchema> {
    schemas.get(name)
        .or_else(|| {
            let short = name.rsplit('/').next().unwrap_or(name);
            schemas.get(short)
        })
        .or_else(|| {
            schemas.iter()
                .find(|(k, _)| {
                    k.ends_with(name)
                        || k.rsplit('/').next() == Some(name)
                        || k.rsplit('.').next() == Some(name)
                })
                .map(|(_, v)| v)
        })
}
