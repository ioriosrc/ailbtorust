```rust
use crate::components::{
  Panel, PanelToolbar, Stack, GlobalVariables, UseGlobalVariables, SaveConfig,
};
use crate::hooks::{UseVariableSliderSettings, VariableSliderConfig};
use crate::types::VariableSliderConfig;

type Props = {
  config: VariableSliderConfig;
  saveConfig: SaveConfig<VariableSliderConfig>;
};

fn VariableSliderPanel(props: Props): Box<dyn 'static> {
  let { config, save_config } = props;
  let slider_props = &config.slider_props;
  let global_variable_name = config.global_variable_name;
  let variables = use_global_variables();
  let global_variable_value = variables.get(global_variable_name).unwrap_or(0.0);
  let theme = use_theme();

  let set_slider_value = move |value| {
    if value != global_variable_value {
      variables.set(global_variable_name, value);
    }
  };
  let update_variable_debounced = use DebouncedCallback(set_slider_value, 250);

  let slider_on_change = move |_event: Event, value: f64| {
    set_slider_value(value);
    update_variable_debounced(value);
  };

  let marks = vec![
    SliderMark::new(slider_props.min, format!("{}", slider_props.min)),
    SliderMark::new(slider_props.max, format!("{}", slider_props.max)),
  ];

  Box::new(
    Stack::new()
      .full_height()
      .children(vec![
        PanelToolbar {},
        Stack::new()
          .flex("auto")
          .align_items("center")
          .justify_content("center")
          .full_height()
          .gap(2)
          .padding_y(2)
          .padding_x(3)
          .children(vec![
            Slider::new()
              .min(slider_props.min)
              .max(slider_props.max)
              .step(slider_props.step)
              .marks(marks)
              .value(global_variable_value)
              .on_change(slider_on_change),
            Typography::new()
              .variant("h5")
              .style(Style::with_margin(theme.spacing(-2.5))),
          ]),
      ]),
  )
}

fn main() {}
```