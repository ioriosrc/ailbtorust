```rust
use crate::fixtures::*;
use log::*;
use serde_json::{self, Value};

pub struct LogFixture;

impl Fixture for LogFixture {
    fn topics(&self) -> Vec<LogTopic> {
        vec![
            LogTopic {
                name: "/rosout".to_string(),
                schema_name: "rosgraph_msgs/Log".to_string(),
            },
            LogTopic {
                name: "/foo/rosout".to_string(),
                schema_name: "rosgraph_msgs/Log".to_string(),
            },
            LogTopic {
                name: "/studio_source_2/rosout".to_string(),
                schema_name: "rosgraph_msgs/Log".to_string(),
            },
        ]
    }

    fn frame(&self) -> Value {
        serde_json::to_value(make_long_fixture()).unwrap()
    }
}

pub struct PanelSetupFixture {
    fixture: LogFixture,
    include_settings: bool,
}

impl Fixture for PanelSetupFixture {
    fn topics(&self) -> Vec<LogTopic> {
        let fixture = &self.fixture;
        if self.include_settings {
            vec![LogTopic {
                name: "/rosout".to_string(),
                schema_name: "rosgraph_msgs/Log".to_string(),
            }]
        } else {
            fixture.topics()
        }
    }

    fn frame(&self) -> Value {
        serde_json::to_value(self.fixture.frame()).unwrap()
    }
}
```