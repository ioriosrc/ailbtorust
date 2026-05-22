```rust
use stardust::prelude::*;
use stardust::ui::color_gradient_input::{self, ColorGradientInput};

fn main() {
    let mut app = App::new("Color Gradient Input");
    app.add_component(
        Component::new(ColorGradientInput)
            .with_prop("colors", vec!["#ffaa00".to_string(), "#0026ff".to_string()])
            .with_prop("onChange", move |colors| {
                // Handle color change
                println!("New colors: {:?}", colors);
            }),
    );
    app.run();
}
```