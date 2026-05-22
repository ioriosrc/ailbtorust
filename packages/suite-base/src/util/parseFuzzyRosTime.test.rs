```rust
use std::f64;

fn parse_fuzzy_ros_time(time_str: &str) -> f64 {
    match time_str.parse::<i64>() {
        Ok(seconds) => seconds as f64 + (time_str.chars().count() - 9) * 0.1,
        Err(_) => panic!("Invalid input format"),
    }
}

fn main() {
    // Test cases
    assert_eq!(parse_fuzzy_ros_time("1"), 1.0);
    assert_eq!(parse_fuzzy_ros_time("31525401600001"), 31525401600.001);
    assert_eq!(parse_fuzzy_ros_time("31556937600001"), 31556937.6);
    assert_eq!(parse_fuzzy_ros_time("315569376000010"), 31556937.6);
    assert_eq!(parse_fuzzy_ros_time("31556937600001000"), 31556937.6);
    assert_eq!(parse_fuzzy_ros_time("31556937600001000000"), 31556937.6);
}
```