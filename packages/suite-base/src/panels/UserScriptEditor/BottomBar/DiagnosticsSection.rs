```rust
use mui::{
    components::{Stack, Typography},
    Icon,
};
use lodash::invert;
use crate::players::UserScriptPlayer::{Diagnostic, DIAGNOSTIC_SEVERITY};

const severity_icons = {
    "Hint": <Icon fontSize="small" />,
    "Info": <Icon fontSize="small" color="info" />,
    "Warning": <Icon fontSize="small" color="warning" />,
    "Error": <Icon fontSize="small" color="error" />,
};

type Props<'a> = {
    diagnostics: &'a [Diagnostic];
};

fn severity_label(severity: DIAGNOSTIC_SEVERITY) -> &str {
    invert(DIAGNOSTIC_SEVERITY).get(severity).unwrap_or("Error")
}

fn error_loc(start_column: u32, start_line_number: u32) -> Option<&'static str> {
    if start_line_number != 0 && start_column != 0 {
        Some(&format!("[{start_line_number + 1},{start_column + 1}]"))
    } else {
        None
    }
}

fn main() {
    // Your Rust code here
}
```