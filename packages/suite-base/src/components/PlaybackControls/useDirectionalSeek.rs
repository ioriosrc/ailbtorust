```rust
use std::rc::Rc;
use crate::components::PlaybackControls::{sharedHelpers::jumpSeek};
use crate::players::types::{NonNullable, Player};
use crate::suite_base::hooks::useAppConfigurationValue;
use crate::suite_base::util::broadcast::BroadcastManager;

type UseDirectionalSeek = {
  seekForwardAction: Rc<dyn Fn(Option<&std::event::KeyboardEvent>)>,
  seekBackwardAction: Rc<dyn Fn(Option<&std::event::KeyboardEvent>)>,
};

type UseDirectionalSeekProps = {
  seek: NonNullable<Player["seekPlayback"]>;
  playUntil: Option<NonNullable<Player["playUntil"]>>;
  getTimeInfo: () -> { startTime: Option<std::time::Time>; endTime: Option<std::time::Time>; currentTime: Option<std::time::Time> };
};

pub fn use_directional_seek(props: UseDirectionalSeekProps) -> UseDirectionalSeek {
    let default_step_size = props.get_app_configuration_value::<i32>(AppSetting::DEFAULT_STEP_SIZE);

    let seek_forward_action = Rc::new(move |ev| {
        if let Some(current_time) = props.get_current_time() {
            // If playUntil is available, we prefer to use that rather than seek, which performs a jump
            // seek.
            //
            // Playing forward up to the desired seek time will play all messages to the panels which
            // mirrors the behavior panels would expect when playing without stepping. This behavior is
            // important for some message types which convey state information.
            //
            // i.e. Skipping coordinate frame messages may result in incorrectly rendered markers or
            // missing markers altogther.

            let target_time = jump_seek(DIRECTION::FORWARD, current_time, ev, default_step_size);
            if let Some(play_until) = props.get_play_until() {
                play_until(target_time);

                BroadcastManager::instance().post_message({
                    type: "playUntil",
                    time: target_time,
                });
            } else {
                props.seek(target_time);

                BroadcastManager::instance().post_message({
                    type: "seek",
                    time: target_time,
                });
            }
        }
    });

    let seek_backward_action = Rc::new(move |ev| {
        if let Some(current_time) = props.get_current_time() {
            let target_time = jump_seek(DIRECTION::BACKWARD, current_time, ev, default_step_size);
            props.seek(target_time);

            BroadcastManager::instance().post_message({
                type: "seek",
                time: target_time,
            });
        }
    });

    UseDirectionalSeek {
        seek_forward_action,
        seek_backward_action,
    }
}
```