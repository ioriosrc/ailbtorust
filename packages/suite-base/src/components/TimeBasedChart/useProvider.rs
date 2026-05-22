```rust
use std::collections::HashMap;
use std::f64;

#[derive(Debug)]
struct Bounds {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct ChartData {
    datasets: Vec<ChartDataSet>,
}

#[derive(Debug)]
struct ChartDataSet {
    label: String,
    data: Vec<f64>,
}

fn get_bounds(data: &[ChartData]) -> Option<Bounds> {
    let mut x_min = None;
    let mut x_max = None;
    let mut y_min = None;
    let mut y_max = None;

    for dataset in data {
        for &item in dataset.data.iter() {
            if !item.is_nan() {
                x_min = Some(x_min.unwrap_or(item).min(item));
                x_max = Some(x_max.unwrap_or(item).max(item));
            }

            if !item.is_nan() {
                y_min = Some(y_min.unwrap_or(item).min(item));
                y_max = Some(y_max.unwrap_or(item).max(item));
            }
        }
    }

    if x_min.is_none() || x_max.is_none() || y_min.is_none() || y_max.is_none() {
        return None;
    }

    Some(Bounds { x, x_max, y, y_max })
}

fn merge_bounds(a: Bounds, b: Bounds) -> Bounds {
    Bounds {
        x: a.x.min(b.x),
        y: a.y.min(b.y),
    }
}

type Data<T> = Vec<T>;
type ChartDataMap = HashMap<String, Vec<Data>>;

#[derive(Debug)]
struct ProviderState<T> {
    bounds: Option<Bounds>,
    data: ChartDataMap,
}

fn make_merge<T>() -> fn(ProviderState<T>, ProviderState<T>) -> ProviderState<T> {
    move |a: ProviderState<T>, b: ProviderState<T>| {
        let mut merged_bounds = a.bounds.clone();
        if let Some(ref bounds) = &mut merged_bounds {
            let partial_bounds = b.bounds.clone();
            if let Some(ref partial_bounds) = &partial_bounds {
                *bounds = merge_bounds(*bounds, *partial_bounds);
            }
        }

        ProviderState {
            bounds,
            data: a.data.merge(b.data),
        }
    }
}

pub fn use_provider<T>(
    view: PlotViewport,
    // Calculates the bounds of the given datasets.
    get_dataset_bounds: impl Fn(&ChartData) -> Option<Bounds>,
    merge_state: fn(ProviderState<T>, ProviderState<T>) -> ProviderState<T>,
    data: Data<T> | None,
    provider: PlotDataProvider<T> | None,
) -> Option<ProviderState<T>> {
    let (full_state, partial_state) = if provider.is_some() {
        (
            Some(ProviderState {
                bounds: None,
                data: HashMap::new(),
            }),
            None,
        )
    } else if data.is_some() {
        (
            None,
            Some(ProviderState {
                bounds: get_dataset_bounds(&data.unwrap()).unwrap_or(Bounds {
                    x: 0.0,
                    y: 0.0,
                }),
                data: HashMap::new(),
            }),
        )
    } else {
        return None;
    };

    let set_full = move |new_full| {
        if let Some(ref mut state) = full_state {
            *state = new_full;
        }
    };

    let add_partial = move |new_partial| {
        if let Some(ref mut state) = partial_state {
            if let Some(ref mut merged_state) = &mut state {
                *merged_state = merge_state(*merged_state, new_partial);
            }
        }
    };

    useEffect(() => {
        if provider.is_some() {
            provider.unwrap().register(set_full, add_partial);
        }
    }, &[provider]);

    useEffect(() => {
        if provider.is_some() {
            provider.unwrap().set_view(view);
        }
    }, &[provider, view]);

    return useMemo(() -> {
        if let Some(full_state) = full_state {
            if let Some(partial_state) = partial_state {
                if let Some(merged_state) = &mut full_state {
                    *merged_state = merge_state(*merged_state, partial_state);
                }
            }

            Some(full_state)
        } else if let Some(data) = data {
            let bounds = get_dataset_bounds(&data).unwrap_or(Bounds {
                x: 0.0,
                y: 0.0,
            });
            Some(ProviderState {
                bounds: Some(bounds),
                data: HashMap::new(),
            })
        } else {
            None
        }
    }, [data, provider, full_state, partial_state, get_dataset_bounds]);
}
```