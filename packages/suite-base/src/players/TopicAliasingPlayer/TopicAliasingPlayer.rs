```rust
use crate::players::{Player, PlayerState};
use std::sync::Mutex;

// This is a player that wraps an underlying player and applies aliases to all topic names
// in data emitted from the player.
pub struct TopicAliasingPlayer {
    player: Player,
    inputs: Immutable<StateFactoryInput>,
    alias_functions: Vec<TopicAliasFunction>,
    subscriptions: SubscribePayload[],
    listener: Mutex<Option<PlayerStateCallback>>,
}

impl TopicAliasingPlayer {
    pub fn new(player: Player) -> Self {
        TopicAliasingPlayer {
            player,
            inputs: Immutable::new(StateFactoryInput {
                alias_functions: vec![],
                topics: None,
                variables: {},
            }),
            alias_functions: Vec::new(),
            subscriptions: Vec::new(),
            listener: Mutex::new(None),
        }
    }

    pub fn get_metadata(&self) -> ReadonlyArray<Readonly<Metadata>> {
        self.player.get_metadata().unwrap_or_default()
    }

    pub fn get_batch_iterator(
        &mut self,
        topic: &str,
        options: Option<TimeRange>,
    ) -> Box<dyn AsyncIterator<Item = IteratorResult>> {
        self.player.get_batch_iterator(topic, options)
    }

    pub fn set_listener(&self, listener: PlayerStateCallback) {
        *self.listener.lock().unwrap() = Some(listener);
    }

    pub fn set_alias_functions(&mut self, alias_functions: Vec<TopicAliasFunction>) {
        self.alias_functions = alias_functions;
        self.skip_aliases();
    }

    pub fn close(&mut self) {
        self.player.close();
    }

    pub fn set_subscriptions(&mut self, subscriptions: SubscribePayload[]) {
        self.subscriptions = subscriptions;
        self.aliases_updated();
    }

    pub fn set_publishers(&mut self, publishers: Vec<AdvertiseOptions>) {
        self.player.set_publishers(publishers);
    }

    pub fn set_parameter(&mut self, key: &str, value: ParameterValue) {
        self.player.set_parameter(key, value);
    }

    pub fn publish(&mut self, request: PublishPayload) {
        self.player.publish(request);
    }

    pub async fn call_service(&mut self, service: &str, request: impl Into<serde_json::Value>) -> serde_json::Value {
        self.player.call_service(service, request.into())
    }

    pub fn start_playback(&self) {
        if let Some(state) = self.get_last_player_state() {
            self.on_player_state(state);
        }
    }

    pub fn pause_playback(&self) {
        self.player.pause_playback();
    }

    pub fn seek_playback(&mut self, time: Time) {
        self.player.seek_playback(time);
    }

    pub fn play_until(&mut self, time: Time) {
        if let Some(state) = self.get_last_player_state() {
            self.on_player_state(state);
        }
    }

    pub fn set_playback_speed(&mut self, speed_fraction: f64) {
        self.player.set_playback_speed(speed_fraction);
    }

    pub fn set_global_variables(&mut self, global_variables: GlobalVariables) {
        self.inputs = Immutable::new(StateFactoryInput {
            alias_functions: vec![],
            topics: None,
            variables: global_variables,
        });
        self.skip_aliases();
    }

    pub async fn fetch_asset(&self, uri: &str) -> Asset {
        if let Some(state) = self.get_last_player_state() {
            self.on_player_state(state);
        }
        self.player.fetch_asset(uri)
    }

    async fn on_player_state(&mut self, player_state: PlayerState) {
        // If we are already emitting a player state, avoid emitting another one
        if let Some(listener) = &self.listener.lock().unwrap() {
            listener.run(move |listener| {
                if self.skip_aliases() {
                    listener(player_state);
                    Ok(())
                } else {
                    self.aliases_updated();
                    self.on_player_state(player_state)
                }
            })
        } else {
            Err("Listener not set")
        }
    }

    pub fn aliases_updated(&mut self) {
        let state_processor = self.state_processor_factory.build_state_processor(&self.inputs);
        if !state_processor.is_equal(self.current_state_processor()) {
            self.state_processor = state_processor;
            self.reset_subscriptions();
        }
    }

    fn current_state_processor(&self) -> &dyn IStateProcessor {
        &self.state_processor
    }

    pub fn skip_aliases(&mut self) -> bool {
        !self.alias_functions.is_empty() || self.inputs.topics.is_none()
    }

    pub fn reset_subscriptions(&mut self) {
        let aliased_subscriptions = self.state_processor.alias_subscriptions(&self.subscriptions);
        if !aliased_subscriptions.is_equal(self.current_alias_subscriptions()) {
            self.state_processor = state_processor;
            self.reset_aliases();
        }
    }

    fn current_alias_subscriptions(&self) -> SubscribePayload {
        &self.aliases
    }
}
```