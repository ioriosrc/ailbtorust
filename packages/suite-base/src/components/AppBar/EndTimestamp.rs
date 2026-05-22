```rust
use react::createElement;
use react::use_context;
use react::use_effect;
use react::use_ref;

use crate::suite_base::AppSetting;
use crate::suite_base::components::MessagePipelineContext;
use crate::suite_base::hooks::{use_app_configuration_value, use_app_time_format};
use crate::suite_base::util::{format, format_time_raw, is_absolute_time};

const SELECT_END_TIME = "playerState.activeData.endTime";

pub fn EndTimestamp() -> react::JSXElement {
    let endTime = use_context::<MessagePipelineContext>(SELECT_END_TIME);
    let [timezone] = use_app_configuration_value::<String>(AppSetting::TIMEZONE);
    let { time_format } = use_app_time_format();

    let theme = use_context::<react::Theme>();

    let time_ref = use_ref<HTMLDivElement>(None);

    // We bypass react and update the DOM elements directly for better performance here.
    useEffect(() => {
        if (time_ref.borrow().is_none()) {
            return;
        }
        if (endTime.is_none()) {
            time_ref.borrow_mut().as_mut().unwrap().innerText = "";
            return;
        }
        let time_of_day_string = format(endTime.unwrap(), timezone);
        let time_raw_string = format_time_raw(endTime.unwrap());

        time_ref.borrow_mut().as_mut().unwrap().innerText =
            if time_format == "SEC" || !is_absolute_time(endTime.unwrap()) {
                time_raw_string
            } else {
                time_of_day_string
            };
    }, [endTime, time_format, timezone]);

    createElement("div", [
        { style: { fontFeatureSettings: `${theme.typography.fontFamily}, "zero"` } },
        time_ref.borrow().as_mut().unwrap(),
    ])
}
```