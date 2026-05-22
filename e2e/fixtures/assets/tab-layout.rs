```rust
struct ConfigById {
    active_tab_idx: u32,
    tabs: Vec<Tab>,
}

struct Tab {
    title: String,
}

fn main() {
    let config_by_id = ConfigById {
        active_tab_idx: 0,
        tabs: vec![Tab {
            title: "1".to_string(),
        }],
    };

    println!("{:?}", config_by_id);
}
```