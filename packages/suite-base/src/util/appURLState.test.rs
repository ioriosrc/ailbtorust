```rust
use url::Url;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
mod app_url_state_test {
    use crate::utils::{BasicBuilder, Time};
    use chrono::{DateTime, Utc};
    use time::{Duration, SystemTime as Timestamp};

    fn mock_is_desktop() -> bool {
        false
    }

    #[test]
    fn test_parse_app_url_state() {
        let is_desktop = mock_is_desktop();
        let url_builder = |is_desktop: bool| {
            if is_desktop {
                Url::parse("lichtblick://host/open").unwrap()
            } else {
                Url::parse("https://studio.foxglove.dev/").unwrap()
            }
        };

        // Note that the foxglove URL here is different from actual foxglove URLs because Node's URL parser
        // interprets lichtblick:// URLs differently than the browser does.

        let mut cases = vec![
            (
                true,
                url_builder(true),
                None,
                "lichtblick://host/open",
            ),
            (
                false,
                url_builder(false),
                Some("http://example.com"),
                "https://studio.foxglove.dev/?ds=ros1-remote-bagfile&ds.url=http%3A%2F%2Fexample.com",
            ),
        ];

        for (is_desktop, url, ds_params, expected_url) in cases {
            let parsed = parse_app_url_state(url, is_desktop);
            assert_eq!(parsed.href(), expected_url);
        }
    }

    fn parse_app_url_state(url: Url, is_desktop: bool) -> Url {
        if !is_desktop {
            return url;
        }

        // Implement the logic to parse and return the parsed URL
        // This may involve parsing the search parameters and other necessary components
        // based on the specific requirements of the application
        unreachable!()
    }
}

mod update_app_url_state_test {
    use crate::utils::{BasicBuilder, Time};
    use url::Url;
    use std::time::{Duration, SystemTime as Timestamp};

    #[cfg(test)]
    mod test_update_app_url_state() {
        use crate::utils::{BasicBuilder, Time};
        use chrono::{DateTime, Utc};
        use time::{Duration, SystemTime as Timestamp};

        fn mock_is_desktop() -> bool {
            false
        }

        #[test]
        fn test_encodes_rosbag_urls() {
            let url = BasicBuilder.string();
            let event_id = BasicBuilder.string();
            let url_state: AppURLState = {
                ds_params: Some(BasicBuilder.map_strings("url", |s| s)),
                ds: "ros1-remote-bagfile",
                time: None,
            };

            let result = update_app_url_state(url, &event_id, &url_state);
            assert_eq!(result.href(), format!("{}?ds=ros1-remote-bagfile&ds.url={}", BasicBuilder.string(), url));
        }

        fn test_encodes_multiple_remote_files_urls() {
            let urls: Vec<Url> = vec![
                Url::parse("http://localhost:8080/bag").unwrap(),
                Url::parse("http://localhost:8081/bag").unwrap(),
            ];
            let event_id = BasicBuilder.string();
            let url_state: AppURLState = {
                ds_params_array: Some(BasicBuilder.map_strings("urls", |s| s)),
                ds: "remote-file",
                time: None,
            };

            let result = update_app_url_state(urls[0].clone(), &event_id, &url_state);
            assert_eq!(
                result.href(),
                format!("{}?ds=remote-file&ds.url={}&ds.url={}", BasicBuilder.string(), urls[0], urls[1])
            );
        }

        fn test_encodes_ds_param_key_substitutes() {
            let key = BasicBuilder.string();
            let param_array: Vec<String> = vec![BasicBuilder.string(), BasicBuilder.string()];
            let event_id = BasicBuilder.string();
            let url_state: AppURLState = {
                ds_params_array: Some(BasicBuilder.map_strings(key, |s| s)),
                ds: "remote-file",
                time: None,
            };

            let result = update_app_url_state(BasicBuilder.string(), &event_id, &url_state);
            assert_eq!(
                result.href(),
                format!("{}?ds=remote-file&ds.{}={}&ds.{}={}", BasicBuilder.string(), key, param_array[0], key, param_array[1])
            );
        }

        fn update_app_url_state(
            origin: Url,
            event_id: &str,
            url_state: &AppURLState,
        ) -> Url {
            // Implement the logic to encode and return the updated URL
            // This may involve parsing the search parameters, adding new ones, or modifying existing ones
            unreachable!()
        }
    }

    #[derive(Default)]
    struct AppURLState {
        ds_params: Option<BasicBuilderMap<String, String>>,
        ds_params_array: Option<BasicBuilderMap<String, Vec<String>>>,
        ds: String,
        time: Option<Time>,
    }

    fn basic_builder_map<K, V>(key: K, value: V) -> BasicBuilderMap<K, V> {
        BasicBuilderMap::from([(key, value)])
    }
}
```