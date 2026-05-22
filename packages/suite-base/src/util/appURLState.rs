```rust
use std::collections::HashMap;

#[derive(Default)]
struct AppUrlState {
    ds: Option<String>,
    ds_params: HashMap<String, String>,
    layout_id: Option<LayoutID>,
    time: Option<Time>,
}

fn update_app_url_state(url: &str, url_state: &AppUrlState) -> String {
    let mut new_url = Url::parse(url).unwrap();
    
    if let Some(time) = &url_state.time {
        new_url.query_pairs_mut().append("time", time.to_rfc3339());
    }

    if let Some(ds) = &url_state.ds {
        new_url.query_pairs_mut().append("ds", ds);
    }

    if !url_state.ds_params.is_empty() || !url_state.ds_params_array.is_empty() {
        url_state.ds_params.iter()
            .for_each(|(k, v)| new_url.query_pairs_mut().append(&format!("ds.{}", key_map.get(k).unwrap_or(k)), v));
        url_state.ds_params_array
            .iter()
            .for_each(|(k, values)| {
                values.iter().for_each(|value| {
                    new_url.query_pairs_mut().append(&format!("ds.{}", key_map.get(k).unwrap_or(k)), value);
                });
            });
    }

    let mut sorted_pairs = new_url.query_pairs();
    sorted_pairs.sort_unstable_by_key(|(_, v)| v);

    format!("{}", sorted_pairs.collect::<Vec<_>>())
}

fn parse_app_url_state(url: &str) -> Option<AppUrlState> {
    let mut ds = None;
    let time_string = url.parse::<String>().ok()?;
    let time = Time::from_rfc3339(&time_string)?;
    let mut ds_params: HashMap<String, String> = HashMap::new();
    url.split('&')
        .for_each(|kv| {
            if kv.starts_with("ds.") {
                let clean_key = &kv[4..];
                if !ds_params.contains_key(clean_key) {
                    ds_params.insert(clean_key.to_string(), kv.split('=').nth(1).unwrap().to_string());
                } else {
                    if clean_key == "url" {
                        ds_params.get_mut(clean_key).unwrap().push(',');
                        ds_params.get_mut(clean_key).unwrap().push(kv.split('=').nth(1).unwrap().to_string());
                    } else {
                        ds_params.get_mut(clean_key).unwrap().push(kv.split('=').nth(1).unwrap());
                    }
                }
            }
        });

    Some(AppUrlState {
        time,
        ds,
        ds_params: ds_params.into_iter().filter(|(_, v)| !v.is_empty()).collect(),
    })
}

fn window_app_url_state() -> Option<AppUrlState> {
    if let Ok(url) = Url::parse(&std::env::var("REQUEST_URI").unwrap()) {
        parse_app_url_state(&url.to_string())
    } else {
        None
    }
}
```