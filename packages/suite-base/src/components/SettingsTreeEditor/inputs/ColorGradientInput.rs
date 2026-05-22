```rust
use std::rc::Rc;

use crate::{components, utils};

use super::utils::{
  ColorSwatch, ColorPickerControl, useColorPickerControl, Stack,
};

pub struct ColorGradientInput {
  colors: Option<(String, String)>,
  disabled: bool,
  onChange: Rc<dyn Fn(Vec<String>)>,
}

impl ColorGradientInput {
  pub fn new(
    colors: Option<(String, String)>,
    disabled: bool,
    onChange: Rc<dyn Fn(Vec<String>)>,
  ) -> Self {
    ColorGradientInput { colors, disabled, onChange }
  }

  pub fn render(&self) -> Box<dyn components::Element> {
    let [left_color, right_color] = self.colors.unwrap_or_default();
    let safe_left_color = utils::get_safe_color(left_color);
    let safe_right_color = utils::get_safe_color(right_color);

    let theme = utils::use_theme();

    let left_swatch = useColorPickerControl({
      alpha_type: "alpha",
      onChange: Rc::clone(&self.onChange),
      value: left_color,
    });

    let right_swatch = useColorPickerControl({
      alpha_type: "alpha",
      onChange: Rc::clone(&self.onChange),
      value: right_color,
    });

    Box::new(components::Stack {
      direction: components::Direction::Row,
      alignItems: components::AlignItems::Center,
      position: components::Position::Relative,
      padding_x: 0.75,
      style: utils::styles!({
        opacity: if self.disabled { 0.5 } else { 1 },
        pointer_events: if self.disabled { "none" } else { "auto" },
        background: format!(
          "linear-gradient(to right, {}, {}), repeating-conic-gradient(transparent 0 90deg, {} 90deg 180deg) top left/10px 10px repeat",
          safe_left_color,
          safe_right_color,
          theme.palette.action.disabled
        ),
      }),
    })
    .with_child(
      components::ColorSwatch {
        color: Rc::clone(&safe_left_color),
        onClick: self.on_click_left,
      },
    )
    .with_child(
      utils::components::TextField {
        variant: "filled",
        size: "small",
        fullWidth: true,
        value: format!("{} / {}", left_color, right_color),
        style: components::Style::new(),
      },
    )
    .with_child(
      components::ColorSwatch {
        color: Rc::clone(&safe_right_color),
        onClick: self.on_click_right,
      },
    )
    .with_children([
      utils::components::Popover::new({
        open: false,
        anchor_el: None,
        onClose: Rc::clone(&self.handle_close),
        anchor_origin: components::Origin {
          vertical: components::Vertical::Bottom,
          horizontal: components::Horizontal::Left,
        },
        transform_origin: components::TransformOrigin {
          vertical: components::Vertical::Top,
          horizontal: components::Horizontal::Center,
        },
      })
      .with_child(ColorPickerControl {
        on_enter_key: Rc::clone(&self.handle_close),
        ..Rc::downgrade(&left_swatch)
      }),
      utils::components::Popover::new({
        open: false,
        anchor_el: None,
        onClose: Rc::clone(&self.handle_close),
        anchor_origin: components::Origin {
          vertical: components::Vertical::Bottom,
          horizontal: components::Horizontal::Left,
        },
        transform_origin: components::TransformOrigin {
          vertical: components::Vertical::Top,
          horizontal: components::Horizontal::Center,
        },
      })
      .with_child(ColorPickerControl {
        on_enter_key: Rc::clone(&self.handle_close),
        ..Rc::downgrade(&right_swatch)
      }),
    ])
  }
}

impl ColorGradientInput {
  fn on_click_left(&self, event: components::MouseEvent) {
    if self.disabled {
      return;
    }

    let left_color = utils::get_safe_color(event.target().unwrap());
    (self.onChange)(vec![left_color.clone(), right_color.clone()]);
  }

  fn on_click_right(&self, event: components::MouseEvent) {
    if self.disabled {
      return;
    }

    let right_color = utils::get_safe_color(event.target().unwrap());
    (self.onChange)(vec![left_color.clone(), right_color.clone()]);
  }

  fn handle_close(&self) {
    self.on_change(vec![]);
  }
}
```