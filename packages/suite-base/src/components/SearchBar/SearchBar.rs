```rust
use crate::components::SearchBarStyle as useStyles;
use materialize::{TextField, InputAdornment, IconButton, Button};
use materialize::prelude::*;
use materialize::theme::DefaultTheme;

fn SearchBar(props: PropsWithChildren<TextFieldProps>) -> Element {
  let { id = "search-bar", variant = "filled", disabled = false, value, onChange, onClear, show_clear_icon = false, start_adornment = <SearchIcon /> } = props;

  let classes = useStyles();

  div(classes.filter_search_bar)
    .child(
      TextField::new()
        .id(id)
        .variant(variant)
        .disabled(disabled)
        .value(value)
        .on_change(on_change)
        .full_width()
        .slot_props({
          ...props.slot_props,
          input: {
            start_adornment: (
              InputAdornment::new(classes.filter_start_adornment, start_adornment)
            ),
            end_adornment: show_clear_icon && (
              IconButton::new()
                .size("small")
                .title("Clear")
                .on_click(onClear)
                .edge("end")
                .child(ClearIcon::new())
            ),
          },
        }),
    )
}

pub fn SearchBarStyle() -> Style {
  Theme::default().with_font_size("16px").with_line_height("24px")
}
```