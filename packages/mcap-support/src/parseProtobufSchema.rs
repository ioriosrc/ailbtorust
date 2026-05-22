```rust
use protobuf_rs::{io::BytesReader, Message};

mod types;

pub fn parse_protobuf_schema(schema_name: &str, schema_data: &[u8]) -> Result<(types::MessageDefinitionMap, impl Fn(&[u8]) -> types::ProtobufMessage), Box<dyn std::error::Error>> {
    let mut reader = BytesReader::new(schema_data);
    let descriptor_set = pb_rs::descriptor::FileDescriptorSet::parse(&mut reader)?;

    // Create a protobuf root from the parsed descriptor set
    let root = pb_rs::reflect::Root::from_descriptor(descriptor_set);

    // Resolve all dependencies to ensure that the types are fully populated and can be accessed
    root.resolve_all()?;

    // Look up the type in the root based on the schema name
    let root_type = root.lookup_type(schema_name)?;

    // Modify the definition of google.protobuf.Timestamp and Duration so they are interpreted as
    // {sec: number, nsec: number}, compatible with the rest of Studio. The standard Protobuf types
    // use different names (`seconds` and `nanos`), and `seconds` is an `int64`, which would be
    // deserialized as a bigint by default.
    //
    // protobuf_definitions_to_datatypes also has matching logic to rename the fields.
    let fix_time_type = |type_: &pb_rs::reflect::Type| {
        if type_ == &root_type || !(type_ instanceof pb_rs::reflect::Type) {
            return;
        }
        type_.setup(); // ensure the original optimized toObject has been created
        let prev_to_object = type_.to_object; // eslint-disable-line @typescript-eslint/unbound-method
        let new_to_object: impl Fn(&pb_rs::reflect::Message, &protobuf_rs::reflect::FieldDescriptorProto) -> serde_json::Value => {
            let message = type_.decode(type_.encode(&prev_to_object(message)).unwrap());
            let result = prev_to_object(message);
            let { seconds, nanos } = result as { seconds: i64; nanos: u32 };
            if seconds > i64::MAX {
                return serde_json::json!({ "error": "Timestamps with seconds greater than 2^53-1 are not supported (found seconds=${seconds}, nanos=${nanos})" });
            }
            serde_json::json!({
                "sec": seconds,
                "nsec": nanos
            })
        };
        type_.to_object = new_to_object;
    };

    fix_time_type(&root_type);

    // Create a deserialize function that uses the resolved root to decode binary data into Protobuf messages
    let deserialize: impl Fn(&[u8]) -> types::ProtobufMessage = move |data| {
        let message_bytes = std::array::from_slice(data);
        let mut reader = BytesReader::new(message_bytes.as_ref());
        type_.decode(&mut reader).unwrap()
    };

    // Create a datatypes map based on the parsed root and Protobuf definitions
    let mut datatypes = types::MessageDefinitionMap::default();
    types::protobuf_definitions_to_datatypes(&mut datatypes, &root_type);

    if !datatypes.contains_key(schema_name) {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Protobuf schema does not contain an entry for '{}'. The schema name should be fully-qualified, e.g. '{}'").replace("'", "\""), strip_leading_dot(root_type.full_name()))));
    }

    Ok((datatypes, deserialize))
}
```