```rust
use std::fmt::{Display, Formatter};
use yew::prelude::*;

pub struct DataSourceOption {
    icon: Option<Icon>,
    onClick: Callback<(), ()>,
    text: String,
    secondary_text: Option<String>,
    href: Option<String>,
    target: Option<String>,
}

impl DataSourceOption {
    pub fn new(icon: Option<Icon>, onClick: Callback<(), ()>, text: String, secondary_text: Option<String>, href: Option<String>, target: Option<String>) -> Self {
        Self {
            icon,
            onClick,
            text,
            secondary_text,
            href,
            target,
        }
    }
}

pub enum Icon {
    // Implement your icon types here
}

impl Display for DataSourceOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<div class=\"connection-button\">")?;
        if let Some(icon) = self.icon {
            // Render the icon based on the icon type
            write!(f, "<i class=\"icon-{}</i>", icon)?;
        }
        writeln!(f, "<span class=\"text-primary\">{}</span> <span class=\"text-secondary\">{}</span></div>", self.text, self.secondary_text.as_ref().unwrap_or(&"").trim_end())?;
        Ok(())
    }
}

pub struct DataSourceDialog {
    // Implement your DataSourceDialog struct here
}
```