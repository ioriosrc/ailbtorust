```rust
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
struct Initialization {
    datatypes: HashMap<String, OptionalMessageDefinition>,
    topics: Vec<Topic>,
    alerts: Vec<Alert>,
}

#[derive(Debug, PartialEq)]
struct Alert {
    message: String,
    severity: &'static str,
    tip: &'static str,
}

#[derive(Debug, PartialEq)]
struct Topic {
    name: String,
    schema_name: String,
}

fn validate_and_add_new_datatypes(accumulated: Initialization, current: Initialization) {
    let is_same_datatype = |a, b| a.definitions == b.definitions;

    for (datatype, current_definition) in current.datatypes {
        if !accumulated.datatypes.contains_key(&datatype) {
            accumulated.datatypes.insert(datatype.clone(), current_definition);
            continue;
        }

        if !is_same_datatype(&accumulated.datatypes[&datatype], &current_definition) {
            accumulated.alerts.push(Alert {
                message: format!("Different datatypes found for schema \"{}\"", datatype),
                severity: "warn",
                tip: "Ensure all MCAPs use the same schema for each datatype. Merging files may cause issues in visualization.",
            });
        }
    }
}

fn validate_and_add_new_topics(accumulated: Initialization, current: Initialization) {
    for topic in current.topics {
        if !accumulated.topics.contains(&topic.name) {
            accumulated.topics.push(topic);
            continue;
        }

        if accumulated.topics.iter().any(|t| t.schema_name == topic.schema_name) {
            let existing_topic = accumulated.topics.iter().find(|&t| t.name == topic.name).unwrap();
            if &existing_topic.schema_name != &topic.schema_name {
                accumulated.alerts.push(Alert {
                    message: format!("Schema name mismatch detected for topic \"{}\": Expected \"{}\", but found \"{}\".", topic.name, existing_topic.schema_name, topic.schema_name),
                    severity: "warn",
                    tip: "Ensure all MCAPs use a consistent schema for this topic.",
                });
            }
        }
    }
}
```