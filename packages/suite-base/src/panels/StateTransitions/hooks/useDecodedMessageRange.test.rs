```rust
use async_std::prelude::*;
use mockall::{mock, Mock};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// Define the necessary types from the TypeScript/React codebase
type MessageEvent = Vec<u8>; // Placeholder for actual message event type
type SubscribeMessageRangeArgs = (String, Arc<Mutex<dyn Fn(MessageEvent)>>); // Placeholder for actual args type

mock! {
    struct MockDecodeMessagePathsForMessagesByTopic {
        fn decode_message_paths_for_messages_by_topic(
            &self,
            topic: String,
        ) -> HashMap<String, Vec<MessageEvent>>;
    }
}

async fn simulate_batches(topic: String, batches: Vec<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    // Implement the logic to simulate batching of messages
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::{prelude::*};
    use mockall::{mock, Mock};

    #[test]
    fn test_subscribe_to_topics() {
        let topic_a = BasicBuilder.string();
        let topic_b = BasicBuilder.string();

        let mut decode_paths = Arc::new(Mutex::new(|_| HashMap::new()));
        let mock_decode_path = Box::new(move |msg| decode_paths.lock().unwrap().insert(topic_a.clone(), vec![msg]));
        let mock_decode_path2 = Box::new(move |msg| decode_paths.lock().unwrap().insert(topic_b.clone(), vec![msg]));

        let mut subscribe_message_range = MockSubscribeMessageRangeArgs {
            0: topic_a.clone(),
            1: Arc::new(Mutex::new(mock_decode_path)),
        };

        let mut mock_decode_msg_paths_for_messages_by_topic = MockDecodeMessagePathsForMessagesByTopic;
        mock_decode_msg_paths_for_messages_by_topic
            .expect_decode_message_paths_for_messages_by_topic()
            .withf(move |topic| topic == topic_a)
            .return_once(|_| HashMap::new());

        let result = use_decoded_message_range(vec![topic_a.clone(), topic_b.clone()], vec![BasicBuilder.string()]);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [HashMap::from([(topic_a.clone(), vec![BasicBuilder.string().unwrap()])])]
        );
    }

    #[test]
    fn test_cancel_subscriptions_on_unmount() {
        let topic = BasicBuilder.string();

        let mut decode_paths = Arc::new(Mutex::new(|_| HashMap::new()));
        let mock_decode_path = Box::new(move |msg| decode_paths.lock().unwrap().insert(topic.clone(), vec![msg]));

        let mut subscribe_message_range = MockSubscribeMessageRangeArgs {
            0: topic.clone(),
            1: Arc::new(Mutex::new(mock_decode_path)),
        };

        let result = use_decoded_message_range(vec![topic.clone()], vec![BasicBuilder.string()]);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [HashMap::from([(topic.clone(), vec![BasicBuilder.string().unwrap()])])]
        );

        let unmount = Arc::new(Mutex::new(()));
        use_decoded_message_range(vec![topic.clone()], vec![BasicBuilder.string()]);

        assert!(unmount.lock().unwrap() == ());
    }

    #[test]
    fn test_accumulate_messages_and_decode_after_flush() {
        let topic = BasicBuilder.string();

        let mut decode_paths = Arc::new(Mutex::new(|_| HashMap::new()));
        let mock_decode_path = Box::new(move |msg| decode_paths.lock().unwrap().insert(topic.clone(), vec![msg]));

        let mut subscribe_message_range = MockSubscribeMessageRangeArgs {
            0: topic.clone(),
            1: Arc::new(Mutex::new(mock_decode_path)),
        };

        let result = use_decoded_message_range(vec![topic.clone()], vec![BasicBuilder.string()]);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [HashMap::from([(topic.clone(), vec![BasicBuilder.string().unwrap()])])]
        );

        let msgs = [
            MessageEventBuilder.message_event({ topic }),
            MessageEventBuilder.message_event({ topic }),
        ];

        simulate_batches(topic, vec![msgs.clone()]).await.unwrap();

        assert_eq!(
            *decode_paths.lock().unwrap(),
            HashMap::from([(topic.clone(), vec![msgs[0], msgs[1]])])
        );
    }

    #[test]
    fn test_handle_empty_topics() {
        let mut decode_paths = Arc::new(Mutex::new(|_| HashMap::new()));
        let mock_decode_path = Box::new(move |msg| decode_paths.lock().unwrap().insert(BasicBuilder.string(), vec![msg]));

        let result = use_decoded_message_range(vec![], vec![BasicBuilder.string()]);

        assert!(result.is_ok());
        assert_eq!(
            *decode_paths.lock().unwrap(),
            HashMap::from([(BasicBuilder.string(), vec![BasicBuilder.string().unwrap()])])
        );
    }

    #[test]
    fn test_reset_accumulated_data_when_new_range_iterator_is_provided() {
        let topic = BasicBuilder.string();

        let mut decode_paths = Arc::new(Mutex::new(|_| HashMap::new()));
        let mock_decode_path = Box::new(move |msg| decode_paths.lock().unwrap().insert(topic.clone(), vec![msg]));

        let mut subscribe_message_range = MockSubscribeMessageRangeArgs {
            0: topic.clone(),
            1: Arc::new(Mutex::new(mock_decode_path)),
        };

        let result = use_decoded_message_range(vec![topic.clone()], vec![BasicBuilder.string()]);

        assert!(result.is_ok());
        assert_eq!(
            *decode_paths.lock().unwrap(),
            HashMap::from([(topic.clone(), vec![BasicBuilder.string().unwrap()])])
        );

        let first_batch = [MessageEventBuilder.message_event({ topic })];
        let second_batch = [MessageEventBuilder.message_event({ topic })];

        simulate_batches(topic, vec![first_batch]).await.unwrap();

        assert_eq!(
            *decode_paths.lock().unwrap(),
            HashMap::from([(topic.clone(), vec![first_batch[0], first_batch[1]])])
        );

        let unmount = Arc::new(Mutex::new(()));
        use_decoded_message_range(vec![topic.clone()], vec![BasicBuilder.string()]);

        assert!(unmount.lock().unwrap() == ());
    }
}
```