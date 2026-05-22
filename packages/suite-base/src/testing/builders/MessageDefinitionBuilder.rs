```rust
use lichtblick::message_definition::{MessageDefinition, MessageDefinitionField};
use lichtblick::test_builders::{BasicBuilder, defaults};

#[derive(Default)]
pub struct MessageDefinitionBuilder {}

impl MessageDefinitionBuilder {
    pub fn message_definition_field(
        props: Option<MessageDefinitionField>,
    ) -> MessageDefinitionField {
        let mut field = defaults(props.unwrap_or_default(), {
            type_: BasicBuilder.string(),
            name: BasicBuilder.string(),
            is_complex: BasicBuilder.boolean(),
            isArray: BasicBuilder.boolean(),
            array_length: BasicBuilder.number(),
            is_constant: BasicBuilder.boolean(),
            value: BasicBuilder.string(),
            defaultValue: BasicBuilder.string(),
            array_upper_bound: BasicBuilder.number(),
            upper_bound: BasicBuilder.number(),
            value_text: BasicBuilder.string(),
        });

        field
    }

    pub fn message_definition_fields(count: usize) -> Vec<MessageDefinitionField> {
        (0..count).map(|_| MessageDefinitionBuilder.message_definition_field(None)).collect()
    }

    pub fn message_definition(props: Option<MessageDefinition>) -> MessageDefinition {
        let mut definition = defaults(props.unwrap_or_default(), {
            name: BasicBuilder.string(),
            definitions: MessageDefinitionBuilder.message_definition_fields(3),
        });

        definition
    }
}
```