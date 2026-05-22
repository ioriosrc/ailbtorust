```rust
use crate::suite_base::{AppConfigurationContext, AppSetting};
use crate::suite_base::{make_mock_app_configuration, Time};
use crate::rostime::Time;

const ABSOLUTE_TIME = Time {
    sec: 1643800942,
    nsec: 222222222,
};

const RELATIVE_TIME = Time {
    sec: 630720000,
    nsec: 597648236,
};

pub fn timestamp_story(props: impl PropsWithChildren<Props>) -> React::JSX.Element {
    let config = props.props.config.unwrap_or(vec![]);
    let time = props.props.time;

    let value = make_mock_app_configuration(config);

    <AppConfigurationContext.Provider value={value}>
        <Stack padding="2" spacing="2">
            <Timestamp horizontal=time />
            <Timestamp time=time />
            <Timestamp disable_date=time />
        </Stack>
    </AppConfigurationContext.Provider>
}

pub fn default_timestamp_story() -> React::JSX.Element {
    let config = vec![vec![AppSetting::TIMEZONE, "UTC"]];
    let time = ABSOLUTE_TIME;

    timestamp_story(props! {config => config, time => time})
}

pub fn time_format_seconds_timestamp_story() -> React::JSX.Element {
    let config = vec![
        vec![AppSetting::TIME_FORMAT, "SEC"],
        vec![AppSetting::TIMEZONE, "UTC"],
    ];
    let time = ABSOLUTE_TIME;

    timestamp_story(props! {config => config, time => time})
}

pub fn time_format_tod_timestamp_story() -> React::JSX.Element {
    let config = vec![
        vec![AppSetting::TIME_FORMAT, "TOD"],
        vec![AppSetting::TIMEZONE, "UTC"],
    ];
    let time = ABSOLUTE_TIME;

    timestamp_story(props! {config => config, time => time})
}

pub fn time_format_relative_timestamp_story() -> React::JSX.Element {
    let config = vec![
        vec![AppSetting::TIME_FORMAT, "TOD"],
        vec![AppSetting::TIMEZONE, "UTC"],
    ];
    let time = RELATIVE_TIME;

    timestamp_story(props! {config => config, time => time})
}
```