```rust
use std::str::FromStr;

fn compare_versions(v1: &str, v2: &str) -> f64 {
    let parts1: Vec<&str> = v1.split('.').collect();
    let parts2: Vec<&str> = v2.split('.').collect();

    for i in 0..std::max(parts1.len(), parts2.len()) {
        if i < parts1.len() && i < parts2.len() {
            if parts1[i].parse::<f64>().unwrap() > parts2[i].parse::<f64>().unwrap() {
                return 1.0;
            } else if parts1[i].parse::<f64>().unwrap() < parts2[i].parse::<f64>().unwrap() {
                return -1.0;
            }
        } else if i < parts1.len() && i >= parts2.len() {
            return 1.0;
        } else if i >= parts1.len() && i < parts2.len() {
            return -1.0;
        }
    }

    0.0
}

fn main() {
    assert_eq!(compare_versions("1.2.3", "1.2.3"), 0.0);
    assert_eq!(compare_versions("1.2.0", "1.2"), 0.0);
    assert_eq!(compare_versions("1.2.0", "1.2.0"), 0.0);
    assert_eq!(compare_versions("1.2.3", "1.2.4"), 1.0);
    assert_eq!(compare_versions("1.9.9", "2.0.0"), -1.0);
    assert_eq!(compare_versions("1.3.0", "1.2.9"), 1.0);
    assert_eq!(compare_versions("1.2.3.1", "1.2.3"), 1.0);
    assert_eq!(compare_versions("1.2.3", "1.2.3.1"), -1.0);
    assert_eq!(compare_versions("a.b.c", "x.y.z"), NaN);
    assert_eq!(compare_versions("1..3", "1.2.3"), NaN);
}
```