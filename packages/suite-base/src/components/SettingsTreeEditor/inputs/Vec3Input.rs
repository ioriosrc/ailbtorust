```rust
use crate::{prelude::*, NumberInput};

type Vec3Props = {
  disabled: bool;
  onChange: fn(&mut [Option<f64>]);
  precision: Option<usize>;
  readOnly: bool;
  step: Option<f64>;
  value: Option<[f64; 3]>;
  min: Option<f64>;
  max: Option<f64>;
  placeholder: Option<[String; 3]>;
};

pub fn Vec3Input(props: Vec3Props): Component<()>::VNode {
  let disabled = props.disabled;
  let value = props.value.unwrap_or_default();
  let onChange = props.onChange;

  let mut numbers = vec![None, None, None];
  for i in 0..3 {
    if let Some(precision) = props.precision {
      numbers[i] = Some((value[i] as f64 * 10f64.powi(-precision)) as u32);
    } else {
      numbers[i] = Some(value[i] as u32);
    }
  }

  let onChangeCallback = move |position: usize, newValue| {
    let mut new_value = value.clone();
    if let Some(precision) = props.precision {
      new_value[position] = (newValue[position] as f64 * 10f64.powi(precision)) as u32;
    } else {
      new_value[position] = newValue[position];
    }
    numbers[position] = new_value[position];
    onChange(&numbers);
  };

  let placeholder: Option<String> = props.placeholder.unwrap_or_default();
  let min: Option<f64> = props.min;
  let max: Option<f64> = props.max;

  let inputs: Vec<Node<()>> = (0..3)
    .map(|i| {
      NumberInput::new()
        .data("Vec3Input-{}".to_string())
        .size(Small)
        .disabled(disabled)
        .read_only(props.readOnly)
        .variant(Filled)
        .full_width(true)
        .precision(props.precision.unwrap_or(2))
        .step(props.step.unwrap_or(1.0))
        .placeholder(placeholder.to_string())
        .value(numbers[i].clone())
        .min(min.clone())
        .max(max.clone())
        .on_change(move |new_value| {
          numbers[position] = new_value;
          onChangeCallback(position, new_value);
        })
    })
    .collect();

  Node::stack(inputs)
}
```