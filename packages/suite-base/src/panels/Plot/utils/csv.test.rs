```rust
use csv::{Writer, StringRecord};
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

fn generate_csv(data: Vec<HashMap<String, String>>, timestamp_key: &str) -> String {
    let mut writer = Writer::new(vec![]);

    // Write header
    write!(writer, "elapsed time,receive time,header.stamp,topic,value").unwrap();

    // Write data rows
    for item in data.iter() {
        let receive_time = item.get(timestamp_key).map(|t| t.to_string()).unwrap();
        let topic = item.keys().nth(0).unwrap().to_string();
        let value = item.values().next().unwrap().clone();

        writeln!(writer, "{},{},{}\",{}\",\"{}", receive_time, timestamp_key, topic, value).unwrap();
    }

    String::from_utf8(writer.into_inner()).unwrap()
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_csv() {
        let data = vec![
            HashMap::from([
                ("label".to_string(), "value".to_string()),
                ("x".to_string(), "0".to_string()),
                ("y".to_string(), "0".to_string()),
                ("value".to_string(), "0".to_string()),
            ]),
        ];

        let csv = generate_csv(data, "timestamp");
        assert_eq!(
            csv,
            "elapsed time,receive time,header.stamp,topic,value\n0,0.000000000,,label,0\n",
        );

        let data_with_bigint = vec![
            HashMap::from([
                ("label".to_string(), "value".to_string()),
                ("x".to_string(), "0".to_string()),
                ("y".to_string(), "9999999999999001".to_string()),
                ("value".to_string(), "9999999999999001".to_string()),
            ]),
        ];

        let csv_with_bigint = generate_csv(data_with_bigint, "timestamp");
        assert_eq!(
            csv_with_bigint,
            "elapsed time,receive time,header.stamp,topic,value\n0,0.000000000,,label,9999999999999001\n",
        );
    }
}
```