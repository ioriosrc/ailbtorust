```rust
use crate::theme;

pub fn useStyles() -> theme::StyleProps {
  theme::create_style_props!({
    inputError: {
      color: theme.palette.error.main,
    },
  }, {
    root: {
      "& .MuiAutocomplete-endAdornment": {
        top: "50%",
      },
      "& .MuiAutocomplete-clearIndicator": {
        top: "50% !important",
      },
    },
  })
}
```