```rust
use crate::message_pipeline::{MessagePipelineContext, MessageEvent};
use crate::players::types::{PlayerStateActiveData, SubscribePayload};
use std::collections::HashMap;

pub struct UseMessageReducerProps<T> {
    topics: Vec<String>,
    preload_type: SubscriptionPreloadType,
    restore: Box<dyn Fn(Option<&T>) -> T>,
    add_message: Option<Box<dyn Fn(&mut T, &MessageEvent) -> Option<&mut T>>>,
    add_messages: Option<Box<dyn Fn(&mut T, &[&MessageEvent]) -> Option<&mut T>>>,
}

pub fn use_message_reducer<T>(props: UseMessageReducerProps<T>) -> Vec<MessageEvent> {
    let id = "use-message-reducer".to_string();
    let restore = props.restore;
    let add_message = props.add_message;
    let add_messages = props.add_messages;

    if (add_message.is_some() && add_messages.is_some()) {
        panic!("useMessageReducer must be provided with exactly one of addMessage or addMessages");
    }

    use_should_not_change_frequently(props.restore, || {
        log::warn!(
            "useMessageReducer restore() is changing frequently. " +
                "restore() will be called each time it changes, so a new function " +
                "shouldn't be created on each render. (If you're using Hooks, try useCallback.)",
        );
    });
    use_should_not_change_frequently(props.add_message, || {
        log::warn!(
            "useMessageReducer addMessage() is changing frequently. " +
                "addMessage() will be called each time it changes, so a new function " +
                "shouldn't be created on each render. (If you're using Hooks, try useCallback.)",
        );
    });
    use_should_not_change_frequently(props.add_messages, || {
        log::warn!(
            "useMessageReducer addMessages() is changing frequently. " +
                "addMessages() will be called each time it changes, so a new function " +
                "shouldn't be created on each render. (If you're using Hooks, try useCallback.)",
        );
    });

    let requested_topics = props.topics.into_iter().collect::<Vec<_>>();

    let subscriptions = {
        let mut subscriptions = Vec::new();
        for topic in &requested_topics {
            if let Ok(payload) = serde_json::from_str(topic.as_ref()) {
                if let SubscribePayload { topic: payload_topic, preload_type } = payload {
                    subscriptions.push(SubscribePayload {
                        topic: payload_topic,
                        preload_type,
                    });
                }
            } else {
                subscriptions.push(SubscribePayload { topic, preload_type: "partial" });
            }
        }
        subscriptions
    };

    let set_subscriptions = MessagePipelineContext::set_subscription(&id);
    useEffect(() => {
        set_subscriptions(id.clone(), &subscriptions).unwrap();
    }, [id, set_subscriptions, subscriptions]);
    useEffect(() => {
        return move || {
            set_subscriptions(id.clone(), &Vec::<SubscribePayload>::new()).unwrap();
        };
    }, [id, set_subscriptions]);

    let state = Vec::with_capacity(1024);

    let reducer = Box::new(move |ctx| -> T {
        let message_events = ctx.message_events_by_subscriber_id.get(&id);
        let last_seek_time = ctx.player_state.active_data.map(|data| data.last_seek_time);

        let mut new_reduced_value: T;
        if !state.is_empty() && *state.last().unwrap() != &last_seek_time {
            new_reduced_value = restore(*state.last().unwrap());
        } else if *state.is_empty()
            || restore != state.get(0).map(|item| item.restore())
            || add_message
                .as_ref()
                .is_some()
                && message_events.is_some()
                && !message_events.unwrap_or_default().is_empty()
        {
            new_reduced_value = restore(None);
        } else if *state.is_empty() && add_messages.is_some() {
            for event in message_events.unwrap_or_default() {
                new_reduced_value = add_messages
                    .as_ref()
                    .unwrap()(new_reduced_value, event)
                    .expect("add_message failed");
            }
        }

        state.push(new_reduced_value);

        new_reduced_value
    });

    use_should_not_change_frequently(reducer.as_mut(), || {
        log::warn!(
            "useMessageReducer reducer() is changing frequently. " +
                "reducer() will be called each time it changes, so a new function " +
                "shouldn't be created on each render. (If you're using Hooks, try useCallback.)",
        );
    });

    MessagePipelineContext::run_message_reducer(
        &id,
        reducer.as_ref(),
        message_events.map(|events| events.clone()),
        last_seek_time,
    )
}
```