```rust
mod tests {

    use super::*;

    #[tokio::test]
    async fn maps_subscriptions() {
        let mut fake_player = FakePlayer::new();
        let mappers = vec![TopicAliasFunctions {
            extension_id: "anh",
            alias_function: |args| vec![[args.source_topic_name, "/renamed_topic_1"]]],
        }];
        let player = TopicAliasingPlayer::new(fake_player);
        player.set_alias_functions(mappers);

        player.set_listener(async move {});
        player.set_subscriptions(vec![
            Subscription { topic: "/renamed_topic_1", fields: vec!["a", "b"] },
            Subscription { topic: "/topic_2", fields: vec!["c", "d"] },
        ]);

        fake_player.emit(mock_player_state(None, |state| {
            let mut topics = state.topics.to_vec();
            topics.push(Topic {
                name: "/original_topic_1".to_string(),
                schema_name: Some("any.schema".to_string()),
            });

            Ok(topics)
        }));

        assert_eq!(
            fake_player.subscriptions,
            vec![
                Subscription { topic: "/renamed_topic_1", fields: vec!["a", "b"] },
                Subscription { topic: "/topic_2", fields: vec!["c", "d"] },
            ]
        );
    }

    #[tokio::test]
    async fn preserves_sliced_subscriptions() {
        let mut fake_player = FakePlayer::new();
        let mappers = vec![TopicAliasFunctions {
            extension_id: "anh",
            alias_function: |args| vec![[args.source_topic_name, "/renamed_topic_1"]]],
        }];
        let player = TopicAliasingPlayer::new(fake_player);
        player.set_alias_functions(mappers);

        player.set_listener(async move {});
        player.set_subscriptions(vec![
            Subscription { topic: "/renamed_topic_1", fields: vec!["a", "b"] },
            Subscription { topic: "/topic_2", fields: vec!["c", "d"] },
        ]);

        fake_player.emit(mock_player_state(None, |state| {
            let mut topics = state.topics.to_vec();
            topics.push(Topic {
                name: "/original_topic_1".to_string(),
                schema_name: Some("any.schema".to_string()),
            });

            Ok(topics)
        }));

        assert_eq!(
            fake_player.subscriptions,
            vec![
                Subscription { topic: "/renamed_topic_1", fields: vec!["a", "b"] },
                Subscription { topic: "/topic_2", fields: vec!["c", "d"] },
            ]
        );
    }

    #[tokio::test]
    async fn maps_messages() {
        let mut fake_player = FakePlayer::new();
        let mappers = vec![TopicAliasFunctions {
            extension_id: "any",
            alias_function: |args| vec![[args.source_topic_name, "/renamed_topic_1"]]],
        }];
        let player = TopicAliasingPlayer::new(fake_player);
        player.set_alias_functions(mappers);

        let listener = async move {};
        player.set_listener(listener);
        fake_player.emit(mock_player_state(None, |state| {
            Ok(state)
        }));

        assert_eq!(
            state.active_data.messages,
            vec![
                mock_message("message", { topic: "/original_topic_1" }),
                mock_message("message", { topic: "/renamed_topic_1" }),
                mock_message("message", { topic: "/topic_2" }),
            ]
        );
    }

    #[tokio::test]
    async fn marks_disallowed_mappings_as_player_alerts() {
        let mut fake_player = FakePlayer::new();
        let mappers = vec![
            TopicAliasFunctions {
                extension_id: "ext1",
                alias_function: |args| vec![[args.source_topic_name, "/original_topic_1"]]],
            },
            TopicAliasFunctions {
                extension_id: "ext2",
                alias_function: |args| vec![[args.source_topic_name, "/original_topic_1"]]],
            },
        ];
        let player = TopicAliasingPlayer::new(fake_player);
        player.set_alias_functions(mappers);

        let listener = async move {};
        player.set_listener(listener);
        fake_player.emit(mock_player_state(None, |state| {
            Ok(state)
        }));

        assert_eq!(
            state.alerts,
            vec![
                PlayerAlert {
                    message: "Disallowed topic alias",
                    tip: "Extension ext1 aliased topic /original_topic_1 is already present in the data source.",
                    severity: Severity::Error,
                },
                PlayerAlert {
                    message: "Disallowed topic alias",
                    tip: "Extension ext2 requested duplicate alias from topic /original_topic_1 to topic /renamed_topic_1.",
                    severity: Severity::Error,
                },
            ]
        );
    }

    #[tokio::test]
    async fn maps_blocks() {
        let mut fake_player = FakePlayer::new();
        let mappers = vec![TopicAliasFunctions {
            extension_id: "any",
            alias_function: |args| vec![[args.source_topic_name, "/renamed_topic_1"]]],
        }];
        let player = TopicAliasingPlayer::new(fake_player);
        player.set_alias_functions(mappers);

        let listener = async move {};
        player.set_listener(listener);
        fake_player.emit(mock_player_state(None, |state| {
            Ok(state)
        }));

        assert_eq!(
            state.active_data.messages,
            vec![
                mock_message("message", { topic: "/original_topic_1" }),
                mock_message("message", { topic: "/renamed_topic_1" }),
                mock_message("message", { topic: "/topic_2" }),
            ]
        );
    }

    #[tokio::test]
    async fn provides_global_variables_on_startup() {
        let mut fake_player = FakePlayer::new();
        let mappers = vec![TopicAliasFunctions {
            extension_id: "some-id",
            alias_function: |args| if !args.global_variables.contains_key("doMap") { vec![] } else { vec![["/original_topic_1", "/renamed_topic_1"]]] },
        }];
        let player = TopicAliasingPlayer::new(fake_player);
        player.set_alias_functions(mappers);

        assert_eq!(
            player.get_metadata(),
            vec![
                {
                    name: "metadataFake",
                    metadata: { key: "value" },
                },
            ]
        );
    }

    #[tokio::test]
    async fn updates_when_global_variables_change() {
        let mut fake_player = FakePlayer::new();
        let mappers = vec![TopicAliasFunctions {
            extension_id: "any",
            alias_function: |args| if !args.global_variables.contains_key("doMap") { vec![] } else { vec![["/original_topic_1", "/renamed_topic_1"]]] },
        }];
        let player = TopicAliasingPlayer::new(fake_player);
        player.set_alias_functions(mappers);

        let listener = async move {};
        player.set_listener(listener);
        fake_player.emit(mock_player_state(None, |state| {
            Ok(state)
        }));

        assert_eq!(
            state.active_data.messages,
            vec![
                mock_message("message", { topic: "/original_topic_1" }),
                mock_message("message", { topic: "/renamed_topic_1" }),
                mock_message("message", { topic: "/topic_2" }),
            ]
        );

        listener.mock_clear();

        // Setting should re-process the alias functions
        player.set_global_variables(&{"doMap": "true"});
        assert_eq!(
            state.active_data.messages,
            vec![
                mock_message("message", { topic: "/original_topic_1" }),
                mock_message("message", { topic: "/renamed_topic_1" }),
                mock_message("message", { topic: "/topic_2" }),
            ]
        );
    }

    #[tokio::test]
    async fn updates_subscriptions_when_global_variable_changes_update_aliases() {
        let mut fake_player = FakePlayer::new();
        let mappers = vec![TopicAliasFunctions {
            extension_id: "some-id",
            alias_function: |args| if !args.global_variables.contains_key("doMap") { vec![] } else { vec![["/original_topic_1", "/renamed_topic_1"]]] },
        }];
        let player = TopicAliasingPlayer::new(fake_player);
        player.set_alias_functions(mappers);

        let listener = async move {};
        player.set_listener(listener);
        fake_player.emit(mock_player_state(None, |state| {
            Ok(state)
        }));

        assert_eq!(
            state.active_data.messages,
            vec![
                mock_message("message", { topic: "/original_topic_1" }),
                mock_message("message", { topic: "/renamed_topic_1" }),
                mock_message("message", { topic: "/topic_2" }),
            ]
        );

        // update the global variables to create an alias
        player.set_global_variables(&{"doMap": "true"});
        assert_eq!(
            state.active_data.messages,
            vec![
                mock_message("message", { topic: "/original_topic_1" }),
                mock_message("message", { topic: "/renamed_topic_1" }),
                mock_message("message", { topic: "/topic_2" }),
            ]
        );
    }

    #[tokio::test]
    async fn returns_the_correct_metadata() {
        let fake_player = FakePlayer::new();
        let player = TopicAliasingPlayer::new(fake_player);

        assert_eq!(
            player.get_metadata(),
            vec![
                {
                    name: "metadataFake",
                    metadata: { key: "value" },
                },
            ]
        );
    }
}
```