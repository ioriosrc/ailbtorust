```rust
use std::cmp;

use serde_json::{self, Value};

use crate::{
    common::utils::{format_time_raw},
    copy_message_replacer,
    metadata::PropsMetadata,
};

pub fn metadata(props: PropsMetadata) -> String {
    let mut output = format!(
        "base @ {} sec\n",
        format_time_raw(props.message.receive_time)
    );

    if props.diff_message.is_some() {
        output.push_str("\ndiff @ {} sec\n".format(format_time_raw(props.diff_message.unwrap().receive_time)));
        output.push_str("Copy diff of msgs\n");
    }

    output
}
```