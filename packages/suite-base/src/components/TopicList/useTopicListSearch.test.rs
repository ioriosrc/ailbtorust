```rust
use test_case::test;

async fn sort_topics_with_matches_above_matching_paths() {
    let topics = vec![
        TopicListItem { name: "abc".to_string(), schema_name: "ABCD".to_string() },
        TopicListItem { name: "xyz".to_string(), schema_name: "XYZW".to_string() },
    ];
    let datatypes = Default::default();
    let filter_text = "xyz".to_string();

    let result = use_topic_list_search(&topics, &datatypes, &filter_text).await.unwrap();

    assert_eq!(result.map(|item| item.item.name), vec!["xyz", "abc", "abc.xyz"]);
}

async fn sort_topics_with_matches_schema_names_above_matching_paths() {
    let topics = vec![
        TopicListItem { name: "abc".to_string(), schema_name: "ABCD".to_string() },
        TopicListItem { name: "xyz".to_string(), schema_name: "XYZW".to_string() },
    ];
    let datatypes = Default::default();
    let filter_text = "d".to_string();

    let result = use_topic_list_search(&topics, &datatypes, &filter_text).await.unwrap();

    assert_eq!(result.map(|item| item.item.name), vec!["abc", "xyz", "xyz.abcd"]);
}

async fn sort_better_matches_to_top() {
    let topics = vec![
        TopicListItem { name: "footballer".to_string(), schema_name: "ABCD".to_string() },
        TopicListItem { name: "xyz".to_string(), schema_name: "XYZW".to_string() },
    ];
    let datatypes = Default::default();
    let filter_text = "foobar".to_string();

    let result = use_topic_list_search(&topics, &datatypes, &filter_text).await.unwrap();

    assert_eq!(result.map(|item| item.item.name), vec!["xyz", "xyz.foobar", "footballer"]);
}

async fn include_topic_matches_when_trailing_dot() {
    let topics = vec![
        TopicListItem { name: "abc".to_string(), schema_name: "ABCD".to_string() },
        TopicListItem { name: "abc2".to_string(), schema_name: "ABCD".to_string() },
        TopicListItem { name: "xyz".to_string(), schema_name: "XYZW".to_string() },
    ];
    let datatypes = Default::default();
    let filter_text = "abc.".to_string();

    let result = use_topic_list_search(&topics, &datatypes, &filter_text).await.unwrap();

    assert_eq!(result.map(|item| item.item.name), vec!["abc", "abc.xyz", "abc2", "abc2.xyz"]);
}
```