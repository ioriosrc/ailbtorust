```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use lichtblick_deno::monads::{unwrap, Option};
use lichtblick_message_path::parse_message_path;
use lichtblick_suite::Immutable;
use lichtblick_suite_base_testing_builders::MessageEventBuilder;
use lichtblick_test_builders::BasicBuilder;

use crate::utils::build_current_series_actions;
use crate::utils::last_matching_topic;

fn make_series_item(key: &str, value: &str) -> HashMap<String, String> {
    let parsed = parse_message_path(value).unwrap();
    HashMap::from([
        ("configIndex".to_string(), "0".to_string()),
        ("parsed".to_string(), parsed.to_string()),
        ("color".to_string(), "red".to_string()),
        ("contrastColor".to_string(), "blue".to_string()),
        ("enabled".to_string(), "true".to_string()),
        ("timestampMethod".to_string(), "receiveTime".to_string()),
        ("key".to_string(), key.to_string()),
        ("lineSize".to_string(), "1".to_string()),
        ("messagePath".to_string(), value.to_string()),
        ("showLine".to_string(), "true".to_string()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use lichtblick_deno::monads::{Option, Ok};
    use lichtblick_message_path::parse_message_path;
    use lichtblick_suite::Immutable;
    use lichtblick_suite_base_testing_builders::MessageEventBuilder;
    use lichtblick_test_builders::BasicBuilder;

    #[test]
    fn last_matching_topic() {
        let match_topic = BasicBuilder.string();
        let other_topic = BasicBuilder.string();

        assert_eq!(last_matching_topic([], match_topic), None);
        assert_eq!(
            last_matching_topic(
                vec![MessageEventBuilder.message_event().with_topic(other_topic).build()],
                match_topic
            ),
            None
        );
        assert_eq!(
            last_matching_topic(
                vec![
                    MessageEventBuilder.message_event().with_topic(match_topic).message(1).build(),
                    MessageEventBuilder.message_event().with_topic(match_topic).message(2).build(),
                ],
                match_topic
            ),
            Some(MessageEventBuilder.message_event().with_topic(match_topic).message(2).build())
        );
        assert_eq!(
            last_matching_topic(
                vec![
                    MessageEventBuilder.message_event().with_topic(other_topic).message(3).build(),
                    MessageEventBuilder.message_event()
                        .with_topic(match_topic)
                        .message(2)
                        .build(),
                ],
                match_topic
            ),
            Some(MessageEventBuilder.message_event().with_topic(match_topic).message(2).build())
        );
    }

    #[test]
    fn build_current_series_actions() {
        let series_a = make_series_item("a", "/topic_a.field");
        let series_b = make_series_item("b", "/topic_b.field");

        assert_eq!(
            build_current_series_actions(vec![], false, |_| vec![1]),
            Ok((
                vec![],
                false
            ))
        );

        let items = vec![42];
        assert_eq!(
            build_current_series_actions(
                vec![series_a.clone()],
                false,
                |_| items
            ),
            Ok((
                vec![
                    Action::AppendCurrent {
                        series: "a".to_string(),
                        items: vec![1],
                    }
                ],
                true
            ))
        );

        assert_eq!(
            build_current_series_actions(
                vec![series_a.clone()],
                true,
                |_| vec![1]
            ),
            Ok((
                vec![
                    Action::ResetCurrent {
                        series: "a".to_string(),
                    },
                    Action::AppendCurrent {
                        series: "a".to_string(),
                        items: vec![1],
                    }
                ],
                true
            ))
        );

        let items = vec![99];
        assert_eq!(
            build_current_series_actions(
                vec![series_a.clone()],
                false,
                |_| vec![2, 3]
            ),
            Ok((
                vec![
                    Action::AppendCurrent {
                        series: "a".to_string(),
                        items: vec![1],
                    }
                ],
                true
            ))
        );

        assert_eq!(
            build_current_series_actions(
                vec![series_a.clone(), series_b.clone()],
                false,
                |_| vec![]
            ),
            Ok((
                vec![
                    Action::AppendCurrent {
                        series: "a".to_string(),
                        items: vec![1],
                    }
                ],
                true
            ))
        );

        assert_eq!(
            build_current_series_actions(
                vec![series_a.clone(), series_b.clone()],
                false,
                |_| vec![]
            ),
            Ok((
                vec![
                    Action::AppendCurrent {
                        series: "b".to_string(),
                        items: vec![2, 3],
                    }
                ],
                true
            ))
        );
    }

    #[test]
    fn build_full_series_actions() {
        let topic = BasicBuilder.string();
        let other_topic = BasicBuilder.string();

        assert_eq!(
            build_full_series_actions(vec![], topic.clone(), false, |_| vec![1]),
            Ok((
                vec![
                    Action::AppendFull {
                        series: "a".to_string(),
                        items: vec![1],
                    }
                ],
                false
            ))
        );

        let items = vec![42];
        assert_eq!(
            build_full_series_actions(
                vec![series_a.clone()],
                topic.clone(),
                false,
                |_| items
            ),
            Ok((
                vec![
                    Action::AppendFull {
                        series: "a".to_string(),
                        items: vec![1],
                    }
                ],
                true
            ))
        );

        assert_eq!(
            build_full_series_actions(
                vec![series_a.clone()],
                topic.clone(),
                true,
                |_| vec![1]
            ),
            Ok((
                vec![
                    Action::ResetFull {
                        series: "a".to_string(),
                    },
                    Action::AppendFull {
                        series: "a".to_string(),
                        items: vec![1],
                    }
                ],
                true
            ))
        );

        let items = vec![99];
        assert_eq!(
            build_full_series_actions(
                vec![series_a.clone()],
                topic.clone(),
                false,
                |_| vec![2, 3]
            ),
            Ok((
                vec![
                    Action::AppendFull {
                        series: "a".to_string(),
                        items: vec![1],
                    }
                ],
                true
            ))
        );

        assert_eq!(
            build_full_series_actions(
                vec![series_a.clone(), series_b.clone()],
                topic.clone(),
                false,
                |_| vec![]
            ),
            Ok((
                vec![
                    Action::AppendFull {
                        series: "a".to_string(),
                        items: vec![1],
                    }
                ],
                true
            ))
        );

        assert_eq!(
            build_full_series_actions(
                vec![series_a.clone(), series_b.clone()],
                topic.clone(),
                false,
                |_| vec![]
            ),
            Ok((
                vec![
                    Action::AppendFull {
                        series: "b".to_string(),
                        items: vec![2, 3],
                    }
                ],
                true
            ))
        );
    }

    fn make_series_item(key: &str, value: &str) -> HashMap<String, String> {
        let parsed = parse_message_path(value).unwrap();
        HashMap::from([
            ("configIndex".to_string(), "0".to_string()),
            ("parsed".to_string(), parsed.to_string()),
            ("color".to_string(), "red".to_string()),
            ("contrastColor".to_string(), "blue".to_string()),
            ("enabled".to_string(), "true".to_string()),
            ("timestampMethod".to_string(), "receiveTime".to_string()),
            ("key".to_string(), key.to_string()),
            ("lineSize".to_string(), "1".to_string()),
            ("messagePath".to_string(), value.to_string()),
            ("showLine".to_string(), "true".to_string()),
        ])
    }
}
```