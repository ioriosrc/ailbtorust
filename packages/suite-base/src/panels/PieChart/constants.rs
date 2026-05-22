```rust
fn main() {
    let supported_data_types = vec![
        "int8".to_string(),
        "uint8".to_string(),
        "int16".to_string(),
        "uint16".to_string(),
        "int32".to_string(),
        "uint32".to_string(),
        "float32".to_string(),
        "float64".to_string(),
        "string".to_string(),
    ];

    let default_config = DefaultConfig {
        path: "".to_string(),
        title: String::from("Pie Chart"),
        legend1: String::from("Legend 1"),
        legend2: String::from("Legend 2"),
        legend3: String::from("Legend 3"),
        legend4: String::from("Legend 4"),
        legend5: String::from("Legend 5"),
        legend6: String::from("Legend 6"),
        legend7: String::from("Legend 7"),
        legend8: String::from("Legend 8"),
        legend9: String::from("Legend 9"),
        legend10: String::from("Legend 10"),
    };
}
```