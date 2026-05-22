```rust
use styled_components::css;

fn MuiFilledInput() -> css! {
    &{
        disable_underline: true,
    }
}

#[allow(dead_code)]
pub fn styleOverrides() -> css! {
    input: {
        padding: 10px 12.5px;
        @media (min-width: 480px) {
            padding: 7.5px 10px;
        }
    }

    root: {
        border_radius: theme.shape.borderRadius;

        &:focus::after {
            background_color: theme.palette.action.focus;
        }

        &[aria-invalid="true"]::after {
            background_color: alpha(theme.palette.error.main, theme.palette.action.focusOpacity);
        }

        &.Mui-disabled:after {
            opacity: 0.5;
        }

        &[aria-invalid="true"]:after {
            background_color: alpha(theme.palette.error.main, theme.palette.action.hoverOpacity);
        }
    }
}
```