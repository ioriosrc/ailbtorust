```rust
use std::collections::{HashMap, HashSet};

// Define the types used in the TypeScript code
type Immutable = (); // Assuming Immutable is a placeholder for some immutable data structure
type MessageEvent = (); // Assuming MessageEvent is a placeholder for some event structure

type SeriesConfigKey = String;
type CurrentFrameSeriesItem = (); // Assuming CurrentFrameSeriesItem is a placeholder for some item struct
type Dataset = (); // Assuming Dataset is a placeholder for some dataset struct

// Define the functions used in the TypeScript code
fn set_series(existing: HashMap<SeriesConfigKey, CurrentFrameSeriesItem>, series: Vec<SeriesItem>) -> HashMap<SeriesConfigKey, CurrentFrameSeriesItem> {
    let mut new_series: HashMap<SeriesConfigKey, CurrentFrameSeriesItem> = HashMap::new();
    
    for item in series {
        let existing_series = existing.get(&item.key).cloned();
        let default_series = Some(CurrentFrameSeriesItem { config_index: item.config_index, enabled: item.enabled, message_path: item.message_path, parsed: item.parsed });
        
        new_series.insert(item.key.clone(), existing_series.or(default_series.unwrap()));
    }
    
    new_series
}

fn build_viewport_datasets(series_by_key: HashMap<SeriesConfigKey, CurrentFrameSeriesItem>, paths_with_mismatched_data_lengths: HashSet<&str>) -> Vec<Dataset> {
    let datasets: Vec<Dataset> = series_by_key.into_iter().filter_map(|(_, series)| series.dataset).collect();
    
    return datasets;
}

fn last_matching_topic(msg_events: Vec<MessageEvent>, topic: String) -> Option<MessageEvent> {
    for (i, msg_event) in msg_events.iter().enumerate().rev() {
        if msg_event.topic == topic {
            return Some(msg_event.clone());
        }
    }

    return None;
}

type SeriesCurrentAction<T> = (
    'static,
    &SeriesConfigKey,
    impl FnOnce(Vec<&T>) -> Result<(), ()>,
);

fn build_current_series_actions<TItem>(
    series: Vec<(&SeriesConfigKey, Immutable<SeriesItem>)>,
    options: { did_seek: bool },
    get_items: fn(Immutable<SeriesItem>) -> Vec<TItem>,
) -> (Vec<SeriesCurrentAction<TItem>>, bool) {
    let mut actions: Vec<SeriesCurrentAction<TItem>> = Vec::new();
    let datasets_changed = false;
    
    for (_, series_item) in series {
        if options.did_seek {
            actions.push((
                "reset-current",
                &series_item.key,
                |items| -> Result<(), ()> { Ok(()) },
            ));
        }
        
        let items = get_items(series_item.config);
        datasets_changed |= !items.is_empty();
        actions.push((
            "append-current",
            &series_item.key,
            move |items| {
                for item in items {
                    // Assuming the dataset has a method append
                    series_item.dataset.append(item); // Implement this method accordingly
                }
                
                Ok(())
            },
        ));
    }
    
    (actions, datasets_changed)
}

type SeriesFullAction<T> = (
    'static,
    &SeriesConfigKey,
    impl FnOnce(Vec<&T>) -> Result<(), ()>,
);

fn build_full_series_actions<TItem>(
    series: Vec<(&SeriesConfigKey, Immutable<SeriesItem>)>,
    topic: String,
    options: { is_reset: bool },
    get_items: fn(Immutable<SeriesItem>) -> Vec<TItem>,
) -> (Vec<SeriesFullAction<TItem>>, bool) {
    let mut actions: Vec<SeriesFullAction<TItem>> = Vec::new();
    let datasets_changed = false;
    
    for (_, series_item) in series {
        if series_item.config.parsed.topic_name != topic {
            continue;
        }
        
        if options.is_reset {
            actions.push((
                "reset-full",
                &series_item.key,
                |items| -> Result<(), ()> { Ok(()) },
            ));
        }
        
        let items = get_items(series_item.config);
        datasets_changed |= !items.is_empty();
        actions.push((
            "append-full",
            &series_item.key,
            move |items| {
                for item in items {
                    // Assuming the dataset has a method append
                    series_item.dataset.append(item); // Implement this method accordingly
                }
                
                Ok(())
            },
        ));
    }
    
    (actions, datasets_changed)
}
```