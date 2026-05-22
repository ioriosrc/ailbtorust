```rust
use serde_json::{json, Value};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

fn converter_key(topic: &str, schema: &str) -> String {
    format!("{}\\n{}", topic, schema)
}

/// Convert message into converted_messages using the keyed converters. Modifies
/// converted_messages in place for efficiency.
///
/// # Example
///
/// ```
/// use lichtblick_suite::{convert_message};
/// use lichtblick_suite_base::players::types::PlayerTopic;
/// use lichtblick_suite_base::util::time::format_time_raw;
///
/// let message_event = Immutable::from(message);
/// let converters = Immutable::from(topic_schema_converter_map);
/// let converted_messages = vec![];
/// convert_message(message_event, converters, &converted_messages);
/// ```
pub fn convert_message(
    message_event: Value,
    converters: Vec<InstalledMessageConverter>,
    converted_messages: &mut Vec<Value>,
) {
    let key = converter_key(&message_event["topic"].as_str().unwrap(), &message_event["schema_name"].as_str().unwrap());
    let matched_converters = converters
        .iter()
        .filter_map(|converter| {
            if key == converter.key() {
                Some(converter)
            } else {
                None
            }
        })
        .collect::<Vec<&InstalledMessageConverter>>();

    for converter in matched_converters {
        match converter.convert(&message_event, None) {
            Ok(converted_message) => converted_messages.push(converted_message),
            Err(error) => {
                let alert = json!({
                    "severity": "error",
                    "message": format!(
                        "Uncaught error in message converter ({}) at {}",
                        &converter.extension_id,
                        format_time_raw(&message_event["receiveTime"])
                    ),
                    "error": serde_json::to_string_pretty(error).unwrap(),
                });
                // Emit the alert if an emitAlert function is provided
                if let Some(emitter) = converted_message.get("emitter") {
                    emitter["emit_alert"](alert);
                }
            },
        }
    }
}

/// Returns a new map consisting of all items in `a` not present in `b`.
///
/// # Example
///
/// ```
/// use lichtblick_suite::map_difference;
/// let a = vec!["apple", "banana", "cherry"];
/// let b = vec!["banana", "date"];
/// let difference = map_difference(a, b);
/// assert_eq!(difference, vec!["apple", "cherry"]);
/// ```
pub fn map_difference<K, V>(a: Vec<&K>, b: Option<Vec<&V>>) -> Vec<&K> {
    if b.is_none() {
        return a.clone();
    }
    let b = b.unwrap();
    a.into_iter().filter(|item| !b.contains(item)).collect()
}

/// Builds a set of topics we can render without conversion and a map of
/// converterKey -> converter arguments we use to produce converted messages.
///
/// This will be memoized for performance so the inputs should be stable.
///
/// # Example
///
/// ```
/// use lichtblick_suite::{collate_topic_schema_conversions};
/// use lichtblick_suite_base::players::types::PlayerTopic;
/// use lichtblick_suite_base::util::time::format_time_raw;
///
/// let subscriptions = vec![Subscription {
///     topic: "example",
///     convert_to: Some("json"),
/// }];
/// let sorted_topics = vec![PlayerTopic {
///     name: "example",
///     schema_name: Some("json"),
/// }];
/// let message_converters = None::<Vec<InstalledMessageConverter>>;
/// let conversions = collate_topic_schema_conversions(subscriptions, &sorted_topics, message_converters);
/// assert_eq!(conversions.unconverted_subscription_topics, ["example"]);
/// assert_eq!(conversions.topic_schema_converters.len(), 1);
/// ```
pub fn collate_topic_schema_conversions(
    subscriptions: Vec<Subscription>,
    sorted_topics: Vec<PlayerTopic>,
    message_converters: Option<Vec<InstalledMessageConverter>>,
) -> TopicSchemaConversions {
    let mut topic_schema_converters = HashMap::new();
    let mut unconverted_subscription_topics = HashSet::new();

    // Bin the subscriptions into two sets: those which want a conversion and those that do not.
    //
    // For the subscriptions that want a conversion, if the topic schemaName matches the requested
    // convertTo, then we don't need to do a conversion.
    for subscription in subscriptions {
        if !subscription.convert_to.is_none() {
            unconverted_subscription_topics.insert(subscription.topic.clone());
        }

        // If the convertTo is the same as the original schema for the topic then we don't need to
        // perform a conversion.
        let no_conversion = sorted_topics.iter().any(|topic| topic.name == subscription.topic && topic.schema_name == subscription.convert_to);
        if no_conversion {
            unconverted_subscription_topics.insert(topic.name.clone());
            continue;
        }

        // Since we don't have an existing topic with out destination schema we need to find
        // a converter that will convert from the topic to the desired schema
        let subscriber_topic = sorted_topics.iter().find(|topic| topic.name == subscription.topic);
        if subscriber_topic.is_none() {
            continue;
        }

        let key = converter_key(&subscriber_topic.unwrap().name, &subscriber_topic.unwrap().schema_name.clone());
        let existing_converters = topic_schema_converters.entry(key).or_insert_with(|| Vec::new());

        // We've already stored a converter for this topic to convertTo
        let have_converter = existing_converters.iter().any(|converter| converter.to_schema_name == subscription.convert_to);
        if have_converter {
            continue;
        }

        // Find a converter that can go from the original topic schema to the target schema
        let converters = message_converters.as_ref()
            .filter(|converter| {
                converter.from_schema_name() == subscriber_topic.unwrap().schema_name
                    && converter.to_schema_name() == subscription.convert_to
            })
            .cloned();

        if !converters.is_empty() {
            existing_converters.extend(converters);
            topic_schema_converters.insert(key, &existing_converters);
        }
    }

    TopicSchemaConversions {
        unconverted_subscription_topics,
        topic_schema_converters,
    }
}

/// Function to iterate and call function over multiple sorted arrays in sorted order across all items in all arrays.
/// Time complexity is O(t*n) where t is the number of arrays and n is the total number of items in all arrays.
/// Space complexity is O(t) where t is the number of arrays.
///
/// # Example
///
/// ```
/// use lichtblick_suite::{forEach_sorted_arrays};
/// use serde_json::Value;
///
/// let arrays = vec![
///     vec![1, 3],
///     vec![2, 4, 5],
/// ];
/// let compare_fn = |a: &Value, b: &Value| a.as_i64().unwrap() - b.as_i64().unwrap();
/// let mut values = vec![Value::from(0)];
/// forEach_sorted_arrays(arrays, compare_fn, |item| {
///     values.push(item.clone());
/// });
/// assert_eq!(values, vec![Value::from(0), Value::from(1), Value::from(2), Value::from(3), Value::from(4), Value::from(5)]);
/// ```
pub fn forEach_sorted_arrays<Item>(
    arrays: Vec<Vec<&Item>>,
    compare_fn: impl Fn(&Item, &Item) -> i64,
    mut forEach: impl FnMut(Item),
) {
    let mut cursors = vec![0; arrays.len()];
    if arrays.is_empty() {
        return;
    }
    for (;;) {
        let min_cursor_index = arrays.iter().enumerate().min_by(|&(i, &array)| compare_fn(&array[cursors[i]!], &arrays[0][cursors[0]!])).unwrap();
        let item = arrays[min_cursor_index.0][cursors[min_cursor_index.1]];
        forEach(item);
        cursors[min_cursor_index.1] += 1;
        if cursors[min_cursor_index.1] >= arrays[min_cursor_index.0].len() {
            break;
        }
    }
}
```