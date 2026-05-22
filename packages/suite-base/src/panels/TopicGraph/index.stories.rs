```rust
use asyncronous::{Async, FutureExt};
use futures::stream::{FuturesUnordered, StreamExt};
use parking_lot::Mutex;
use std::sync::Arc;

use crate::{
    fixture::{Fixture, TopicVisibility},
    panel_setup::PanelSetup,
    topic_graph::TopicGraph,
};

pub fn empty_story() -> impl FnOnce() -> Async<()> {
    async move {
        <PanelSetup as Async>::block_on(async move {
            let fixture = Fixture {
                frame: {},
                topics: vec![Fixture::topic("topic", "std_msgs/Header")],
                active_data: Fixture::active_data(vec![
                    (String::from("/topic"), vec![String::from("pub-1"), String::from("pub-2")]),
                    (
                        String::from("/topic_without_subscriber"),
                        vec![String::from("pub-1"), String::from("pub-2")],
                    ),
                ]),
            };

            <PanelSetup as Async>::block_on(async move {
                let fixture = fixture.clone();
                let panel_setup = PanelSetup::new(fixture);
                let topic_graph = TopicGraph::new(panel_setup);

                <TopicGraph as Async>::block_on(topic_graph.run());
            });
        });
    }
}

pub fn with_settings_story() -> impl FnOnce() -> Async<()> {
    async move {
        <PanelSetup as Async>::block_on(async move {
            let fixture = Fixture {
                frame: {},
                topics: vec![Fixture::topic("topic", "std_msgs/Header")],
                active_data: Fixture::active_data(vec![
                    (String::from("/topic"), vec![String::from("pub-1"), String::from("pub-2")]),
                    (
                        String::from("/topic_without_subscriber"),
                        vec![String::from("pub-1"), String::from("pub-2")],
                    ),
                ]),
            };

            <PanelSetup as Async>::block_on(async move {
                let fixture = fixture.clone();
                let panel_setup = PanelSetup::new(fixture);
                let topic_graph = TopicGraph::new(panel_setup);

                <TopicGraph as Async>::block_on(topic_graph.run());
            });
        });
    }
}

pub async fn topics_story(initial_topic_visibility: &str) -> impl FnOnce() -> Async<()> {
    async move {
        let fixture = Fixture {
            frame: {},
            topics: vec![Fixture::topic("topic", "std_msgs/Header")],
            active_data: Fixture::active_data(vec![
                (String::from("/topic"), vec![String::from("pub-1"), String::from("pub-2")]),
                (
                    String::from("/topic_without_subscriber"),
                    vec![String::from("pub-1"), String::from("pub-2")],
                ),
            ]),
        };

        let panel_setup = PanelSetup::new(fixture.clone());
        let topic_graph = TopicGraph::new(panel_setup);

        let mut topics_stream =
            FuturesUnordered::from(vec![
                <PanelSetup as Async>::block_on(async move {
                    document
                        .query_selector("button[data-testid='set-topic-visibility']")
                        .expect("Could not find set topic visibility button")
                        .click();

                    let radio_option = document
                        .query_selector(`[data-testid="${initial_topic_visibility}"]`)
                        .expect("Could not find radio option");

                    radio_option.click();
                    document
                        .query_selector("div[data-testid='set-topic-visibility'] button:last-child")
                        .expect("Could not find set topic visibility button")
                        .click();
                }),
            ]);

        <TopicGraph as Async>::block_on(topic_graph.run());

        topics_stream.await;
    }
}

pub fn all_topics_story() -> impl FnOnce() -> Async<()> {
    async move {
        <PanelSetup as Async>::block_on(async move {
            let fixture = Fixture {
                frame: {},
                topics: vec![Fixture::topic("topic", "std_msgs/Header")],
                active_data: Fixture::active_data(vec![
                    (String::from("/topic"), vec![String::from("pub-1"), String::from("pub-2")]),
                    (
                        String::from("/topic_without_subscriber"),
                        vec![String::from("pub-1"), String::from("pub-2")],
                    ),
                ]),
            };

            let panel_setup = PanelSetup::new(fixture.clone());
            let topic_graph = TopicGraph::new(panel_setup);

            <TopicGraph as Async>::block_on(topic_graph.run());
        });
    }
}

pub fn topics_with_subscribers_story() -> impl FnOnce() -> Async<()> {
    async move {
        <PanelSetup as Async>::block_on(async move {
            let fixture = Fixture {
                frame: {},
                topics: vec![Fixture::topic("topic", "std_msgs/Header")],
                active_data: Fixture::active_data(vec![
                    (String::from("/topic"), vec![String::from("pub-1"), String::from("pub-2")]),
                    (
                        String::from("/topic_without_subscriber"),
                        vec![String::from("pub-1"), String::from("pub-2")],
                    ),
                ]),
            };

            let panel_setup = PanelSetup::new(fixture.clone());
            let topic_graph = TopicGraph::new(panel_setup);

            <TopicGraph as Async>::block_on(topic_graph.run());
        });
    }
}

pub fn topics_hidden_story() -> impl FnOnce() -> Async<()> {
    async move {
        <PanelSetup as Async>::block_on(async move {
            let fixture = Fixture {
                frame: {},
                topics: vec![Fixture::topic("topic", "std_msgs/Header")],
                active_data: Fixture::active_data(vec![
                    (String::from("/topic"), vec![String::from("pub-1"), String::from("pub-2")]),
                    (
                        String::from("/topic_without_subscriber"),
                        vec![String::from("pub-1"), String::from("pub-2")],
                    ),
                ]),
            };

            let panel_setup = PanelSetup::new(fixture.clone());
            let topic_graph = TopicGraph::new(panel_setup);

            <TopicGraph as Async>::block_on(topic_graph.run());
        });
    }
}

pub fn re_layout_story() -> impl FnOnce() -> Async<()> {
    async move {
        let fixture = Fixture {
            frame: {},
            topics: vec![Fixture::topic("topic", "std_msgs/Header")],
            active_data: Fixture::active_data(vec![
                (String::from("/topic"), vec![String::from("pub-1"), String::from("pub-2")]),
                (
                    String::from("/topic_without_subscriber"),
                    vec![String::from("pub-1"), String::from("pub-2")],
                ),
            ]),
        };

        let panel_setup = PanelSetup::new(fixture.clone());
        let topic_graph = TopicGraph::new(panel_setup);

        let mut topics_stream =
            FuturesUnordered::from(vec![
                <PanelSetup as Async>::block_on(async move {
                    document
                        .query_selector("button[data-testid='set-topic-visibility']")
                        .expect("Could not find set topic visibility button")
                        .click();

                    let radio_option = document
                        .query_selector(`[data-testid="${initial_topic_visibility}"]`)
                        .expect("Could not find radio option");

                    radio_option.click();
                    document
                        .query_selector("div[data-testid='set-topic-visibility'] button:last-child")
                        .expect("Could not find set topic visibility button")
                        .click();
                }),
            ]);

        <TopicGraph as Async>::block_on(topic_graph.run());

        topics_stream.await;
    }
}
```