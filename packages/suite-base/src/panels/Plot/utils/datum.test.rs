```rust
use rstest::rstest;
use chrono::{Duration, Utc};

#[rstest]
fn is_chart_value_valid_types() {
    assert!(is_chart_value(42n));
    assert!(is_chart_value(true));
    assert!(is_chart_value(3.14));
    assert!(is_chart_value(BasicBuilder.string()));
}

#[rstest]
fn is_chart_value_time_objects(mock_is_time: bool, mock_to_sec: Option<f64>) {
    let time = Time { sec: 10, nsec: 500 };
    setup(&mock_is_time, &mock_to_sec);

    assert!(is_chart_value(time));
    assert_eq!(is_time.mock_calls(), vec![(Time { sec: 10, nsec: 500 },)]);
}

#[rstest]
fn is_chart_value_unsupported_object_types(mock_is_time: bool) {
    setup(&mock_is_time, None);

    assert!(!is_chart_value({}));
    assert!(!is_chart_value(true));
    assert!(!is_chart_value(false));
    assert!(!is_chart_value(3.14));
    assert!(!is_chart_value("not-a-number"));
}

#[rstest]
fn get_chart_value_valid_types(mock_is_time: bool, mock_to_sec: Option<f64>) {
    let value = 42n;
    setup(&mock_is_time, &mock_to_sec);

    assert_eq!(get_chart_value(value), value);
    assert_eq!(is_chart_value(mock_is_time).mock_calls(), vec![(value,)]);
}

#[rstest]
fn get_chart_value_time_objects(mock_is_time: bool, mock_to_sec: Option<f64>) {
    let time = Time { sec: 10, nsec: 500 };
    setup(&mock_is_time, &mock_to_sec);

    assert_eq!(get_chart_value(time), 10.5);
    assert_eq!(is_chart_value(mock_is_time).mock_calls(), vec![(Time { sec: 10, nsec: 500 },)]);
}

#[rstest]
fn get_chart_value_unsupported_object_types(mock_is_time: bool) {
    setup(&mock_is_time, None);

    assert_eq!(get_chart_value({}), None);
    assert_eq!(get_chart_value(true), None);
    assert_eq!(get_chart_value(false), None);
    assert_eq!(get_chart_value(3.14), None);
    assert_eq!(get_chart_value("not-a-number"), None);
}

fn setup(mock_is_time: &bool, mock_to_sec: Option<&f64>) {
    if *mock_is_time {
        is_chart_value::assert_impl_matches!(is_chart_value, fn _(_: i128) -> bool { true });
        if let Some(val) = mock_to_sec {
            is_chart_value::assert_impl_matches!(to_sec, fn _(_: &Time) -> f64 { val.clone() });
        }
    } else {
        is_chart_value::assert_impl_matches!(is_chart_value, fn _(_: i128) -> bool { false });
        is_chart_value::assert_impl_matches!(to_sec, fn _(_: &Time) -> f64 { 0.0 });
    }
}

#[derive(Debug)]
struct Time {
    sec: i32,
    nsec: u32,
}
```