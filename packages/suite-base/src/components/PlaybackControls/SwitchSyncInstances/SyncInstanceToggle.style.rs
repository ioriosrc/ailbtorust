```rust
use styled_components::css;
use styled_components::Theme;

pub fn useStyles(props: &UseStyleProps) -> css! {
  button {
    padding: props.spacing(0.3, 0);
    background_color: "transparent";
    color: if props.sync_instances { "primary" } else { theme.color.inherit };
    :hover {
      background_color: theme.palette.action.hover;
    }
  }

  textWrapper {
    display: "flex";
    align_items: "end";
  }

  syncText {
    font_size: 12px;
    font_weight: 500;
  }

  onOffText {
    font_size: 10px;
    font_weight: 400;
    margin_top: "-8px";
  }
}
```