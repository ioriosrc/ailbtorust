```rust
use async_std::{stream::Stream, Future};
use std::collections::HashSet;

pub struct Logger;
impl Logger {
    fn getLogger(_filename: &str) -> Self {
        Self {}
    }
}

type MessageEvent = Vec<();>; // Placeholder for actual message event structure

pub enum Subscription {
    Topic(String),
    Preload(Option<String>),
    ConvertTo { topic: String, converter: Box<dyn Fn(&MessageEvent, &HashSet<&String>) -> Option<Vec<()>>>,
                     convertFrom: Box<dyn Fn(&MessageEvent) -> ()> },
}

type CreateMessageRangeIteratorParams = {
    topic: String,
    convertTo: Option<String>,
    rawBatchIterator: Box<dyn Fn() -> impl Stream<MessageEvent>>,
    sortedTopics: HashSet<&String>,
    messageConverters: HashSet<&str>,
    emitAlert: Box<dyn Fn(&MessageEvent)>,
};

const BATCH_INTERVAL_MS: u64 = 16; // Placeholder for actual batch interval

pub async fn create_message_range_iterator(params: CreateMessageRangeIteratorParams): (
    impl Stream<MessageEvent>,
    impl Fn() -> Result<(), ()>,
) {
    let logger = Logger::getLogger("");

    let mut cancelled = false;

    let message_event_stream = async move {
        match &params.convertTo {
            Some(convert_to) => Subscription::ConvertTo {
                topic: params.topic.clone(),
                converter: Box::new(|msg_event, _| -> Option<Vec<()>> {
                    // Placeholder for actual conversion logic
                    None
                }),
                convert_from: Box::new(|msg_event| {
                    // Placeholder for actual conversion cleanup
                }),
            },
            None => Subscription::Topic(params.topic.clone()),
        };

        let subscription = message_event_stream;

        while !cancelled && !subscription.is_done() {
            let msg_event = subscription.next().await.unwrap();

            if msg_event.type_ != "message-event" {
                continue;
            }

            let msg_event = msg_event.msg_event;

            // Placeholder for actual topic schema conversion logic
            let converted_messages: Vec<();> = Vec::new();

            if !converted_messages.is_empty() {
                yield converted_messages; // No copy needed - we clear it immediately after
            }
        }

        Ok(())
    };

    (message_event_stream, Box::new(move || {
        cancelled = true;
        Ok(())
    }))
}
```