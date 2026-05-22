```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Bool,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Float32,
    Float64,
    String,
}

#[derive(Debug, Clone)]
pub enum MessagePathFilter {
    Filter { path: Vec<String>, value: Option<PrimitiveType> },
}

#[derive(Debug, Clone)]
pub struct Immutable<T> {
    value: T,
}

impl<T> Immutable<T> {
    pub fn get(&self) -> &T {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct RosDatatypes {
    // Define the structure of RosDatatypes here
}

fn is_primitive_type(type_: String) -> bool {
    match type_.as_str() {
        "bool" | "int8" | "uint8" | "int16" | "uint16" | "int32" | "uint32" | "int64" | "uint64"
        | "float32" | "float64" | "string" => true,
        _ => false,
    }
}

fn structure_item_is_integer_primitive(item: &MessagePathStructureItem) -> bool {
    item.structure_type == "primitive" && STRUCTURE_ITEM_INTEGER_TYPES.contains(&item.primitive_type)
}

#[derive(Debug, Clone)]
pub struct MessagePathStructureItem {
    pub structure_type: String,
    pub next_by_name: std::collections::HashMap<String, MessagePathStructureItem>,
    pub datatype: Option<String>,
}

pub fn message_path_structures(datatypes: Immutable<RosDatatypes>) -> std::collections::HashMap<String, MessagePathStructureItem> {
    let cached = message_path_structures_cache.get(&datatypes);
    if cached.is_some() {
        return cached.unwrap().clone();
    }

    let mut structures: std::collections::HashMap<String, MessagePathStructureItem> = std::collections::HashMap::new();
    for datatype in datatypes.get() {
        let next_by_name = structure_for(datatype.to_string(), &[]);
        structures.insert(datatype.to_string(), next_by_name);
    }
    message_path_structures_cache.insert(datatypes.clone(), structures);

    structures
}

pub fn valid_terminating_structure_item(
    structure_item: Option<&MessagePathStructureItem>,
    valid_types: &[&str],
) -> bool {
    if let Some(structure_item) = structure_item {
        !valid_types.is_empty() || matches!(
            (structure_item.structure_type.as_str(), structure_item.datatype),
            ("primitive", _)
                | (structure_item.structure_type.as_str(), Some(_))
                | (&_, "time")
                | (&_, "duration"),
        )
    } else {
        false
    }
}

fn message_paths_for_structure(
    structure: MessagePathStructureItem,
    message_paths_structure_args: Option<MessagePathsForStructureArgs>,
) -> Vec<MessagePathsForStructure> {
    let default_args = MessagePathsForStructureArgs {
        valid_types: None,
        no_multi_slices: None,
        message_path: vec![],
    };
    let args = message_paths_structure_args.unwrap_or(default_args);

    let cache_key = format!(
        "{}_{}_{}_{}",
        structure.datatype.as_str(),
        args.valid_types.map(|v| v.to_string()).collect::<Vec<_>>().join(","),
        args.no_multi_slices,
        args.message_path.iter().map(|p| p.path.join("|")).collect::<Vec<_>>().join("::")
    );
    let cached = message_paths_cache.get(&cache_key);
    if cached.is_some() {
        return cached.unwrap().clone();
    }

    let mut message_paths: Vec<MessagePathsForStructure> = vec![];
    fn traverse(structure_item: &MessagePathStructureItem, built_string: String) {
        if valid_terminating_structure_item(Some(structure_item), &args.valid_types) {
            message_paths.push(MessagePathsForStructure {
                path: built_string,
                terminating_structure_item: structure_item.to_owned(),
            });
        }
        if structure_item.structure_type == "message" {
            for (name, item) in &structure_item.next_by_name {
                traverse(item, format!("{}/{name}", built_string));
            }
        } else if structure_item.structure_type == "array" {
            if structure_item.next.structure_type == "message" {
                // When we have an array of messages, you probably want to filter on
                // some field, like `/topic.object{some_id=123}`. If we can't find a
                // typical filter name, fall back to `/topic.object[0]`.
                let typical_filter_item = item.next_by_name.iter().find(|(name)| is_typical_filter_name(name));
                if typical_filter_item.is_some() {
                    let (typical_filter_name, typical_filter_value) = typical_filter_item.unwrap();
                    // Find matching filter from clonedMessagePath
                    let mut cloned_message_path = args.message_path.clone();
                    let matching_filter_part = cloned_message_path.iter().find(|p| p.path[0] == typical_filter_name);
                    if matching_filter_part.is_some() {
                        let matching_filter_value = &cloned_message_path[matching_filter_part.unwrap().index];
                        traverse(
                            item.next,
                            format!("{}/{}/[{}]", built_string, typical_filter_name, matching_filter_value),
                        );
                    } else if is_primitive_type(typical_filter_value) {
                        traverse(item.next, format!("{}/{}/[0]", built_string, typical_filter_name));
                    } else if typical_filter_value.structure_type == "primitive" && typical_filter_value.primitive_type == "string" {
                        traverse(
                            item.next,
                            format!("{}/{}/[\"\"]", built_string, typical_filter_name),
                        );
                    } else {
                        traverse(item.next, format!("{}/{}/[0]", built_string, typical_filter_name));
                    }
                } else {
                    traverse(item.next, format!("{}/{}/[]", built_string));
                }
            } else if args.no_multi_slices != Some(true) {
                traverse(item.next, format!("{}/{}/[]", built_string));
            } else {
                traverse(item.next, format!("{}/{}/[0]", built_string));
            }
        }
    }

    traverse(&structure, "");
    let result = message_paths.sort_unstable_by_key(|p| p.path.clone());
    result
}

// Traverse down the structure given a `message_path`. Return if the path
// is valid, given the structure, `valid_types`, and `no_multi_slices`.
//
// We return the `msgPathPart` that was invalid to determine what sort
// of autocomplete we should show.
//
// We use memoizeWeak because it works with multiple arguments (lodash's memoize
// does not) and does not hold onto objects as strongly (it uses WeakMap).
pub fn traverse_structure(
    initial_structure_item: Option<MessagePathStructureItem>,
    message_path: Vec<MessagePathPart>,
) -> StructureTraversalResult {
    let structure_item = initial_structure_item;
    if structure_item.is_none() {
        return StructureTraversalResult {
            valid: false,
            msg_path_part: None,
            structure_item: None,
        };
    }
    for msg_path_part in message_path {
        if !structure_item.is_some() {
            return StructureTraversalResult {
                valid: false,
                msg_path_part: Some(msg_path_part),
                structure_item: None,
            };
        }
        if msg_path_part.type_ == "name" {
            if structure_item.as_ref().unwrap().structure_type != "message" {
                return StructureTraversalResult {
                    valid: false,
                    msg_path_part: Some(msg_path_part),
                    structure_item: None,
                };
            }
            let next = structure_item.as_ref().unwrap().next_by_name.get(&msg_path_part.name);
            structure_item = next.to_owned();
        } else if msg_path_part.type_ == "slice" {
            if structure_item.as_ref().unwrap().structure_type != "array" {
                return StructureTraversalResult {
                    valid: false,
                    msg_path_part: Some(msg_path_part),
                    structure_item: None,
                };
            }
            structure_item = structure_item.as_ref().unwrap().next.to_owned();
        } else if msg_path_part.type_ == "filter" {
            if structure_item.as_ref().unwrap().structure_type != "message"
                || msg_path_part.path.is_empty()
                || msg_path_part.value.is_none()
            {
                return StructureTraversalResult {
                    valid: false,
                    msg_path_part: Some(msg_path_part),
                    structure_item: None,
                };
            }
            let mut current = structure_item.as_ref().unwrap();
            for name in &msg_path_part.path {
                if let Some(next) = current.next_by_name.get(name) {
                    current = next.to_owned();
                } else {
                    return StructureTraversalResult {
                        valid: false,
                        msg_path_part: Some(msg_path_part),
                        structure_item: None,
                    };
                }
            }
        } else {
            assert_ne!(msg_path_part.type_, "name", "Invalid msgPathPart type: {}", msg_path_part.type_);
        }
    }
    StructureTraversalResult {
        valid: true,
        msg_path_part: Some(msg_path_part),
        structure_item: Some(structure_item.to_owned()),
    }
}
```