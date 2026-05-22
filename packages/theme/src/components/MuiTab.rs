```rust
// Import necessary Rust modules
use styled_components::Css;

pub struct MuiTab;
impl Css for MuiTab {
    fn css() -> Self {
        let opacity = "0.8";

        // Define the root style
        let root_style = format!(
            "{{ opacity: {} }}",
            opacity
        );

        // Define the selected style
        let selected_style = format!(
            "{{ opacity: {}; color: {}}};", 
            "1",
            "theme.palette.text.primary"
        );

        // Format the full CSS string
        let css_string = format!(
            "{root_style}
             .Mui-selected {{
                 {} }}
             .Mui-not(.Mui-selected):hover {{
                 {} }}",
            selected_style,
            root_style
        );

        MuiTab {
            css: css_string,
        }
    }
}
```