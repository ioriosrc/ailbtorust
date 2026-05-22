```rust
fn create_property(name: String, type_node: ast::TypeNode) -> ast::PropertyDeclaration {
    ast::PropertyDeclaration {
        decorators: vec![],
        name,
        optional_modifier: None,
        question_mark: None,
        type_annotation: Some(type_node),
    }
}

fn create_time_interface_declaration(name: &str) -> ast::InterfaceDeclaration {
    let mut properties = Vec::<ast::PropertySignature>::new();
    properties.push(create_property("sec".to_string(), ast::KeywordTypeNode(ast::KeywordTypeKind::NumberKeyword)));
    properties.push(create_property("nsec".to_string(), ast::KeywordTypeNode(ast::KeywordTypeKind::NumberKeyword)));

    let mut interface_declaration = ast::InterfaceDeclaration {
        decorators: vec![],
        name: ast::Identifier::new(name.to_string()),
        type_parameters: None,
        heritage_clauses: None,
        members: properties,
    };

    interface_declaration
}

fn generate_interface_name(type_name: &str) -> String {
    type_name.replace("/", "__")
}

fn ros_primitives_to_typescript_map() -> std::collections::HashMap<String, ast::KeywordTypeKind> {
    let mut map = std::collections::HashMap::new();
    map.insert("uint8".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("int8".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("uint16".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("int16".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("uint32".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("int32".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("float32".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("float64".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("int64".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("uint64".to_string(), ast::KeywordTypeKind::NumberKeyword);
    map.insert("string".to_string(), ast::KeywordTypeKind::StringKeyword);
    map.insert("bool".to_string(), ast::KeywordTypeKind::BooleanKeyword);

    map
}

fn typed_array_map() -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    map.insert("uint8", "Uint8Array");
    map.insert("int8", "Int8Array");

    map
}

fn generate_special_types_to_typescript_map(datatypes: &RosDatatypes) -> std::collections::HashMap<String, ast::TypeReferenceNode> {
    let mut map = std::collections::HashMap::new();
    datatypes.datatypes.iter().for_each(|(datatype, definition)| {
        if datatype.includes(".") {
            // Skip newer types that are not supported by generateRosLib; these will have interfaces
            // generated via generateTypesLib and can be used that way.
            return;
        }
        let type_reference = ast::TypeReferenceNode::new(
            ast::Identifier::new(format_interface_name(datatype)),
            Some(ast::TypeOperatorNode {
                operator: ast::TypeOperatorKind::KeyOf,
                target_type: Some(ast::TypeReferenceNode::new(
                    ast::Identifier::new(TOPICS_TO_MESSAGE_DEFINITION_NAME.to_string()),
                    None,
                )),
            }),
        );

        map.insert(datatype.to_string(), type_reference);
    });

    map
}

fn generate_roslib(topics: &[Topic], datatypes: &RosDatatypes) -> String {
    let mut topics_to_message_definition = ast::InterfaceDeclaration {
        decorators: vec![],
        name: ast::Identifier::new(TOPICS_TO_MESSAGE_DEFINITION_NAME.to_string()),
        type_parameters: None,
        heritage_clauses: None,
        members: Vec::<ast::PropertySignature>::new(),
    };

    let typed_message = ast::InterfaceDeclaration {
        decorators: vec![],
        name: ast::Identifier::new("Input"),
        type_parameters: Some(vec![
            ast::TypeParameterDeclaration {
                modifiers: vec![], // No modifiers
                name: "T".to_string(),
                constraint: Some(ast::TypeOperatorNode {
                    operator: ast::TypeOperatorKind::KeyOf,
                    target_type: Some(ast::TypeReferenceNode::new(
                        ast::Identifier::new(TOPICS_TO_MESSAGE_DEFINITION_NAME.to_string()),
                        None,
                    )),
                }),
            },
        ]),
        heritage_clauses: Some(vec![
            create_property("topic".to_string(), ast::KeywordTypeNode(ast::KeywordTypeKind::StringKeyword)),
            create_property("receive_time".to_string(), ast::KeywordTypeNode(ast::KeywordTypeKind::NumberKeyword)),
            create_property(
                "message".to_string(),
                create_type_reference_node(format_interface_name(datatypes.datatypes[0].schema_name.unwrap())),
            ),
        ]),
        members: Vec::<ast::PropertySignature>::new(),
    };

    let DATATYPES_IDENTIFIER = "Messages";

    let datatype_interfaces = generate_interface_names(datatypes);

    topics.iter().for_each(|topic| {
        if topic.schema_name.is_none() {
            return;
        }
        if !datatype_interfaces.contains_key(topic.schema_name.unwrap()) {
            datatype_interfaces.insert(
                topic.schema_name.unwrap(),
                create_type_reference_node(format_interface_name(topic.schema_name.unwrap())),
            );
        }

        topics_to_message_definition = ast::update_interface_declaration(
            topics_to_message_definition,
            vec![],
            topics_to_message_definition.name,
            Some(ast::TypeOperatorNode {
                operator: ast::TypeOperatorKind::KeyOf,
                target_type: Some(ast::TypeReferenceNode::new(
                    ast::Identifier::new(TOPICS_TO_MESSAGE_DEFINITION_NAME.to_string()),
                    None,
                )),
            }),
        );

        if !datatype_interfaces.contains_key(topic.schema_name.unwrap()) {
            datatype_interfaces.insert(
                topic.schema_name.unwrap(),
                create_type_reference_node(format_interface_name(topic.schema_name.unwrap())),
            );
        }

        topics_to_message_definition = ast::update_interface_declaration(
            topics_to_message_definition,
            vec![],
            topics_to_message_definition.name,
            Some(ast::TypeOperatorNode {
                operator: ast::TypeOperatorKind::KeyOf,
                target_type: Some(ast::TypeReferenceNode::new(
                    ast::Identifier::new(TOPICS_TO_MESSAGE_DEFINITION_NAME.to_string()),
                    None,
                )),
            }),
        );
    });

    let datatypes_namespace = ast::ModuleDeclaration {
        decorators: vec![],
        name: ast::Identifier::new(DATATYPES_IDENTIFIER.to_string()),
        body: Some(ast::ModuleBlock::from_iter(datatype_interfaces.values().cloned())),
        flags: ast::NodeFlags::Namespace,
    };

    let source_file = ast::SourceFile {
        range: ast::Range::default(),
        text: "".to_string(),
        module_declarations: vec![datatypes_namespace],
        script_kind: ast::ScriptKind::TS,
        script_target: ast::ScriptTarget::Latest,
        emit_hints: Vec::new(),
    };

    let printer = ast::Printer::new();
    let result = format!(
        "{}\n\n{}\n\n{}\n\n{}\n",
        printer.print_node(ast::EmitHint::Unspecified, json_interface_declaration, source_file),
        printer.print_node(ast::EmitHint::Unspecified, topics_to_message_definition, source_file),
        printer.print_node(ast::EmitHint::Unspecified, duration_interface, source_file),
        printer.print_node(ast::EmitHint::Unspecified, time_interface, source_file),
        printer.print_node(ast::EmitHint::Unspecified, datatypes_namespace, source_file)
    );

    result
}

fn main() {
    // Example usage:
    let topics = vec![
        Topic {
            name: "/camera/image_raw".to_string(),
            schema_name: Some("sensor_msgs/Image".to_string()),
            definition: RosDatatypeDefinition::Image { encoding: "jpeg".to_string() },
        },
        Topic {
            name: "/camera/depth_registered".to_string(),
            schema_name: Some("sensor_msgs/PointCloud2".to_string()),
            definition: RosDatatypeDefinition::PointCloud2 {
                fields: vec![
                    FieldSchemaDefinition::Field {
                        name: "points".to_string(),
                        datatype: "std_msgs/Vector3".to_string(),
                    },
                    FieldSchemaDefinition::Field {
                        name: "camera_info".to_string(),
                        datatype: "sensor_msgs/CameraInfo".to_string(),
                    },
                ],
            },
        },
    ];

    let datatypes = RosDatatypes {
        datatypes: vec![
            ("sensor_msgs/Image".to_string(), RosDatatypeDefinition::Image { encoding: "jpeg".to_string() }),
            ("sensor_msgs/PointCloud2".to_string(), RosDatatypeDefinition::PointCloud2 {
                fields: vec![
                    FieldSchemaDefinition::Field {
                        name: "points".to_string(),
                        datatype: "std_msgs/Vector3".to_string(),
                    },
                    FieldSchemaDefinition::Field {
                        name: "camera_info".to_string(),
                        datatype: "sensor_msgs/CameraInfo".to_string(),
                    },
                ],
            }),
        ],
    };

    println!("{}", generate_roslib(topics, &datatypes));
}
```

Note: The `ast` module is assumed to be a valid Rust crate that provides the necessary AST nodes for creating and manipulating TypeScript code. You may need to replace the placeholder modules (`sensor_msgs/Image`, etc.) with actual type definitions from your dataset or ROS message types.