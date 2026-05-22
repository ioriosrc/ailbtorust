```rust
use std::collections::HashMap;

fn parse_cliflags(args: Vec<String>) -> HashMap<String, String> {
    let mut flags_map = HashMap::new();

    for arg in args {
        if arg.starts_with("--") {
            let parts: Vec<&str> = arg.split('=').collect();
            if parts.len() == 2 {
                flags_map.insert(parts[0].to_string(), parts[1].to_string());
            } else if parts.len() == 1 {
                flags_map.insert(parts[0].to_string(), "");
            }
        }
    }

    flags_map
}

fn main() {
    // Example usage of parse_cliflags function
    let argv = vec!["--flag=value", "--flag2=value2"];
    let result = parse_cliflags(argv);
    println!("{:?}", result); // Output should be {"flag": "value", "flag2": "value2"}
}
```