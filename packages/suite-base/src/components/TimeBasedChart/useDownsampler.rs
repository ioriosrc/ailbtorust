```rust
use crate::types::{ChartDataset, RpcScales, PlotDataProvider};
use downsampled_dataset;

/// useDownsampler downsamples the given datasets before providing them to the
/// TimeBasedChart. This is used primarily for downsampling data passed into the
/// TimeBasedChart component without using a PlotDataProvider.
pub fn use_downsampler(datasets: &[ChartDataset]) -> {
    let mut downsampler = Downsampler::new();

    // Stable callback to run the downsampler and update the latest copy of the downsampled datasets
    let apply_downsample = move || {
        if let Some(setter) = &setter.as_ref() {
            let downsampled = downsampler.downsample();
            if let Some(bounds) = get_bounds(&downsampled) {
                setter({
                    bounds,
                    data: {
                        datasets: downsampled,
                    },
                });
            }
        }
    };

    // Debounce calls to invoke the downsampler
    let queue_downsample = use_debounced_callback(
        apply_downsample,
        100,
        // maxWait equal to debounce timeout makes the debounce act like a throttle
        // Without a maxWait - invocations of the debounced invalidate reset the countdown
        // resulting in no invalidation when scales are constantly changing (playback)
        DebounceOptions::new().leading(false),
    );

    let set_scales = move |scales: RpcScales| {
        downsampler.update({ scales });
        queue_downsample();
    };

    // Updates to the dataset bounds do not need to queue a downsample
    use_effect!({
        if let Some(view) = view.as_ref() {
            downsampler.update({ dataset_bounds: view });
        }
    }, view);

    // Updates to the viewport or the datasets queue a downsample
    use_effect!({
        if let Some(datasets) = datasets.as_ref() {
            downsampler.update({ datasets });
            queue_downsample();
        }
    }, datasets, setter);

    return {
        downsampler: {
            set_view,
            register: move |new Setter| {
                setter(|| new Setter);
            },
        },
        set_scales,
    };
}
```