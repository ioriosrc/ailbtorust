```rust
use std::fmt::Debug;

fn get_matching_rule(value: f64, rules: &[(&str, &str)]) -> Option<&str> {
    for &(raw_value, label) in rules.iter() {
        if match raw_value.parse::<f64>() {
            Ok(num) => {
                if matches!(num, num if (value > num && operator == ">") || (value < num && operator == "<")) ||
                    matches!(
                        num, num if (value >= num && operator == ">=") || (value <= num && operator == "<=")
                    )
                {
                    return Some(label);
                }
            },
            Err(_) => false,
        } {
            return Some(label);
        }
    }
    None
}

// Test cases
#[test]
fn test_get_matching_rule() {
    let rules: Vec<(String, String)> = vec![
        ("hello".to_string(), "Hello".to_string()),
        ("true".to_string(), "True".to_string()),
        ("false".to_string(), "False".to_string()),
        ("-1.5".to_string(), "Negative float".to_string()),
        ("100000000000000000001".to_string(), "Large int".to_string()),
    ];

    assert_eq!(get_matching_rule(1.0, &rules), Some("True"));
    assert_eq!(get_matching_rule(-2.0, &rules), None);
}

#[test]
fn test_get_matching_rule_operators() {
    let rules: Vec<(String, String)> = vec![
        ("2".to_string(), "First".to_string()),
        ("3".to_string(), "Second".to_string()),
    ];

    assert_eq!(get_matching_rule(1.0, &rules), None);
    assert_eq!(get_matching_rule(2.0, &rules), Some("First"));
    assert_eq!(get_matching_rule(3.0, &rules), Some("Second"));

    let rules: Vec<(String, String)> = vec![
        ("2".to_string(), "First".to_string()),
        ("3".to_string(), "Second".to_string()),
    ];

    assert_eq!(get_matching_rule(-1.0, &rules), None);
    assert_eq!(get_matching_rule(2.0, &rules), Some("First"));
    assert_eq!(get_matching_rule(3.0, &rules), Some("Second"));
}
```