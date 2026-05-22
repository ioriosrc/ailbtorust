```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub type MessageTypeBySchemaName = HashMap<String, HashMap<String, String>>;

pub fn generate_types_interface(datatypes: &HashMap<String, Vec<&str>>) -> String {
    let mut src = r#"/**
 * MessageTypeBySchemaName enumerates the message types for all the schema names
 * in the current data source.
 *
 * You probably want to use Message<...> instead.
 */"#;
    
    for (datatype, definitions) in datatypes {
        src += r#"export type MessageTypeBySchemaName = {";
        
        for definition in definitions {
            let mut field_types: Vec<&str> = definition.split(',').collect();
            
            if field_types.len() > 1 {
                field_types.pop(); // Remove the last item, which is always "null"
            }
            
            src += r#"  "${datatype}": {";
            
            for (let i = 0; i < field_types.len(); i++) {
                let field_type = field_types[i].trim();
                
                if i > 0 {
                    src += ",";
                }
                
                src += format!("    \"{}\": {}", field_type, get_type_string(field_type));
            }
            
            src += r#"  },";
        }
        
        src += r#"};"#;
    }
    
    src
}

fn generate_types_by_topic_interface(topics: &[Topic]) -> String {
    let mut src = r#"/**
 * MessageTypeByTopic enumerates the Messages types for all the topics in
 * the current data source.
 *
 * You probably want to use Input<"/my-topic"> instead of MessageTypeByTopic.
 */"#;
    
    for topic in topics {
        src += format!("  \"{}\": MessageTypeBySchemaName[\"{}\"];\n", topic.name, topic.schema_name.unwrap_or_default());
    }
    
    src
}

fn generate_types_lib(args: Args) -> String {
    let types_by_topic = generate_types_by_topic_interface(&args.topics);
    
    // A topic may reference a datatype that we don't have in args.datatypes.
    // This happens for some data sources if nothing's subscribe to a topic and we never get info
    // about the specific datatype.
    //
    // We want the types library to still generate and compile so we use empty placeholders for such datatypes.
    let mut all_datatypes = HashMap::new();
    
    for (datatype, definitions) in &args.datatypes {
        if !all_datatypes.contains_key(datatype) {
            all_datatypes.insert(datatype.to_string(), Vec::new());
        }
        
        for definition in definitions {
            let mut field_types: Vec<&str> = definition.split(',').collect();
            
            if field_types.len() > 1 {
                field_types.pop(); // Remove the last item, which is always "null"
            }
            
            all_datatypes.get_mut(datatype).unwrap().push(field_types.join(","));
        }
    }
    
    let types_by_schema_name = generate_types_interface(&all_datatypes);
    
    let src = r#"// NOTE:
// This file is generated from the current data source.
// It contains helper types for looking up message definitions by schema name or topic.
//
// You likely want to use the higher-level types in \`./types\` rather than the types in this file directly.

type Time = {
  sec: number,
  nsec: number,
};

type Duration = Time;

${types_by_schema_name}

${types_by_topic}
"#;
    
    return src;
}

struct Args {
    topics: Vec<Topic>,
    datatypes: HashMap<String, Vec<&str>>,
}

#[derive(Debug)]
pub struct Topic {
    pub name: String,
    pub schema_name: Option<String>,
}

fn generate_empty_types_lib() -> String {
    static mut EMPTY_LIB: String = String::new();
    
    if EMPTY_LIB.is_empty() {
        EMPTY_LIB = generate_types_lib(Args {
            topics: Vec::new(),
            datatypes: HashMap::new(),
        });
    }
    
    EMPTY_LIB.clone()
}

pub fn main() {
    // Example usage:
    let args = Args {
        topics: vec![
            Topic { name: "/topic1".to_string(), schema_name: Some("MyMessage".to_string()) },
            Topic { name: "/topic2".to_string(), schema_name: None },
        ],
        datatypes: HashMap::from([
            ("MyMessage".to_string(), vec!["uint8", "int32, null"]),
        ]),
    };
    
    println!("{}", generate_types_lib(args));
}
```