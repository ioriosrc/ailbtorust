```rust
use std::collections::HashMap;

struct Icons {
    name_to_icon: HashMap<String, String>,
}

impl Icons {
    fn new() -> Self {
        let mut icons = Icons {
            name_to_icon: HashMap::new(),
        };
        // Initialize with predefined icons
        icons.name_to_icon.insert("heart".to_string(), "icon_heart.png".to_string());
        icons.name_to_icon.insert("star".to_string(), "icon_star.png".to_string());
        icons.name_to_icon.insert("bell".to_string(), "icon_bell.png".to_string());
        icons
    }
}

fn main() {
    let icons = Icons::new();

    // Example usage in a function
    fn render_icon(name: &str) -> String {
        if let Some(icon_path) = icons.name_to_icon.get(name) {
            format!("Icon: {}", icon_path)
        } else {
            "Icon not found".to_string()
        }
    }

    println!("{}", render_icon("heart")); // Output: Icon: icon_heart.png
    println!("{}", render_icon("star"));  // Output: Icon: icon_star.png
    println!("{}", render_icon("bell"));  // Output: Icon: icon_bell.png
}
```