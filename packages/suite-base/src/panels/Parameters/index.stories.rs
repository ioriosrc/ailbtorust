```rust
use lichtenblick::suite::{self, PLAYER_CAPABILITIES};
use lichtenblick::suite_base::players::constants::PLAYER_CAPABILITIES;
use lichtenblick::suite_base::stories::PanelSetup;

use lichtenblick::{ParameterValue, Map};

#[derive(Debug)]
struct Fixture {
    topics: Vec<String>,
    frame: HashMap<String, Value>,
    capabilities: Vec<PLAYER_CAPABILITIES>,
    active_data: ActiveData,
    set_parameter: Option<fn(&str, &ParameterValue)>,
}

struct ActiveData {
    parameters: Option<Map<String, ParameterValue>>,
}

impl Fixture {
    fn new() -> Self {
        let mut params = Map::new();
        params.insert("undefined".to_string(), None);
        params.insert("boolean".to_string(), false.into());
        params.insert("number".to_string(), -42.into());
        params.insert("string".to_string(), "Hello, world!".into());
        params.insert(
            "date".to_string(),
            chrono::DateTime::parse_from_str("1618876820517", "%s").unwrap().into(),
        );
        params.insert(
            "Uint8Array".to_string(),
            vec![0, 1].as_slice().into(),
        );
        params.insert("array".to_string(), vec![1, 2].into());
        params.insert(
            "string array".to_string(),
            vec!["one", "two", "three"].into(),
        );
        params.insert(
            "struct".to_string(),
            serde_json::from_str(r#"{"a": 1, "b": [2, 3], "c": "String value"}"#).unwrap(),
        );

        Self {
            topics: Vec::new(),
            frame: HashMap::new(),
            capabilities: vec![PLAYER_CAPABILITIES.get_parameters(), PLAYER_CAPABILITIES.set_parameters()],
            active_data: ActiveData { parameters },
            set_parameter: None,
        }
    }

    fn with_get_params(mut self) -> Self {
        if let Some(params) = self.active_data.parameters.clone() {
            self.set_parameter = Some(|key, value| {
                let mut new_params = params.clone();
                new_params.insert(key.to_string(), value);
                self.active_data.parameters = Some(new_params);
            });
        }
        self
    }

    fn with_set_params(mut self) -> Self {
        if let Some(params) = self.active_data.parameters.clone() {
            self.set_parameter = Some(|key, value| {
                let mut new_params = params.clone();
                new_params.insert(key.to_string(), value);
                self.active_data.parameters = Some(new_params);
            });
        }
        self
    }

    fn with_parameters(mut self, params: Map<String, ParameterValue>) -> Self {
        self.active_data.parameters = Some(params);
        self
    }
}

#[derive(Debug)]
struct EditableParameters;

impl Default for EditableParameters {
    fn default() -> Self {
        EditableParameters {}
    }
}

fn main() {
    let fixture = Fixture::new().with_get_params();
    // ...
}
```