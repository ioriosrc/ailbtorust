```rust
use mui::icons::material::ArrowForward as ArrowForwardIcon;
use mui::widgets::{Button};
use mui::theme::create_theme;

fn main() {
    let theme = create_theme();
    let root = Button::new(theme, |mut button| {
        button.set_icon(ArrowForwardIcon);
        button.set_size("medium");
        button.set_variant("text");
        button.set_color("#404040");
        button.set_background("transparent");
        button.set_padding(&[12.5 as f32, 8.75 as f32]);
    });

    println!("{}", root);
}
```