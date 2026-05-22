```rust
use std::rc::Rc;
use std::cell::RefCell;

use crate::scale_value;
use crate::{ReactNode, useCallback, useEffect, useRef, use_layout_effect};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use typed_jsx::{jsx, Fragment};

type HoverOverEvent = {
  fraction: f64;
  clientX: f32;
  clientY: f32;
};

type Props = {
  fraction: Option<f64>;
  disabled: bool;
  onChange: (value: f64) => void;
  onHoverOver?: (event: HoverOverEvent) => void;
  onHoverOut?: () => void;
  renderSlider?: (value: Option<f64>) -> ReactNode;
};

fn default_render_slider(value: Option<f64>, className: String) -> Fragment {
  if let Some(val) = value {
    return <div className={className} style={{ width: `${val * 100}%` }} />;
  }
  Fragment::default()
}

pub struct Slider {
  props: Props,
  fraction_ref: RefCell<f64>,
  disabled_ref: RefCell<bool>,
  render_slider_ref: RefCell<Option<Box<dyn Fn(f64) -> ReactNode>>>,
  on_hover_over_ref: RefCell<Option<Box<dyn Fn(HoverOverEvent) -> ()>>>,
  on_hover_out_ref: RefCell<Option<Box<dyn Fn() -> ()>>>,
  el_ref: Rc<RefCell<HTMLDivElement>>,
}

impl Slider {
  fn new(props: Props) -> Self {
    Self {
      props,
      fraction_ref: RefCell::new(0.5),
      disabled_ref: RefCell::new(false),
      render_slider_ref: RefCell::new(Some(Box::new(default_render_slider))),
      on_hover_over_ref: RefCell::new(None),
      on_hover_out_ref: RefCell::new(None),
      el_ref: Rc::new(RefCell::new(document.createElement("div"))),
    }
  }

  fn update_fraction(&self, value: f64) {
    self.fraction_ref.borrow_mut() = value;
    if let Some(ref render_slider) = *self.render_slider_ref.borrow_mut() {
      render_slider(value);
    }
  }

  fn update_disabled(&self, disabled: bool) {
    self.disabled_ref.borrow_mut() = disabled;
    self.el_ref.borrow_mut().borrow_mut().style().set("cursor", disabled.into());
  }

  fn update_on_hover_over(&self, on_hover_over: Option<Box<dyn Fn(HoverOverEvent) -> ()>>) {
    self.on_hover_over_ref.borrow_mut() = on_hover_over;
  }

  fn update_on_hover_out(&self, on_hover_out: Option<Box<dyn Fn() -> ()>>) {
    self.on_hover_out_ref.borrow_mut() = on_hover_out;
  }
}

fn main() {
  // Example usage
}
```