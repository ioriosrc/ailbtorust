```rust
use crate::styles::{makeStyles, CSSProperties};

pub const useStyles = makeStyles()(() => ({
  root: {
    font_family: "Arial, sans-serif",
    color: "#333",
  },
  title: {
    text_align: "center",
    font_size: "24px",
    margin_bottom: "20px",
  },
}));

pub const tooltip_style: CSSProperties = {
  background_color: "rgba(255, 255, 255, 0.8)",
  border_radius: "10px",
  border: "none",
  color: "#fff",
  font_size: "14px",
  padding: "10px",
  box_shadow: "0px 4px 6px rgba(0, 0, 0, 0.3)",
};
```