```rust
use std::sync::{Arc, Condvar, Mutex};
use tokio::{
    sync::{broadcast, broadcast::Receiver, Sender},
    task,
};

use crate::suite::{
    types::{AdvertiseOptions, MessageEvent, SubscribePayload},
    util::is_desktop_app,
};

type FramePromise = Arc<tokio::task::JoinHandle<()>>;
type PublishersById = std::collections::HashMap<String, Vec<AdvertiseOptions>>;

#[derive(Clone)]
struct Player {
    // Implement player methods here
}

#[derive(Default)]
pub struct MessagePipelineContext {
    pub player_state: PlayerState,
    pub message_events_by_subscriber_id: HashMap<String, Vec<MessageEvent>>,
    pub subscriptions: Vec<(String, Vec<SubscribePayload>)>,
    pub sorted_topics: Vec<String>,
    pub sorted_services: Vec<String>,
    pub datatypes: HashMap<String, DataTypeInfo>,
    pub start_playback: Option<fn() -> ()>,
    pub play_until: Option<fn(Option<Time>, Option<Time>) -> ()>,
    pub pause_playback: Option<fn() -> ()>,
    pub set_playback_speed: Option<fn(f64) -> ()>,
    pub seek_playback: Option<fn() -> ()>,
}

#[derive(Default)]
pub struct PlayerState {
    // Implement player state methods here
}

impl Default for MessagePipelineInternalState {
    fn default() -> Self {
        MessagePipelineInternalState {
            dispatch: |action| {
                let mut lock = self.lock().unwrap();
                reducer(&mut *lock, action);
            },
            reset: || {
                let mut lock = self.lock().unwrap();
                reducer(&mut *lock, UpdatePlayerStateAction::default());
            },
            player: None,
            publishers_by_id: PublishersById::new(),
            all_publishers: Vec::new(),
            subscription_memoizer: make_subscription_memoizer(),
            subscriptions_by_id: std::collections::HashMap::new(),
            subscriber_ids_by_topic: std::collections::HashMap::new(),
            public: MessagePipelineContext::default(),
        }
    }
}

fn update_player_state_action(
    prevState: &MessagePipelineInternalState,
    action: UpdatePlayerStateAction,
) -> MessagePipelineInternalState {
    let messages = action.player_state.active_data.as_ref().map(|data| data.messages);

    let mut seen_topics = std::collections::HashSet::new();

    // We need a new set of message arrays for each subscriber since downstream users rely
    // on object instance reference checks to determine if there are new messages
    let messages_by_subscriber_id: std::collections::HashMap<String, Vec<MessageEvent>> =
        std::collections::HashMap::new();

    let subscriber_ids_by_topic = prevState.subscriber_ids_by_topic.clone();

    let last_message_event_by_topic = prevState.last_message_event_by_topic.clone();

    // Put messages into per-subscriber queues
    if messages.is_some() && !messages.eq(&prevState.public.player_state.active_data.as_ref().unwrap().messages) {
        for message_event in messages.unwrap() {
            // Save the last message on every topic to send the last message
            // to newly subscribed panels.
            last_message_event_by_topic.insert(message_event.topic, message_event);

            seen_topics.insert(message_event.topic);
            let ids = subscriber_ids_by_topic.get(message_event.topic).unwrap();
            if !ids.is_empty() {
                for id in ids.iter() {
                    let subscriber_message_events = messages_by_subscriber_id
                        .entry(*id)
                        .or_insert_with(Vec::new);
                    subscriber_message_events.push(message_event);
                }
            }
        }
    }

    let new_public_state = MessagePipelineContext {
        player_state: action.player_state,
        message_events_by_subscriber_id,
    };
    let topics = action.player_state.active_data.as_ref().map(|data| data.topics);

    if topics.is_some() && !topics.eq(&prevState.public.player_state.active_data.as_ref().unwrap().topics) {
        new_public_state.sorted_topics = topics.unwrap()
            .into_iter()
            .sorted_by_key(|topic| topic.name)
            .collect();
    }
    let services = action.player_state.active_data.as_ref().map(|data| data.services);

    if services.is_some() && !services.eq(&prevState.public.player_state.active_data.as_ref().unwrap().services) {
        new_public_state.sorted_services = services
            .keys()
            .into_iter()
            .sorted_by_key(|service| service)
            .collect();
    }
    if (
        action.player_state.active_data.as_ref()
            .map(|data| data.datatypes)
            .is_some()
            && !action.player_state.active_data.as_ref().unwrap().datatypes.eq(&prevState.public.player_state.active_data.as_ref().unwrap().datatypes)
    ) {
        new_public_state.datatypes = action.player_state.active_data.as_ref().unwrap().datatypes.clone();
    }

    let capabilities = action.player_state.capabilities;
    let player = prevState.player.as_ref();

    if player.is_some() && !capabilities.eq(&prevState.last_capabilities) {
        new_public_state.start_playback = Some(move || {
            player.unwrap().start_playback();
        });
        new_public_state.play_until = Some(move |time1, time2| {
            player.unwrap().play_until(time1, time2);
        });
        new_public_state.pause_playback = Some(move || {
            player.unwrap().pause_playback();
        });
        new_public_state.set_playback_speed = Some(move |speed| {
            player.unwrap().set_playback_speed(speed);
        });
        new_public_state.seek_playback = Some(move || {
            player.unwrap().seek_playback();
        });
    }

    MessagePipelineInternalState {
        render_done: action.render_done,
        public: new_public_state,
        last_capabilities: capabilities.clone(),
        last_message_event_by_topic,
    }
}

fn update_subscriber_action(
    prevState: &MessagePipelineInternalState,
    action: UpdateSubscriberAction,
) -> MessagePipelineInternalState {
    let mut lock = prevState.lock().unwrap();
    reducer(&mut *lock, action);
}

#[derive(Default)]
struct UpdatePlayerStateAction {
    player_state: PlayerState,
    render_done: Option<fn()>,
}

#[derive(Default)]
struct UpdateSubscriberAction {
    id: String,
    payloads: Vec<SubscribePayload>,
}

type BroadcastChannel = broadcast::Sender<MessageEvent>;
type ReceiverChannel = broadcast::Receiver<MessageEvent>;

#[tokio::main]
async fn main() {
    let (broadcast_tx, mut broadcast_rx) = broadcast::channel(10);
    let (subscriber_tx, subscriber_rx) = std::sync::mpsc::channel();

    // Create the player and set it up
    let player = Player {
        // Initialize the player here
    };

    let (start_playback_tx, start_playback_rx) = std::sync::mpsc::channel();
    let (play_until_tx, play_until_rx) = std::sync::mpsc::channel();
    let (pause_playback_tx, pause_playback_rx) = std::sync::mpsc::channel();
    let (set_playback_speed_tx, set_playback_speed_rx) = std::sync::mpsc::channel();
    let (seek_playback_tx, seek_playback_rx) = std::sync::mpsc::channel();

    tokio::spawn(async move {
        let mut last_message_event_by_topic: std::collections::HashMap<String, MessageEvent> =
            std::collections::HashMap::new();

        while let Some(event) = broadcast_rx.recv().await {
            if let Some(prev_event) = last_message_event_by_topic.get(&event.topic) {
                if prev_event.uri != event.uri || prev_event.data != event.data {
                    // If the message is different, update the last message
                    last_message_event_by_topic.insert(event.topic, event);
                }
            } else {
                // If this is the first time receiving the message for this topic,
                // save it as the last message.
                last_message_event_by_topic.insert(event.topic, event);
            }

            subscriber_tx.send(event).await.unwrap();
        }
    });

    let (lock, _guard) = Mutex::new(MessagePipelineInternalState {
        dispatch: |action| {
            reducer(&mut *lock, action);
        },
        reset: || {
            let mut lock = lock.lock().unwrap();
            reducer(&mut *lock, UpdatePlayerStateAction::default());
        },
        player: Some(player),
        publishers_by_id: PublishersById::new(),
        all_publishers: Vec::new(),
        subscription_memoizer: make_subscription_memoizer(),
        subscriptions_by_id: std::collections::HashMap::new(),
        subscriber_ids_by_topic: std::collections::HashMap::new(),
        public: MessagePipelineContext {
            player_state: PlayerState::default(),
            message_events_by_subscriber_id: std::collections::HashMap::new(),
            subscriptions: Vec::new(),
            sorted_topics: Vec::new(),
            sorted_services: Vec::new(),
            datatypes: HashMap::new(),
            start_playback: None,
            play_until: None,
            pause_playback: None,
            set_playback_speed: None,
            seek_playback: None,
        },
    });

    let (player_state_tx, player_state_rx) = std::sync::mpsc::channel();

    tokio::spawn(async move {
        loop {
            if let Some(action) = player_state_rx.recv().await {
                lock.lock().unwrap().dispatch(action);
            }
        }
    });

    // Example usage
    let topic = "example_topic".to_string();
    let subscribe_payload = SubscribePayload::new("message".to_string());

    subscriber_tx.send(MessageEvent {
        uri: "http://example.com".to_string(),
        data: b"some data".to_vec(),
        media_type: Some(String::from("text/plain")),
        topic,
    }).await.unwrap();

    // Process the message
    let event = broadcast_rx.recv().await.unwrap();
    println!("Received message: {:?}", event);

    player_state_tx.send(UpdatePlayerStateAction {
        player_state: PlayerState {
            active_data: Some(Box::new(ActiveData {
                messages: vec![MessageEvent {
                    uri: "http://example.com".to_string(),
                    data: b"some data".to_vec(),
                    media_type: Some(String::from("text/plain")),
                    topic,
                }],
                topics: vec![Topic {
                    name: "example_topic".to_string(),
                }],
                services: std::collections::HashMap::new(),
                datatypes: HashMap::new(),
            })),
            render_done: None,
        },
        render_done: Some(|| println!("Render done")),
    }).await.unwrap();
}
```