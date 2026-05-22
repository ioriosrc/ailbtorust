```rust
use crate::fixtures::{fixture, MessageEvent};
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pipeline_wrapper(initial_props: &Rc<HashMap<String, String>>) -> Box<dyn FnMut(&mut HashMap<String, String>) -> Vec<MessageEvent>> {
        Box::new(move |props| {
            let topics = props.get("topics").unwrap().parse::<Vec<crate::fixtures::Topic>>().unwrap();
            let datatypes = props.get("datatypes").unwrap().parse::<HashMap<&str, &str>>().unwrap();
            let messages: Vec<MessageEvent> = props.get("messages").unwrap().clone();
            let active_data = props.get("activeData").map(|s| s.parse::<crate::fixtures::PlayerStateActiveData>().unwrap());
            let global_variables = props.get("globalVariables").map(|s| serde_json::from_str(s).unwrap());

            use crate::fixtures::{fixture, MessageEvent};

            messages
        })
    }

    #[test]
    fn test_subscribes_based_on_topics() {
        let initial_props = Rc::new(HashMap::from([
            ("topics".to_string(), "/some/topic:/some/other/topic:/sliced_topic.field".to_string()),
        ]));
        let wrapper = make_pipeline_wrapper(&initial_props);
        // Test logic here
    }
}
```