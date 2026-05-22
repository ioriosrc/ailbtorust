```rust
use crate::components::MessagePipeline;
use crate::components::PanelExtensionAdapter;
use crate::components::TopicList::{getDraggedMessagePath, getDraggedMessagePaths};
use crate::components::TopicListSearch;
use crate::components::UseMultiSelection;
use crate::components::UseTopicListSearch;
use crate::services::message_path_dragging::MessagePathSelectionProvider;
use crate::suite_base::{PlayerPresence, TopicListItem};

// Mock dependencies
use mockall::{mock, mock_trait};
mod message_pipeline_impl {
    use super::*;
    pub struct MockMessagePipeline {}
    impl MessagePipeline for MockMessagePipeline {
        fn new(player_presence: PlayerPresence) -> Self {
            Self {}
        }
        fn get_message_path(&self) -> String {
            String::from("path")
        }
    }
}
mod topic_list_search_impl {
    use super::*;
    pub struct MockTopicListSearch {}
    impl TopicListSearch for MockTopicListSearch {
        fn new(tree_items: Vec<TopicListItem>) -> Self {
            Self { tree_items }
        }
    }
}
mock! {
    MessagePathSelectionProvider, {
        fn get_selected_indexes() -> Set<usize>;
        fn on_select(&mut self, index: usize);
    }
}

fn setup(player_presence: PlayerPresence) -> Box<dyn Fn(Vec<TopicListItem>) -> Vec<DraggedMessagePath>> {
    let mut mock = MockTopicListSearch::new(vec![
        TopicListItem {
            type: "topic" as const,
            item: {
                item: { name: "/topic1", schema_name: "Schema1" },
                score: 0,
                positions: Set::from([0, 1]),
                start: 0,
                end: 0,
            },
        },
        TopicListItem {
            type: "topic" as const,
            item: {
                item: { name: "/topic2", schema_name: "Schema2" },
                score: 0,
                positions: Set::from([2, 3]),
                start: 0,
                end: 0,
            },
        },
    ]);

    let mut mock_selection_provider = MockMessagePathSelectionProvider::new();
    mock_selection_provider
        .expect()
        .get_selected_indexes()
        .return_once(|| {
            let mut set = HashSet::new();
            set.insert(2);
            set.insert(0);
            set
        });

    Box::new(move |tree_items| {
        (mock_selection_provider as &mut dyn MessagePathSelectionProvider)
            .expect()
            .on_select(0)
            .return_once(|_| {});

        let mut results = Vec::new();
        for item in tree_items.iter() {
            if mock.expect().get_selected_indexes().iter().contains(&item.index) {
                results.push(get_dragged_message_path(item));
            }
        }

        results
    })
}

#[cfg(test)]
mod topic_list_test {
    use super::*;
    use std::collections::{HashSet, Vec};

    #[test]
    fn test_empty_state_when_player_presence_is_not_present() {
        let setup = setup(PlayerPresence::NOT_PRESENT);
        let rendered = setup(Vec::<TopicListItem>::new());
        assert_eq!(&rendered.text_content(), "No data source selected");
    }

    #[test]
    fn test_empty_state_when_player_presence_is_error() {
        let setup = setup(PlayerPresence::ERROR);
        let rendered = setup(Vec::<TopicListItem>::new());
        assert_eq!(&rendered.text_content(), "An error occurred");
    }

    #[test]
    fn test_loading_state_when_player_presence_is_initializing() {
        let setup = setup(PlayerPresence::INITIALIZING);
        let rendered = setup(Vec::<TopicListItem>::new());
        assert!(rendered.contains("Waiting for data…"));
        assert_eq!(rendered.text_content().contains("16"), true);
    }
}

#[cfg(test)]
mod get_selected_items_test {
    use super::*;
    use std::collections::{HashSet, Vec};

    #[test]
    fn test_returns_empty_array_when_no_indexes_are_selected() {
        let setup = setup(PlayerPresence::PRESENT);
        let selected_indexes = HashSet::new();
        assert_eq!(setup(vec![TopicListItem::default()], selected_indexes), vec![]);
    }

    #[test]
    fn test_returns_dragged_message_paths_for_selected_indexes_in_sorted_order() {
        let setup = setup(PlayerPresence::PRESENT);
        let dragged_path0 = DraggedMessagePath::new("topic1");
        let dragged_path2 = DraggedMessagePath::new("topic3");

        mockall::with_regex!(&setup, "get_dragged_message_path", r"(\w+)").with(move |item| {
            if item == "/topic1" {
                return Some(dragged_path0);
            }
            if item == "/topic3" {
                return Some(dragged_path2);
            }
            None
        });

        assert_eq!(
            setup(vec![
                TopicListItem::new("/topic1", "Schema1"),
                TopicListItem::new("/topic2", "Schema2"),
                TopicListItem::new("/topic3", "Schema3"),
            ], HashSet::from([2, 0])),
            vec![dragged_path0, dragged_path2]
        );
    }

    #[test]
    fn test_filters_out_items_when_index_is_out_of_bounds() {
        let setup = setup(PlayerPresence::PRESENT);
        let dragged_path0 = DraggedMessagePath::new("topic1");

        mockall::with_regex!(&setup, "get_dragged_message_path", r"(\w+)").with(move |item| {
            if item == "/topic1" {
                return Some(dragged_path0);
            }
            None
        });

        assert_eq!(
            setup(vec![
                TopicListItem::new("/topic1", "Schema1"),
                TopicListItem::new("/topic2", "Schema2"),
                TopicListItem::new("/topic3", "Schema3"),
            ], HashSet::from([0, 5])),
            vec![dragged_path0]
        );
    }
}
```