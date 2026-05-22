```rust
use fluentui_system::icon::Alert24Filled;
use fluentui_system::material::{BadgeProps, Badge as MuiBadge};
use fluentui_system::theme::theming::Theme;

fn main() {
    let mut badge = MuiBadge::new();
    badge.set_badge_content(4);
    badge.set_icon(Some(Box::new(Alert24Filled)));
    badge.set_theme(Theme::Light);

    // Rest of the code remains the same as in the TypeScript/React version
}
```