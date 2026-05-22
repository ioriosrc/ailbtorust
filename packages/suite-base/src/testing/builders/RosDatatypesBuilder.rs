```rust
use std::collections::HashMap;

pub struct RosDatatypesBuilder {}

impl RosDatatypesBuilder {
    pub fn optional_message_definition(props: Option<HashMap<&str, String>>) -> HashMap<&str, String> {
        let mut definitions = MessageDefinitionBuilder.message_definition_fields();
        if let Some(props) = props {
            for (key, value) in &props {
                definitions.insert(key.to_string(), value.to_string());
            }
        }
        BasicBuilder.string().to_string() as HashMap<&str, String>
    }
}
```