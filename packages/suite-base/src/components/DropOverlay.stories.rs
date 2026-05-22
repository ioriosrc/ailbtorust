```rust
use lichtblick::suite_base::{components::DropOverlay, Theme};

pub fn dark() -> DropOverlay {
    DropOverlay {
        theme: Theme::Dark,
    }
}

pub fn light() -> DropOverlay {
    DropOverlay {
        theme: Theme::Light,
    }
}
```

Esta função `dark` retorna um novo objeto `DropOverlay` com o tema definido como `"dark"`. Similmente, a função `light` retorna um objeto `DropOverlay` com o tema `"light"`.