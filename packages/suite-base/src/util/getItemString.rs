```rust
use std::fmt::{self, Display};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use serde_json::Value;
use chrono::{Duration, TimeZone};

pub fn get_arrow(x: f64, y: f64) -> Option<&'static str> {
    if x == 0.0 && y == 0.0 {
        None
    } else {
        Some("→")
    }
}

fn round(x: f64, precision: usize) -> f64 {
    format!("{x:.precision$}").parse::<f64>().unwrap()
}

pub fn get_item_string(
    _node_type: &str,
    data: Value,
    item_type: &'static str,
    item_string: String,
    key_path: Vec<&str>,
    timezone: Option<TimeZone>,
) -> String {
    if let Some(data_obj) = data.as_object() {
        let keys = data_obj.keys().collect::<Vec<&str>>();
        if keys.len() == 2 {
            if let Some(sec) = data_obj.get("sec") {
                if let Some(nsec) = data_obj.get("nsec") {
                    let duration = Duration::seconds_f64(sec as f64) + Duration::nanoseconds_f64(nsec);
                    if duration.as_secs_f64() < DURATION_20_YEARS_SEC {
                        return format!("{:?}", duration);
                    } else {
                        return format!("{:?}", duration.to_std_timezone(timezone));
                    }
                }
            }
        }

        // for vectors/points display length
        if keys.len() == 2 {
            if let Some(x) = data_obj.get("x") {
                if let Some(y) = data_obj.get("y") {
                    let len = (x.powi(2) + y.powi(2)).sqrt();
                    return format!("norm = {:.2} {}", len, get_arrow(x, y));
                }
            }

            if let Some(key) = data_obj.get("key") {
                if let Some(value) = data_obj.get("value") {
                    if PRIMITIVE_TYPES.contains(&value.type().as_str()) {
                        return format!("{}, {}", key, value);
                    }
                }
            }
        } else if keys.len() == 3 {
            if let Some(x) = data_obj.get("x") {
                if let Some(y) = data_obj.get("y") {
                    if let Some(z) = data_obj.get("z") {
                        let len = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
                        return format!("norm = {:.2}{}", len, if z == 0.0 { get_arrow(x, y) } else { "" });
                    }
                }
            }

            if let Some(key) = data_obj.get("r") {
                if let Some(value) = data_obj.get("g") {
                    if let Some(z) = data_obj.get("w") {
                        if PRIMITIVE_TYPES.contains(&key.type().as_str()) && PRIMITIVE_TYPES.contains(&value.type().as_str()) && PRIMITIVE_TYPES.contains(&z.type().as_str()) {
                            return format!("{}, {}, {}", key, value, z);
                        }
                    }
                }
            }
        } else if keys.len() == 4 {
            if let Some(x) = data_obj.get("x") {
                if let Some(y) = data_obj.get("y") {
                    if let Some(z) = data_obj.get("w") {
                        if PRIMITIVE_TYPES.contains(&x.type().as_str()) && PRIMITIVE_TYPES.contains(&y.type().as_str()) && PRIMITIVE_TYPES.contains(&z.type().as_str()) && PRIMITIVE_TYPES.contains(&w.type().as_str()) {
                            let (roll, pitch, yaw) = (x.as_f64(), y.as_f64(), z.as_f64());
                            return format!("rpy = [{:.2}, {:.2}, {:.2}]", roll, pitch, yaw);
                        }
                    }
                }
            }

            if let Some(key) = data_obj.get("r") {
                if let Some(value) = data_obj.get("g") {
                    if let Some(b) = data_obj.get("b") {
                        if let Some(a) = data_obj.get("a") {
                            return format!("{}, {}, {}, {}", key, value, b, a);
                        }
                    }
                }
            }
        }

        // Surface typically-used keys directly in the object summary so the user doesn't have to expand it.
        let filter_keys: Vec<String> = data_obj
            .keys()
            .filter_map(|key| {
                if is_typical_filter_name(&key) && (data_obj.get(key).unwrap().type().as_str() == "number" || PRIMITIVE_TYPES.contains(&data_obj.get(key).unwrap().type().as_str())) {
                    Some(format!("{}, {}", key, data_obj[key]))
                } else {
                    None
                }
            })
            .collect();

        return format!("{}{}", item_type, filter_keys.join(", "));
    }

    item_string
}

fn is_typical_filter_name(name: &str) -> bool {
    // Your typical filter name check logic here
    true
}
```