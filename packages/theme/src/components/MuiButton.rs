```rust
use crate::styles::{Button, ButtonProps};

pub fn button(props: &ButtonProps) -> impl 'static + Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    move |f| {
        write!(f, r#"<button disableElevation="true" style={{ transition: "none", fontSize: {}px }} {...props}}" />"#,
               if props.size == ButtonSize::Small {
                   0.625
               } else if props.size == ButtonSize::Large {
                   0.875
               } else {
                   1.0 // Default size
               })
    }
}
```