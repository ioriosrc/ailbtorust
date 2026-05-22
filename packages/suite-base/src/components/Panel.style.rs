```rust
use styled-components::css;

pub fn get_styles(theme: &styled_components::Theme) -> impl Fn(&StyledComponent<impl StyledComponentsProps>) -> styled_components::CssRule {
    css! {
        position: "absolute",
        bottom: "2px",
        left: "3px",
        whiteSpace: "pre-line",
        fontSize: "0.75em",
        fontFeatureSettings: custom_typography.font_feature_settings,
        opacity: 0.7,
        userSelect: "none",
        mixBlendMode: "difference",
    }
}
```