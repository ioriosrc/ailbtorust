```rust
use react::prelude::*;
use lacewing_ui_components::{NumberInput, Stack};
use lacewing_shared::model::{Vec2};

type Vec2Props = {
    disabled: bool,
    onChange: impl FnMut(&mut [f64], Option<f64>),
    precision: usize,
    readOnly: bool,
    step: f64,
    value: Vec2<Option<f64>>,
    min: Option<f64>,
    max: Option<f64>,
};

pub fn vec2_input(props: Vec2Props) -> JSX.Element {
    let {
        disabled = false,
        onChange,
        precision,
        readOnly = false,
        step,
        value,
        min,
        max,
        placeholder,
    } = props;

    let mut mutable_value = vec![None; 2]; // Default to [0.0, 0.0]
    if let Some(v) = value {
        mutable_value = v;
    }

    let mut on_change_callback = Callback::new(move |position: usize, new_value: Option<f64>| {
        let mut new_values = mutable_value.clone();
        new_values[position] = new_value;
        onChange(&mut new_values, min.or_else(|| new_values[0].min(new_values[1]))..max.or_else(|| new_values[0].max(new_values[1])));
    });

    <Stack gap={0.25}>
        <NumberInput
            data-testid="Vec2Input-0"
            size="small"
            disabled={disabled}
            readOnly={readOnly}
            variant="filled"
            fullWidth
            precision=precision as usize
            step=step
            placeholder={placeholder.map(|p| p[0].to_string())}
            value=mutable_value[0]
            min=min.or_else(|| mutable_value[0])
            max=max.or_else(|| mutable_value[0])
            onChange={
                move |new_value: Option<f64>| {
                    on_change_callback.emit(0, new_value);
                }
            }
        />
        <NumberInput
            data-testid="Vec2Input-1"
            size="small"
            disabled={disabled}
            readOnly={readOnly}
            variant="filled"
            fullWidth
            precision=precision as usize
            step=step
            placeholder={placeholder.map(|p| p[1].to_string())}
            value=mutable_value[1]
            min=min.or_else(|| mutable_value[1])
            max=max.or_else(|| mutable_value[1])
            onChange={
                move |new_value: Option<f64>| {
                    on_change_callback.emit(1, new_value);
                }
            }
        />
    </Stack>
}
```