```rust
use serde_json::{Map, Value};

// Define a struct to represent the log message structure
struct RoslogMessage {
    level: i32,
    name: String,
    msg: String,
}

fn parse_message(message_str: &str) -> Result<RoslogMessage, serde_json::Error> {
    serde_json::from_str(message_str)
}

// Function to filter messages based on the given parameters
fn filter_messages(messages: &[RoslogMessage], params: Map<String, Value>) -> Vec<RoslogMessage> {
    let min_level = params.get("minLogLevel").and_then(|v| v.as_i32()).unwrap_or(0);
    let search_terms = params.get("searchTerms").map(|v| serde_json::from_slice(v.as_str().unwrap()).unwrap()).unwrap_or(vec![]);
    let name_filter = params
        .get("nameFilter")
        .and_then(|v| v.as_object())
        .unwrap_or(&HashMap::new());

    messages.into_iter()
        .filter(|msg| {
            // Check if the level is lower than or equal to min_level and contains at least one search term
            msg.level >= min_level && search_terms.iter().any(|term| msg.msg.contains(term))
        })
        .collect()
}

fn main() {
    let ros1_msgs = vec![RoslogMessage {
        level: 2,
        name: "/some_topic".to_string(),
        msg: "Couldn't find int 83757.".to_string(),
    }];

    let ros2_msgs = vec![
        RoslogMessage {
            level: 30,
            name: "/some_topic".to_string(),
            msg: "Couldn't find int 83757.".to_string(),
        },
    ];

    // Test cases to verify the correctness of the filter function
    let params_min_level_3 = map! {};
    assert_eq!(filter_messages(ros1_msgs, params_min_level_3), Vec::<RoslogMessage>::new());

    let params_min_level_2 = map! {};
    assert_eq!(filter_messages(ros1_msgs, params_min_level_2), ros1_msgs);

    let params_min_level_3_search_terms = map![
        "searchTerms".to_string(), vec![Value::String("some".to_string())],
        "minLogLevel".to_string(), Value::Number(json!(3)),
    ];
    assert_eq!(
        filter_messages(ros1_msgs, params_min_level_3_search_terms),
        ros1_msgs
    );

    let params_min_level_3_name_filter = map![
        "nameFilter".to_string(),
        serde_json::json!({
            "/some_topic": {
                "visible": false,
            },
        }),
    ];
    assert_eq!(filter_messages(ros1_msgs, params_min_level_3_name_filter), Vec::<RoslogMessage>::new());

    let params_min_level_2_name_filter = map![
        "nameFilter".to_string(),
        serde_json::json!({
            "/some_topic": {
                "visible": true,
            },
        }),
    ];
    assert_eq!(
        filter_messages(ros1_msgs, params_min_level_2_name_filter),
        ros1_msgs
    );

    let params_min_level_3_search_terms_msg = map![
        "searchTerms".to_string(), vec![Value::String("int")],
        "minLogLevel".to_string(), Value::Number(json!(3)),
    ];
    assert_eq!(
        filter_messages(ros2_msgs, params_min_level_3_search_terms_msg),
        Vec::<RoslogMessage>::new()
    );

    let params_min_level_1_name_filter = map![
        "nameFilter".to_string(),
        serde_json::json!({
            "/some_topic": {
                "visible": true,
            },
        }),
    ];
    assert_eq!(
        filter_messages(ros2_msgs, params_min_level_1_name_filter),
        ros2_msgs
    );
}
```