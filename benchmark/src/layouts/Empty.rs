```rust
fn main() {
    let config_by_id = std::collections::HashMap::new();
    let global_variables = std::collections::HashMap::new();
    let user_nodes = std::collections::HashMap::new();

    // Assuming the playback_config is a struct defined elsewhere
    // let playback_config = PlaybackConfig { speed: 1 };

    println!("{:?}", config_by_id);
    println!("{:?}", global_variables);
    println!("{:?}", user_nodes);
}
```