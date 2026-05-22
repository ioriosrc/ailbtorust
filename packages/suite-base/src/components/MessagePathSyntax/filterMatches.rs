```rust
use std::collections::{HashMap, BTreeMap};

fn filter_matches(filter: &str, value: &str) -> bool {
    if !filter.contains(' ') {
        return false;
    }

    let mut split_filter = filter.split_whitespace();
    let path: Vec<&str> = split_filter.next().unwrap().split('.').collect();
    let operator: char = split_filter.next().unwrap().chars().next().unwrap();

    let mut map: HashMap<String, String> = HashMap::new();
    for name in &path {
        if map.contains_key(name) {
            let existing_value = map.remove(name).unwrap();
            map.insert(name.to_string(), existing_value);
        } else {
            break;
        }
    }

    if !map.is_empty() && map.get(value).is_none() {
        return false;
    }

    match operator {
        '==' => value == filter.split_whitespace().nth(2).unwrap(),
        '!=' => value != filter.split_whitespace().nth(2).unwrap(),
        '>=' => value.parse::<f64>().unwrap() >= filter.split_whitespace().nth(2).unwrap().parse::<f64>().unwrap(),
        '<=' => value.parse::<f64>().unwrap() <= filter.split_whitespace().nth(2).unwrap().parse::<f64>().unwrap(),
        '>' => value.parse::<f64>().unwrap() > filter.split_whitespace().nth(2).unwrap().parse::<f64>().unwrap(),
        '<' => value.parse::<f64>().unwrap() < filter.split_whitespace().nth(2).unwrap().parse::<f64>().unwrap(),
        _ => false,
    }
}
```