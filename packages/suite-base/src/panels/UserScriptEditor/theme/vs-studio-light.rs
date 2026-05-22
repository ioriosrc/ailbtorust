```rust
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Theme {
    base: String,
    inherit: bool,
    rules: Vec<Rule>,
    colors: ColorMap,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    foreground: Value,
    background: Value,
    token: String,
}

#[derive(Debug, Deserialize)]
pub struct ColorMap {
    editor_foreground: Value,
    editor_background: Value,
    editor_selection_background: Value,
    editor_line_highlight_background: Value,
    editor_cursor_foreground: Value,
    editor_whitespace_foreground: Value,
}

fn main() {
    let json_data = r#"
{
  "base": "vs",
  "inherit": true,
  "rules": [
    {
      "foreground": "#eefff8",
      "background": "#2f8963",
      "token": "markup.inserted.diff",
    },
    // ... other rules
  ],
  "colors": {
    "editor.foreground": "#000000",
    "editor.background": "#FFFFFF",
    // ... other colors
  }
}
"#;

    let theme: Theme = serde_json::from_str(json_data).unwrap();

    println!("{:?}", theme);
}
```