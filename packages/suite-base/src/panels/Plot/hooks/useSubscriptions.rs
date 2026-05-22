```rust
use crate::{
    utils::subscription::path_to_subscribe_payload,
    message_pipeline::{MessagePipelineContext, use_message_pipeline},
};

fn use_subscriptions(config: &PlotConfig, subscriber_id: &str) {
    let paths = config.paths.clone();
    let x_axis_val = config.x_axis_val.clone();

    let global_variables = use_global_variables();

    let set_subscriptions = use_message_pipeline(
        move || {
            move |pipeline_set_subscriptions| pipeline_set_subscriptions,
        },
        vec![],
    );

    // We could subscribe in the chart renderer, but doing it with react effects is easier for
    // managing the lifecycle of the subscriptions. The renderer will correlate input message data to
    // the correct paths/series.
    use_effect! {
        let preload_type = match (x_axis_val == "index" || x_axis_val == "currentCustom") {
            true => SubscriptionPreloadType::Partial,
            false => SubscriptionPreloadType::Full,
        };

        let subscriptions: Vec<_> = paths
            .iter()
            .filter_map(|item| {
                if is_reference_line_plot_path_type(&item.value) {
                    return None;
                }

                let parsed_path = parse_message_path(&item.value);
                if parsed_path.is_none() {
                    return None;
                }

                Some(path_to_subscribe_payload(
                    fill_in_global_variables_in_path(parsed_path.unwrap(), &global_variables),
                    preload_type,
                ))
            })
            .collect();

        if matches!(x_axis_val, "custom" | "currentCustom") && config.x_axis_path.is_some() {
            let parsed_x_axis_path = parse_message_path(&config.x_axis_path.as_ref().unwrap());
            if parsed_x_axis_path.is_none() {
                return;
            }

            subscriptions.push(path_to_subscribe_payload(
                fill_in_global_variables_in_path(parsed_x_axis_path.unwrap(), &global_variables),
                preload_type,
            ));
        }

        set_subscriptions(subscriber_id, subscriptions);

        move || {}
    }
}
```