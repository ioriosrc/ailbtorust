```rust
use crate::chart::{Bounds, ScaleOptions};
use crate::data_source::StateTransitionConfig;

pub fn use_chart_scales_and_bounds(
    min_y: Option<f64>,
    current_time_since_start: Option<u32>,
    end_time_since_start: Option<u32>,
    config: StateTransitionConfig,
) -> (ScaleOptions<Linear>, ScaleOptions<Linear>, Bounds, Option<usize>, RefCallback<HTMLDivElement>) {
    let y_scale = useMemo<ScaleOptions<Linear>>(|| {
        return {
            ticks: {
                // Hide all y-axis ticks since each bar on the y-axis is just a separate path.
                display: false,
            },
            grid: {
                display: false,
            },
            type: "linear",
            min: min_y.unwrap_or_default(),
            max: -3.,
        };
    }, [min_y]);

    let x_scale = useMemo<ScaleOptions<Linear>>(|| {
        return {
            type: "linear",
            border: {
                display: false,
            },
        };
    }, []);

    // Compute the fixed bounds (either via min/max x-axis config or end time since start).
    //
    // For recordings, the bounds are actually fixed but for live connections the "endTimeSinceStart"
    // will increase and these bounds are not technically fixed. But in those instances there is also
    // new data coming in when the bounds are changing.
    //
    // We need to keep the fixedBounds reference stable (if it can be stable) for the databounds memo
    // below, otherwise playing through a recording will update the currentTimeSince start and return
    // a new fixedBounds reference which causes expensive downstream rendering.
    let fixed_bounds = useMemo(|| {
        if end_time_since_start.is_none() {
            return None;
        }

        if config.x_axis_min_value.is_some() || config.x_axis_max_value.is_some() {
            return Some({
                x: {
                    min: config.x_axis_min_value.unwrap_or_default(),
                    max: config.x_axis_max_value.unwrap_or(end_time_since_start),
                },
                y: { min: f64::MIN_VALUE, max: f64::MAX_VALUE },
            });
        }

        // If we have no configured xAxis min/max or range, then we set the x axis max to end time
        // This will mirror the plot behavior of showing the full x-axis for data time range rather
        // than constantly adjusting the end time to the latest loaded state transition while data
        // is loading.
        Some({
            x: { min: 0, max: end_time_since_start.unwrap_or_default() },
            y: { min: f64::MIN_VALUE, max: f64::MAX_VALUE },
        });
    }, [config.x_axis_max_value, config.x_axis_min_value, end_time_since_start]);

    // Compute the data bounds. The bounds are either a fixed amount of lookback from the current time
    // or they are fixed bounds with a specific range.
    let databounds: Option<Bounds> = useMemo(|| {
        if config.x_axis_range.is_some() && current_time_since_start.is_some() {
            return Some({
                x: {
                    min: current_time_since_start.unwrap_or_default() - config.x_axis_range.unwrap(),
                    max: current_time_since_start.unwrap_or_default(),
                },
                y: { min: f64::MIN_VALUE, max: f64::MAX_VALUE },
            });
        }

        fixed_bounds.clone()
    }, [config.x_axis_range, current_time_since_start, fixed_bounds]);

    let width = Some(0);
    let size_ref = || {
        // We need to use a wrapper div as the chart component uses wheel events for zoom and pan.
        // After adding more series, the logic expands the chart element beyond the visible area of the panel.
        // When this happens, scrolling on the chart also scrolls the chart wrapper div and results in zooming that chart AND scrolling
        // the panel. This behavior is undesirable.
        // We disable the wheel event for the chart wrapper div (which is where we use size_ref)
        //
        // The chart component uses wheel events for zoom and pan. After adding more series, the logic
        // expands the chart element beyond the visible area of the panel.
        // When this happens, scrolling on the chart also scrolls the chart wrapper div and results in zooming that chart AND scrolling
        // the panel. This behavior is undesirable.
        let el = document.getElementById("chart_wrapper") as HTMLDivElement;
        if (el) {
            return RefCallback::new(el);
        }

        None
    };

    (y_scale, x_scale, databounds, width, size_ref)
}
```