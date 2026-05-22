```rust
use crate::palette;
use crate::typography;

pub fn create_mui_theme(theme_preference: &str) -> mui::Theme {
    let theme = match theme_preference {
        "dark" => palette::dark(),
        "light" => palette::light(),
        _ => panic!("Invalid theme preference"),
    };

    mui::create_theme({
        name: Some(theme_preference),
        palette,
        shape: mui::shape::Shape::default().with_rounded_corners(2.0),
        typography,
        components,
    })
}
```