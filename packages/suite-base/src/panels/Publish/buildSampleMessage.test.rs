```rust
use ros_datatypes::{RosDatatypes, RosDataType};

fn build_sample_message(datatypes: &RosDatatypes, datatype_name: &str) -> Value {
    match datatypes.get(datatype_name) {
        Some(datatype) => match datatype.definitions.len() {
            0 => Value::Object(vec![]),
            1 => {
                let definition = &datatype.definitions[0];
                if definition.is_constant {
                    return Value::Value(definition.type.clone());
                } else if definition.isArray {
                    let array_length = if definition.array_length.is_some() {
                        definition.array_length.unwrap()
                    } else {
                        4
                    };
                    vec![Value::Object(vec![
                        ("data", build_sample_message(datatypes, definition.name)),
                    ])]
                } else {
                    vec![build_sample_message(datatypes, definition.name)]
                }
            },
            _ => vec![build_sample_message(datatypes, definition.name)],
        },
        None => Value::Error("Datatype not found".to_string()),
    }
}

// Example usage
fn main() {
    let datatypes = RosDatatypes::new(
        Object::from_iter([
            ("A", RosDataType {
                definitions: vec![],
            }),
            ("B", RosDataType {
                definitions: vec![Definition::new("data", "A".to_string())],
            }),
            // Define other datatypes as needed
        ]),
    );

    println!("{:?}", build_sample_message(&datatypes, "B"));
}
```