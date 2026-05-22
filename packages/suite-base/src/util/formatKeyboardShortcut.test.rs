```rust
use std::env;
use test::{self, assert_eq};

fn format_keyboard_shortcut(shortcut: char, modifiers: &[char]) -> String {
    let mut shortcut_str = shortcut.to_string();
    for modifier in modifiers {
        shortcut_str.push_str(format!("+{}", modifier).as_str());
    }
    shortcut_str
}

#[test]
fn test_format_keyboard_shortcut() {
    let mut env_vars = env::vars_mut();
    env_vars.insert("USER_AGENT", "Windows");
    assert_eq!(format_keyboard_shortcut('O', &["Shift", "Meta"]), "Shift+Ctrl+O");

    env_vars.insert("USER_AGENT", "Linux");
    assert_eq!(format_keyboard_shortcut('O', &["Shift", "Meta"]), "Shift+Ctrl+O");

    env_vars.insert("USER_AGENT", "Mac");
    assert_eq!(format_keyboard_shortcut('O', &["Shift", "Meta"]), "Shift+Cmd+O");

    env_vars.remove("USER_AGENT");
}
```