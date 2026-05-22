```rust
use serde_json::{Map, Value};
use std::collections::HashMap;

// Define the MessageTypeFilter and OperatorType enums
#[derive(Debug)]
enum MessageTypeFilter {
    Filter(MessagePathFilter),
}

#[derive(Debug)]
enum OperatorType {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

// Define the MessagePathFilter struct
#[derive(Debug)]
struct MessagePathFilter {
    path: Vec<String>,
    value: Value,
    operator: OperatorType,
    type_: String,
    name_loc: i32,
    value_loc: i32,
    repr: String,
}

// Implementing filterMatches function in Rust
fn filter_matches(filter: &MessageTypeFilter, data: &HashMap<String, Value>) -> bool {
    match filter {
        MessageTypeFilter::Filter(sub_filter) => {
            let path = &sub_filter.path;
            let value = &sub_filter.value;
            let operator = &sub_filter.operator;

            // Check if the data matches the path
            if !data.contains_key(&path[0]) {
                return false;
            }

            let first_value = data.get(&path[0]).unwrap();

            // Match the operator and compare values
            match (*operator, *first_value) {
                (OperatorType::Eq, Value::Number(first_num)) => {
                    if value.is_number() && value.as_f64().unwrap() == first_num.unwrap() {
                        true
                    } else {
                        false
                    }
                }
                (OperatorType::Ne, Value::Number(first_num)) => {
                    if value.is_number() && value.as_f64().unwrap() != first_num.unwrap() {
                        true
                    } else {
                        false
                    }
                }
                (OperatorType::Gt, Value::Number(first_num)) => {
                    if value.is_number() && value.as_f64().unwrap() > first_num.unwrap() {
                        true
                    } else {
                        false
                    }
                }
                (OperatorType::Ge, Value::Number(first_num)) => {
                    if value.is_number() && value.as_f64().unwrap() >= first_num.unwrap() {
                        true
                    } else {
                        false
                    }
                }
                (OperatorType::Lt, Value::Number(first_num)) => {
                    if value.is_number() && value.as_f64().unwrap() < first_num.unwrap() {
                        true
                    } else {
                        false
                    }
                }
                (OperatorType::Le, Value::Number(first_num)) => {
                    if value.is_number() && value.as_f64().unwrap() <= first_num.unwrap() {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        },
    }
}

// Example usage of filter_matches function
fn main() {
    let test_data = HashMap::from([
        ("a", Value::Number(1)),
        ("b", Value::Number(2)),
    ]);

    let filter1 = MessagePathFilter {
        path: vec!["a".to_string()],
        value: Value::Number(1),
        operator: OperatorType::Eq,
        type_: String::from("filter"),
        name_loc: 0,
        value_loc: 0,
        repr: String::from("eq"),
    };

    let filter2 = MessagePathFilter {
        path: vec!["b".to_string()],
        value: Value::Number(3),
        operator: OperatorType::Ne,
        type_: String::from("filter"),
        name_loc: 0,
        value_loc: 0,
        repr: String::from("neq"),
    };

    println!("{}", filter_matches(&filter1, &test_data)); // true
    println!("{}", filter_matches(&filter2, &test_data)); // false
}
```