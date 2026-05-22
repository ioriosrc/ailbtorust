```rust
use std::collections::HashMap;

struct FlatbufferSchema {
    raw_schema: Vec<u8>,
}

impl FlatbufferSchema {
    fn new(raw_schema: Vec<u8>) -> Self {
        Self { raw_schema }
    }

    fn parse(&self) -> Result<Schema, String> {
        use flatbuffers_reflection::{Builder, Parser};

        let mut builder = Builder::new();
        Parser::read_from_bytes(&builder, &self.raw_schema).map_err(|e| e.to_string())
    }
}

fn type_for_simple_field(type_: BaseType) -> &'static str {
    match type_ {
        BaseType::Bool => "bool",
        BaseType::Byte => "i8",
        BaseType::UType => "u8",
        BaseType::UByte => "u8",
        BaseType::Short => "i16",
        BaseType::UShort => "u16",
        BaseType::Int => "i32",
        BaseType::UInt => "u32",
        BaseType::Long => "i64",
        BaseType::ULong => "u64",
        BaseType::Float => "f32",
        BaseType::Double => "f64",
        BaseType::String => "String",
        BaseType::Vector | BaseType::Obj | BaseType::Union | BaseType::Array | BaseType::MaxBaseType | BaseType::None => {
            panic!("Unhandled BaseType: {:?}", type_);
        }
    }
}

fn flatbuffer_string(unchecked: &str) -> String {
    unchecked.to_string()
}

fn type_for_field(schema: &Schema, field: &Field) -> Vec<MessageDefinitionField> {
    let mut fields = Vec::new();
    match field.type_.base_type() {
        BaseType::UType | BaseType::Bool | BaseType::Byte | BaseType::UByte | BaseType::Short | BaseType::UShort | BaseType::Int
        | BaseType::UInt | BaseType::Long | BaseType::ULong | BaseType::Float | BaseType::Double | BaseType::String => {
            let simple_type = type_for_simple_field(field.type_.base_type());
            if field.type_.index() != -1 {
                let enums = schema.enums[field.type_.index()].values;
                for enum_val in enums {
                    fields.push(MessageDefinitionField {
                        name: flatbuffer_string(enum_val.name()),
                        type_: simple_type,
                        is_constant: true,
                        value: enum_val.value(),
                    });
                }
            }
            fields.push(MessageDefinitionField {
                name: flatbuffer_string(field.name()),
                type_,
            });
        }
        BaseType::Vector | BaseType::Obj | BaseType::Union | BaseType::Array | BaseType::MaxBaseType => {
            let element = field.type_.element();
            if element != BaseType::Vector && element != BaseType::Obj && element != BaseType::Union && element != BaseType::Array {
                panic!("Vectors of vectors, unions, arrays, and None's are unsupported.");
            }
            let type_ = if element == BaseType::Obj || element == BaseType::Vector || element == BaseType::Union || element == BaseType::Array {
                flatbuffer_string(schema.objects[field.type_.index()].name())
            } else {
                type_for_simple_field(element);
            };
            if field.type_.index() != -1 {
                let enums = schema.enums[field.type_.index()].values;
                for enum_val in enums {
                    fields.push(MessageDefinitionField {
                        name: flatbuffer_string(enum_val.name()),
                        type_,
                        is_constant: true,
                        value: enum_val.value(),
                    });
                }
            }
            fields.push(MessageDefinitionField {
                name: flatbuffer_string(field.name()),
                type_,
                isArray: element != BaseType::Vector,
            });
        }
    }
    fields
}

fn parse_flatbuffer_schema(schema_name: &str, schema_array: Vec<u8>) -> (HashMap<&str, MessageDefinitionMap>, fn(&[u8]) -> Option<serde_json::Value>) {
    let flatbuffer_schema = FlatbufferSchema { raw_schema: schema_array };
    let schema_result = flatbuffer_schema.parse();
    if let Err(e) = schema_result {
        return (HashMap::new(), |_| None);
    }
    let schema = schema_result.unwrap();

    let mut datatypes = HashMap::new();
    let mut type_index = -1;
    for object in &schema.objects {
        if object.name() == schema_name {
            type_index = schema.objects.iter().position(|o| o.name() == schema_name).unwrap();
        }
        let fields = type_for_field(&schema, object);
        datatypes.insert(flatbuffer_string(object.name()), MessageDefinitionMap { definitions: fields });
    }

    if type_index == -1 && schema.root_table().name() != schema_name {
        return (HashMap::new(), |_| None);
    }

    let mut parser = flatbuffers_reflection::Parser::new(&schema.raw_schema);
    let to_object = |table| table.to_object_lambda(type_index, /*read_defaults=*/ true).unwrap();

    fn deserialize(buffer: &[u8]) -> Option<serde_json::Value> {
        use serde_json::{self, Value};
        let byte_buffer = ByteBuffer::from(bytes);
        let mut schema = Schema::new(byte_buffer.clone());
        parser.parse_schema(&mut schema)?;
        let table = Table::with_reader(schema.reader(), type_index).unwrap();
        to_object(table)
    }

    (datatypes, deserialize)
}
```