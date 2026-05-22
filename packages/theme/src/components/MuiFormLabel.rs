```rust
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct MuiFormLabel {
    style_overrides: Option<MuiFormLabelStyleOverrides>,
}

#[derive(Clone, Debug)]
pub struct MuiFormLabelStyleOverrides {
    root: Option<RootMuiFormLabelStyles>,
}

#[derive(Clone, Debug)]
struct RootMuiFormLabelStyles {
    margin_bottom: f64,
    font_size: f64,
    padding: (f64, f64),
}

impl Display for MuiFormLabelStyleOverrides {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.root {
            Some(root) => write!(f, "root {{\n\tmargin-bottom: {}\npadding: {}\n}}", root.margin_bottom, root.padding.0),
            None => write!(f, ""),
        }
    }
}

impl Display for RootMuiFormLabelStyles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "margin-bottom: {}\npadding: {{\n\tleft: {}\ntop: {}\n}}",
            self.margin_bottom, self.padding.0, self.padding.1
        )
    }
}

impl Display for MuiFormLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.style_overrides {
            Some(style_overrides) => write!(
                f,
                "MuiFormLabel {{\n{}\n}}",
                style_overrides.root.as_ref().map_or("", |root| root.to_string())
            ),
            None => write!(f, "MuiFormLabel {{}}"),
        }
    }
}
```