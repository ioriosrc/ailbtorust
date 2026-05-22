```rust
fn format_keyboard_shortcut(key: &str, modifiers: &[ModifierKeys]) -> String {
    let is_mac = navigator.userAgent.contains("Mac");

    let mut shortcut_string = modifiers.iter().map(|&modifier| match modifier {
        ModifierKeys::Meta => if is_mac { "⌘" } else { "Ctrl" },
        ModifierKeys::Control => if is_mac { "⌃" } else { "Ctrl" },
        ModifierKeys::Alt => if is_mac { "⌥" } else { "Alt" },
        ModifierKeys::Shift => if is_mac { "⇧" } else { "Shift" },
    }).collect::<Vec<&str>>();

    shortcut_string.push(key);

    shortcut_string.join(is_mac ? "" : "+")
}
```