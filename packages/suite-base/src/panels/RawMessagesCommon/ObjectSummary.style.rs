```rust
use crate::styles;

pub fn use_styles() -> styles::SummaryClass {
    styles::make_styles!({
        summary: {
            color: styles::Palette::TextSecondary.into(),
            padding_left: "0.5em".into(),
        }
    })
}
```