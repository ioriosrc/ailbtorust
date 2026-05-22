```rust
use crate::broadcast_manager::BroadcastManager;
use crate::messages::BroadcastMessageEvent;
use crate::time::Time;

type UseBroadcastProps = {
  play: Option<fn()>,
  pause: Option<fn()>,
  seek: Option<(fn(Time) -> (), fn(Time) -> ())>,
  play_until: Option<fn(Time)>,
};

pub fn use_broadcast(props: UseBroadcastProps): () {
    let sync_instances = crate::workspace_store::use_workspace_store(|store| store.playback_controls.sync_instances);

    useEffect(() => {
        BroadcastManager::set_should_sync({ should_sync: sync_instances });
    }, [sync_instances]);

    useEffect(() => {
        let handler = move |message: BroadcastMessageEvent| {
            if message.type == "play_until" {
                props.play_until.unwrap()(message.time);
                return;
            }

            if message.type == "play" {
                props.seek.unwrap()(&message.time, &message.time)();
                props.play.unwrap();
            }

            if message.type == "pause" {
                props.pause.unwrap();
                props.seek.unwrap()(&message.time, &message.time)();
            }

            if message.type == "seek" {
                props.seek.unwrap()(&message.time);
            }
        };

        BroadcastManager::get_instance().add_listener(handler);

        return move || {
            BroadcastManager::get_instance().remove_listener(handler);
        };
    }, [props.play, props.pause, props.seek, props.play_until]);
}
```