```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a trait for message definitions that include an optional flag
trait MessageDefinition {
    fn is_optional(&self) -> bool;
}

impl MessageDefinitionField {
    // Implementation of MessageDefinition for OptionalMessageDefinitionField
    fn is_optional(&self) -> bool {
        self.optional.unwrap_or_default()
    }
}

// Define a struct to represent the RosDatatypes map
pub type RosDatatypes = HashMap<String, OptionalMessageDefinition>;

// Example implementation of OptionalMessageDefinitionField
#[derive(Serialize, Deserialize)]
struct OptionalMessageDefinitionField {
    name: String,
    data_type: String,
    optional: Option<bool>,
}

impl MessageDefinition for OptionalMessageDefinitionField {
    fn is_optional(&self) -> bool {
        self.optional.unwrap_or_default()
    }
}

// Example implementation of OptionalMessageDefinition
#[derive(Serialize, Deserialize)]
struct OptionalMessageDefinition {
    name: String,
    fields: Vec<OptionalMessageDefinitionField>,
    optional: Option<bool>,
}
```